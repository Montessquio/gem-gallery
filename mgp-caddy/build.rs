fn main() {
    println!(
        "cargo:rustc-link-search=all={}/dep/lib/{}/lib", 
        std::env::var("CARGO_MANIFEST_DIR").unwrap(),
        current_platform::CURRENT_PLATFORM,
    );
    
    println!("cargo:rustc-link-lib=static:+bundle=dav1d");
    println!("cargo:rustc-link-lib=static:+bundle=ogg");
    println!("cargo:rustc-link-lib=static:+bundle=vorbis");
    println!("cargo:rustc-link-lib=static:+bundle=vorbisenc");
    println!("cargo:rustc-link-lib=static:+bundle=vorbisfile");
    println!("cargo:rustc-link-lib=static:+bundle=vpx");
    println!("cargo:rustc-link-lib=static:+bundle=opus");
    println!("cargo:rustc-link-lib=static:+bundle=mp3lame");
    println!("cargo:rustc-link-lib=static:+bundle=avcodec");
    println!("cargo:rustc-link-lib=static:+bundle=avdevice");
    println!("cargo:rustc-link-lib=static:+bundle=avfilter");
    println!("cargo:rustc-link-lib=static:+bundle=avformat");
    println!("cargo:rustc-link-lib=static:+bundle=avutil");
}