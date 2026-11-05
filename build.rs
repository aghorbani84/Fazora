use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=native/ffmpeg/fazora_ffmpeg.c");
    println!("cargo:rerun-if-env-changed=FAZORA_FFMPEG_ROOT");
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");

    if env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("android") {
        return;
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH")
        .expect("CARGO_CFG_TARGET_ARCH must be set for Android builds");
    let (abi, compiler_triple) = match target_arch.as_str() {
        "aarch64" => ("arm64-v8a", "aarch64-linux-android"),
        "x86_64" => ("x86_64", "x86_64-linux-android"),
        other => panic!("Unsupported Android architecture for FAZORA FFmpeg: {other}"),
    };

    let ffmpeg_root = PathBuf::from(
        env::var_os("FAZORA_FFMPEG_ROOT")
            .expect("FAZORA_FFMPEG_ROOT must point to the matching LGPL FFmpeg build"),
    );
    let include_dir = ffmpeg_root.join("include");
    let lib_dir = ffmpeg_root.join("lib");
    for required in [
        include_dir.join("libavutil/avutil.h"),
        lib_dir.join("libavutil.so"),
        lib_dir.join("libavcodec.so"),
        lib_dir.join("libavformat.so"),
        lib_dir.join("libswresample.so"),
        lib_dir.join("libswscale.so"),
    ] {
        assert!(
            required.exists(),
            "Missing FFmpeg build input for {abi}: {}",
            required.display()
        );
    }

    let ndk = PathBuf::from(env::var_os("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME must be set"));
    let compiler = ndk
        .join("toolchains/llvm/prebuilt/linux-x86_64/bin")
        .join(format!("{compiler_triple}26-clang"));
    assert!(
        compiler.is_file(),
        "Android NDK compiler not found: {}",
        compiler.display()
    );

    cc::Build::new()
        .file("native/ffmpeg/fazora_ffmpeg.c")
        .include(&include_dir)
        .compiler(compiler)
        .flag_if_supported("-fPIC")
        .warnings(true)
        .compile("fazora_ffmpeg_shim");

    // Declare the archive explicitly as well as through cc's metadata. This
    // keeps the native ABI shim linked when Dioxus drives Cargo via its linker wrapper.
    println!(
        "cargo:rustc-link-search=native={}",
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set")).display()
    );
    println!("cargo:rustc-link-lib=static=fazora_ffmpeg_shim");
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=avutil");
}
