//! Code for inspecting and optimizing media files.
//!
//! For optimization purposes, all images are stored in webp format
//! and all videos are stored in webm format. (The GIF analog is an animated webp, not a webm.)

use eyre::{bail, Result};
use ffmpeg_next::{ffi::*, format::context::Input, encoder::{Audio, Video}};
use file_format::FileFormat;
use image::codecs::webp::{WebPEncoder, WebPQuality};
use std::{
    ffi::c_void,
    io::{BufRead, Read, Seek, Write},
    ops::{Deref, DerefMut},
    ptr,
    sync::atomic::AtomicUsize,
};

/// Returns a FileFormat is the given data's format is an accepted media type,
/// and an error otherwise.
///
/// If the data format was successfully determined but it
/// was not of an acceptable type, then the returned error will be what format it was
/// determined to be.
///
/// The following formats are supported:
///
/// **Images**
///
/// - JPEG
/// - PNG
/// - GIF
/// - SVG
/// - WEBP
///
/// **Videos**
///
/// - WEBM
/// - MKV
/// - MP4
/// - MOV
///
fn check_format<R: BufRead + Seek>(mut data: &mut R) -> Result<FileFormat> {
    use FileFormat::*;
    let t = FileFormat::from_reader(&mut data)?;
    _ = data.seek(std::io::SeekFrom::Start(0))?;
    match t {
        // Static Images and Animated Images
        JointPhotographicExpertsGroup |
        PortableNetworkGraphics |
        GraphicsInterchangeFormat |
        Webp |
        // Videos
        Webm |
        MatroskaVideo |
        Mpeg4Part14Video |
        AppleQuicktime => Ok(t),
        _ => bail!(t),
    }
}

fn convert_to_webp<R: BufRead + Seek, W: Write>(mut data: &mut R, out: &mut W) -> Result<()> {
    use image::ImageFormat;
    use FileFormat::*;
    let t = check_format(&mut data)?;

    let image = match t {
        // Static Images and Animated Images
        JointPhotographicExpertsGroup => image::load(data, ImageFormat::Jpeg),
        PortableNetworkGraphics => image::load(data, ImageFormat::Png),
        GraphicsInterchangeFormat => image::load(data, ImageFormat::Gif),
        Webp => image::load(data, ImageFormat::WebP),
        _ => bail!(t),
    }?;

    let encoder = WebPEncoder::new_with_quality(out, WebPQuality::lossy(80));
    encoder.encode(
        image.as_bytes(),
        image.width(),
        image.height(),
        image::ColorType::Rgba8,
    )?;

    Ok(())
}

#[allow(unused)]
/// Converts a given WebM, MKV, MP4, or MOV into a WebM using VP9 video codec and Opus audio codec
pub fn convert_to_webm<R: Read>(mut source: std::io::BufReader<R>) -> Result<()> {
    let input = StreamingInput::new(source)?;

    println!(
        "Format: {}, Duration {} us",
        input.format().name(),
        input.duration()
    );

    Ok(())
}

struct StreamingInput<R> {
    istream: *mut std::io::BufReader<R>,

    avio_ptr: *mut AVIOContext,
    format_ptr: *mut AVFormatContext,

    inner: Input,
}

impl<R> Drop for StreamingInput<R> {
    fn drop(&mut self) {
        // the `self.inner` field (type `Input`) comes from `ffmpeg-next`,
        // and isn't part of the `-sys` bindings. Therefore it comes with
        // its own destructor; the library frees the memory properly for us.
        // Therefore we don't need a custom `drop` function since all the
        // resources we allocated will be properly freed, even our custom buffer.
        //
        // This drop implementation will be kept for reference, in case a
        // future change means we will need to transition to dropping things
        // ourselves again.
        /*
        unsafe {
            // Free the input stream by "returning it"
            // to rust's borrow checker system
            _ = Box::from_raw(self.istream);
            println!("Freed IStream");

            // Free the AVIO Context input buffer
            av_free((*self.avio_ptr).buffer as *mut _);
            println!("Freed Context Buffer");

            // Free the AV contexts themselves
            avio_context_free(&mut self.avio_ptr);
            println!("Freed AVIO Context");

            avformat_free_context(self.format_ptr);
            println!("Freed AVFormat Context");
        }
        */
    }
}

