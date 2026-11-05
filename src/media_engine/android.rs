//! Android bridge and FFmpeg native-library smoke test.

use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn fazora_ffmpeg_version() -> *const c_char;
}

#[manganis::ffi("src/android")]
extern "Kotlin" {
    pub type MediaEnginePlugin;

    pub fn verify_libraries(this: &MediaEnginePlugin) -> bool;
}

/// Verify the packaged FFmpeg shared libraries and return the upstream version string.
/// This does not mark playback as available; audio output and video rendering are separate gates.
pub fn ffmpeg_version() -> Result<String, String> {
    let plugin = MediaEnginePlugin::new().map_err(|error| error.to_string())?;
    let loaded = verify_libraries(&plugin).map_err(|error| error.to_string())?;
    if !loaded {
        return Err("One or more FFmpeg shared libraries could not be loaded".to_string());
    }

    let version_ptr = unsafe { fazora_ffmpeg_version() };
    if version_ptr.is_null() {
        return Err("FFmpeg returned a null version pointer".to_string());
    }

    let version = unsafe { CStr::from_ptr(version_ptr) };
    version
        .to_str()
        .map(str::to_owned)
        .map_err(|error| format!("FFmpeg version string is not valid UTF-8: {error}"))
}
