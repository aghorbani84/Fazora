//! Platform-neutral entry points for the native media backend.

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "android")]
pub use android::ffmpeg_version;

#[cfg(not(target_os = "android"))]
pub fn ffmpeg_version() -> Result<String, String> {
    Err("The FFmpeg native adapter is not built for this target yet".to_string())
}