impl<R: Read> StreamingInput<R> {
    pub fn new(source: std::io::BufReader<R>) -> Result<Self> {
        const BUF_SIZE: usize = 8192;
        unsafe {
            // In order to stream from memory instead of a file we must
            // construct our own AVIOContext with a custom reader function.
            let bufptr = av_malloc(BUF_SIZE) as *mut u8;

            let datptr = Box::into_raw(Box::new(source));
            // println!("DatPtr: {datptr:?}");
            let avio = avio_alloc_context(
                bufptr,
                BUF_SIZE as i32,
                0,
                datptr as *mut c_void,
                Some(Self::read_function),
                None,
                None,
            );

            // Construct an AVFormatContext and then replace its pb field with out AVIOContext.
            let format = avformat_alloc_context();
            (*format).pb = avio;
            (*format).flags |= AVFMT_FLAG_CUSTOM_IO;

            let rv = avformat_open_input(
                Box::into_raw(Box::new(format)),
                ptr::null(),
                ptr::null(),
                ptr::null_mut(),
            );
            if rv < 0 {
                bail!("FFMPEG Error in avformat_open_input: AVERROR(0x{rv:X})");
            }

            let rv = avformat_find_stream_info(format, ptr::null_mut());
            if rv < 0 {
                bail!("FFMPEG Error in avformat_find_stream_info: AVERROR(0x{rv:X})");
            }

            Ok(StreamingInput {
                istream: datptr,
                avio_ptr: avio,
                format_ptr: format,
                inner: Input::wrap(format),
            })
        }
    }

    unsafe extern "C" fn read_function(opaque: *mut c_void, buf: *mut u8, buf_size: i32) -> i32 {
        // println!("Converting Data (0x{opaque:?})");
        let data = &mut *(opaque as *mut std::io::BufReader<R>);
        // println!("Converting buf (0x{buf:?}, size {buf_size})");
        let buf = std::slice::from_raw_parts_mut(buf, buf_size as usize);

        static BYTES_READ: AtomicUsize = AtomicUsize::new(0);

        // println!("Reading");
        match data.read(buf) {
            Err(e) => todo!("{e}"),
            Ok(0) => {
                //println!(
                //    "Sending AVERROR_EOF (Read {} bytes)",
                //    BYTES_READ.load(std::sync::atomic::Ordering::Acquire)
                //);
                0
            }
            Ok(n) => {
                _ = BYTES_READ.fetch_add(n, std::sync::atomic::Ordering::Release);
                //println!("Read {n} bytes ({} total)", BYTES_READ.load(std::sync::atomic::Ordering::Acquire));
                n as i32
            }
        }
    }
}

impl<R> Deref for StreamingInput<R> {
    type Target = Input;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<R> DerefMut for StreamingInput<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

struct AVTranscoder<R> {
    istream: StreamingInput<R>,
    audio_encoder: Audio,
    video_encoder: Video,
}

/*
#[allow(unused)]
impl<R: Read> AVTranscoder<R> {
    pub fn new(
        istream: std::io::BufReader<R>,
        audio_encoder: Audio,
        video_encoder: Video,
    ) -> Result<Self> {
        let input = StreamingInput::new(istream)?;

        // Transcode audio
        let opus_audio = {
            let input = input
                .streams()
                .best(media::Type::Audio)
                .ok_or_else(|| eyre!("Failed to find best audio stream"))?;

            let mut decoder = ffmpeg_next::codec::Context::from_parameters(input.parameters())?
                .decoder()
                .audio()?;
            decoder.set_parameters(input.parameters());

            let encoder = ffmpeg_next::encoder::find(ffmpeg_next::codec::Id::OPUS)
                .ok_or_else(|| eyre!("Failed to find Opus encoder"))?
                .audio()?;
            encoder.set_parameters(input.parameters());

            let channel_layout = encoder
                .channel_layouts()
                .map(|cls| cls.best(decoder.channel_layout().channels()))
                .unwrap_or(ffmpeg::channel_layout::ChannelLayout::STEREO);

            encoder.set_rate(decoder.rate() as i32);

            0
        };

        Ok(Self {
            istream: input,
            audio_encoder,
            video_encoder,
        })
    }
}

impl<R: Read> Read for AVTranscoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}
*/