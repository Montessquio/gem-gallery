//! Code for saving and retrieving files from the disk.

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use dashmap::DashMap;
use eyre::{bail, ensure, Result};
use rand::prelude::*;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
};
use std::{
    io::ErrorKind,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{fs::File, io::AsyncRead, sync::RwLock};
use uuid::Uuid;

/// A pure, safe interface to access files of any kind.
/// This struct guards against path traversal and symlink abuse.
/// It does not check the file contents or names at all though.
#[derive(Debug)]
pub struct FileStore {
    base_path: PathBuf,
    handles: DashMap<String, RwLock<()>>,
}

unsafe impl Send for FileStore {}
unsafe impl Sync for FileStore {}

impl FileStore {
    /// Create a new `FileStore` instance. All reads and writes
    /// will be effectively jailed to the base path.
    pub fn new(base_path: impl AsRef<Path>) -> Result<FileStore> {
        let base_path = base_path.as_ref().to_owned();
        if !base_path.is_dir() {
            bail!("Provided base path is not a directory!");
        }
        Ok(FileStore {
            base_path,
            handles: DashMap::new(),
        })
    }

    /// Write a file's contents into the filesystem.
    /// Returns the serialized filename made with [`gen_file_name`](Self::gen_file_name).
    pub async fn write(
        &mut self,
        normalized_id: &str,
        mut payload: impl AsyncRead + Unpin,
    ) -> Result<String> {
        let (rel_path, fname) = Self::chunk_path(normalized_id);
        let path = self.safe_canonicalize(&rel_path)?;

        ensure!(path.exists(), FSError::NameCollision(fname));

        // Check to see if the file ID is already being accessed
        if self.handles.contains_key(normalized_id) {

        }
        else {
            // Tell the hashmap we have acquired a lock on the file.
        }

        let mut fd = tokio::fs::File::create(path).await?;
        _ = tokio::io::copy(&mut payload, &mut fd).await?;

        Ok(fname)
    }

    /// Retrieves the given file from the filesystem.
    /// Assumes that the input filename is already in
    /// the format returned by `hash_name`.
    pub async fn read(&self, normalized_id: &str) -> Result<impl AsyncRead> {
        let (rel_path, fname) = Self::chunk_path(normalized_id);
        let path = self.safe_canonicalize(&rel_path)?;

        ensure!(path.exists(), FSError::NotFound(fname));

        Ok(tokio::fs::File::open(path).await?)
    }

    /// Safely canonicalizes a given path relative to the base path.
    ///
    /// The `safe_canonicalize` function takes a `Path` as input and attempts to join it
    /// to the base path, eliminating symbolic links and normalizing all intermediate components.
    /// It performs additional safety checks ensuring that the destination file is present,
    /// not a symbolic link, and actually exists within the base_path.
    fn safe_canonicalize(&self, path: &Path) -> Result<PathBuf> {
        let file_name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| format!("Path `{}` had no valid filename!", path.display()));

        let full_path = match self.base_path.join(path).canonicalize() {
            Ok(p) => p,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    return Err(FSError::NotFound(file_name).into());
                }
                _ => return Err(e.into()),
            },
        };

        ensure!(!full_path.is_symlink(), FSError::IsSymlink(file_name));
        ensure!(
            full_path.starts_with(&self.base_path),
            FSError::DirectoryTraversal(file_name)
        );

        Ok(full_path)
    }

    /// Generates a unique file name based on the given input.
    ///
    /// The `gen_file_name` function takes a generic `filename` as input and generates a unique file name based on it.
    /// The generated file name is created by concatenating the little-endian byte representations of
    ///
    /// 1. A random UUID
    /// 2. The current UNIX timestamp
    /// 3. A random 32-bit unsigned integer
    ///
    /// and then Base64 encoding the entire byte array using a url-safe alphabet.
    ///
    fn generate_normal_id() -> String {
        let fid = Uuid::new_v4();

        let now = {
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            since_the_epoch.as_secs().to_le_bytes()
        };

        let num = thread_rng().gen::<u32>().to_le_bytes();

        let bindat = [fid.as_bytes().as_slice(), &now, &num].concat();

        const BASE64: engine::GeneralPurpose =
            engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);
        BASE64.encode(bindat)
    }

    /// The file name is split into the following path:
    /// [First 3 chars]/[Next 3 chars]/[Next 3 chars]/[Entire Filename Unchanged]
    ///
    /// If the file name is "abc123def456.txt", the resulting path would be:
    /// "abc/123/def/abc123def456.txt"
    ///
    /// The `chunk_path` function takes a file name as input and generates the path on-disk the file should be stored at.
    /// The generated path is created by performing the following steps:
    ///
    /// 1. Calls the `gen_file_name` function to generate a unique file name based on the input.
    /// 2. Splits the generated file name into chunks of 3 characters each.
    /// 3. Constructs a path by joining the chunks with forward slashes ("/") in the following format:
    ///    [First 3 chars]/[Next 3 chars]/[Next 3 chars]/[Entire Filename Unchanged]
    ///
    /// For example, if the file name is `abc123def456.txt`, the resulting path would be `abc/123/def/abc123def456.txt`
    ///
    /// This function is intended to be used exclusively in conjunction with [hash_name] and therefore will panic if fname is not long enough.
    fn chunk_path(normalized_id: &str) -> (PathBuf, String) {
        (
            Path::new(&format!(
                "{}/{}/{}/{}",
                &normalized_id[0..=2],
                &normalized_id[3..=5],
                &normalized_id[6..=8],
                &normalized_id
            ))
            .to_owned(),
            normalized_id.to_owned(),
        )
    }
}

/// Errors that can be returned by the FileStore.
#[derive(Debug)]
pub enum FSError {
    /// Indicates the [FileStore::hash_name] method has
    /// failed and produced a non-unique path name.
    NameCollision(String),
    /// Indicates the requested file does not exist on disk.
    NotFound(String),
    /// Indicates the requested file would result in traversal outside
    /// the base path for file storage.
    DirectoryTraversal(String),
    /// Indicates the requested file is a symbolic link.
    IsSymlink(String),
}

impl Display for FSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NameCollision(name) => {
                write!(f, "The file identifier `{}` is already in use", name)
            }
            Self::NotFound(name) => write!(f, "The file identifier `{}` was not found", name),
            Self::DirectoryTraversal(name) => write!(
                f,
                "The file identifier `{}` points to a file located outside the base path",
                name
            ),
            Self::IsSymlink(name) => write!(
                f,
                "The file identifier `{}` points to a symbolic link",
                name
            ),
        }
    }
}

impl Error for FSError {}
