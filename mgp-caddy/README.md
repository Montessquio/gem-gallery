# MGP-Caddy

## Roadmap

- [ ] Docker Image
- [ ] HTTP REST Interface
- [X] Image Transcoding
- [ ] Video Transcoding (Assignee: Monty)
- [X] Name Normalization
- [X] Filesystem I/O

## Building

`cargo build` will build the binary executable with all its dependencies statically linked.

### Dependencies

This project requires LLVM and FFMPEG to build - see [ffmpeg-next](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building) for more info.

Our version of FFMPEG is compiled with certain configuration options - you'll need the following static libraries in `dep/lib/{TARGET-TRIPLE}/lib` in order to compile the project.

- libdav1d, for decoding AV1 video [(VideoLan)](https://code.videolan.org/videolan/dav1d)
- MP3LAME, for MP3 [(SourceForge)](https://lame.sourceforge.io/)
- Opus, [opus-codec.org](https://opus-codec.org/)
- LibVPX, for VP9 video [thewebmproject.org](https://www.webmproject.org/) and all its dependencies

Libraries for `x86_64-unknown-linux-gnu` have been precompiled, you shouldn't need to build any C projects to develop on 64 bit linux.

### FFMPEG

The FFMPEG `av*` suite of libraries are automatically build by the ffmpeg crate, which it does through ffmpeg-sys for bindings. Local copies of both crates are
kept in `dep/crate`. They've had a few patches applied to them in order to force them to build with the required additional codecs.
