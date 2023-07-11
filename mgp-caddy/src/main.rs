//! A media caddy module that serves static files from a filesystem over HTTP.

#![deny(
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    unused_variables
)]
#![allow(dead_code)]
#![allow(unused)]

use std::path::Path;

use warp::hyper::StatusCode;
use warp::multipart::{FormData, Part};
use warp::{Filter, Rejection, Reply};

pub mod convert;
pub mod fs;

static SAMPLE_FILE: &[u8] = include_bytes!(r#"sample.mp4"#);

const MAX_UPLOAD_SIZE: u64 = 5_000_000; // 5mb;
const FILE_STORE_PATH: &str = "./fileStore";
static FILE_STORE: fs::FileStore = fs::FileStore::new(FILE_STORE_PATH).unwrap();

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let input = std::io::BufReader::new(SAMPLE_FILE);
    println!("{:?}", convert::convert_to_webm(input));

    serve().await?;

    Ok(())
}

async fn serve() -> eyre::Result<()> {
    let getmeta = warp::path("meta")
        .and(warp::path::param::<String>())
        .and(warp::get())
        .map(|name| format!("meta, {}!", name));

    let getfile = warp::path("file")
        .and(warp::path::param::<String>())
        .and(warp::get())
        .map(|name| format!("getfile, {}!", name));

    let putfile = warp::path("file")
        .and(warp::multipart::form().max_length(MAX_UPLOAD_SIZE))
        .and(warp::post())
        .map(|name| format!("putfile, {:?}!", name));

    let delfile = warp::path("file")
        .and(warp::path::param::<String>())
        .and(warp::delete())
        .map(|name| format!("delfile, {}!", name));

    let invalidendpoint = warp::any().map(|| {
        Ok(warp::reply::with_status(
            "METHOD_NOT_ALLOWED",
            StatusCode::METHOD_NOT_ALLOWED,
        ))
    });

    let routes = getmeta
        .or(getfile)
        .or(putfile)
        .or(delfile)
        .or(invalidendpoint);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}

async fn getmeta(file_id: String) -> Result<impl Reply, Rejection> {
    let file_id = Path::new(&file_id).to_owned();
    if file_id.components().count() > 1 {
        return Ok(warp::reply::with_status("INVALID_FILE_ID", StatusCode::BAD_REQUEST));
    }
    
    let file_id = match file_id.file_stem() {
        None => return Ok(warp::reply::with_status("INVALID_FILE_ID", StatusCode::BAD_REQUEST)),
        Some(file_id) => file_id.to_string_lossy(),
    };

    // At this point there should be no path traversal elements in file_id

    Ok("success")
}
