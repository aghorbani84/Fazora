//! Android Media3 adapter.
//!
//! The Kotlin side owns ExoPlayer and marshals all player calls to Android's
//! main thread. This Rust wrapper keeps the JNI boundary narrow and JSON-based.

use serde::Deserialize;

#[manganis::ffi("src/android")]
extern "Kotlin" {
    pub type FazoraMediaPlugin;

    pub fn initializeJson(this: &FazoraMediaPlugin) -> String;
    pub fn openAudioPickerJson(this: &FazoraMediaPlugin) -> String;
    pub fn takePickedUriJson(this: &FazoraMediaPlugin) -> String;
    pub fn setMediaUriJson(this: &FazoraMediaPlugin, uri: String) -> String;
    pub fn playJson(this: &FazoraMediaPlugin) -> String;
    pub fn pauseJson(this: &FazoraMediaPlugin) -> String;
    pub fn stopJson(this: &FazoraMediaPlugin) -> String;
    pub fn seekToJson(this: &FazoraMediaPlugin, position_ms: String) -> String;
    pub fn setVolumeJson(this: &FazoraMediaPlugin, volume: String) -> String;
    pub fn getStateJson(this: &FazoraMediaPlugin) -> String;
    pub fn releaseJson(this: &FazoraMediaPlugin) -> String;
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EngineSnapshot {
    pub engine_available: bool,
    pub is_playing: bool,
    pub playback_state: i32,
    pub position_ms: i64,
    pub duration_ms: i64,
    pub volume: f32,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct CommandResponse {
    ok: bool,
    message: Option<String>,
}

pub struct AndroidMediaEngine {
    plugin: FazoraMediaPlugin,
}

impl AndroidMediaEngine {
    pub fn new() -> Result<Self, String> {
        let plugin = FazoraMediaPlugin::new().map_err(|error| error.to_string())?;
        let engine = Self { plugin };
        engine.command(initializeJson(&engine.plugin))?;
        Ok(engine)
    }

    fn command(&self, response: Result<String, impl std::fmt::Display>) -> Result<(), String> {
        let json = response.map_err(|error| error.to_string())?;
        let parsed: CommandResponse =
            serde_json::from_str(&json).map_err(|error| format!("Invalid Media3 response: {error}"))?;
        if parsed.ok {
            Ok(())
        } else {
            Err(parsed.message.unwrap_or_else(|| "Media3 command failed".to_string()))
        }
    }

    pub fn open_audio_picker(&self) -> Result<(), String> {
        self.command(openAudioPickerJson(&self.plugin))
    }

    pub fn take_picked_uri(&self) -> Result<Option<String>, String> {
        let json = takePickedUriJson(&self.plugin).map_err(|error| error.to_string())?;
        #[derive(Deserialize)]
        struct PickerResult {
            uri: Option<String>,
        }
        let result: PickerResult =
            serde_json::from_str(&json).map_err(|error| format!("Invalid picker response: {error}"))?;
        Ok(result.uri)
    }

    pub fn set_media_uri(&self, uri: &str) -> Result<(), String> {
        self.command(setMediaUriJson(&self.plugin, uri.to_string()))
    }

    pub fn play(&self) -> Result<(), String> {
        self.command(playJson(&self.plugin))
    }

    pub fn pause(&self) -> Result<(), String> {
        self.command(pauseJson(&self.plugin))
    }

    pub fn stop(&self) -> Result<(), String> {
        self.command(stopJson(&self.plugin))
    }

    pub fn seek_to_ms(&self, position_ms: u64) -> Result<(), String> {
        self.command(seekToJson(&self.plugin, position_ms.to_string()))
    }

    pub fn set_volume(&self, volume: f32) -> Result<(), String> {
        if !volume.is_finite() {
            return Err("Volume must be finite".to_string());
        }
        self.command(setVolumeJson(&self.plugin, volume.clamp(0.0, 1.0).to_string()))
    }

    pub fn snapshot(&self) -> Result<EngineSnapshot, String> {
        let json = getStateJson(&self.plugin).map_err(|error| error.to_string())?;
        serde_json::from_str(&json).map_err(|error| format!("Invalid Media3 state: {error}"))
    }

    pub fn release(&self) -> Result<(), String> {
        self.command(releaseJson(&self.plugin))
    }
}
