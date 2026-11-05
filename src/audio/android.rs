//! Android Media3 adapter.
//!
//! The Kotlin player is invoked through a single JSON JNI method. Class lookup
//! uses the Activity's application ClassLoader rather than JNI FindClass,
//! which can resolve against the wrong loader in Android application processes.

use serde::Deserialize;
use serde_json::{json, Value};
use std::cell::RefCell;

#[manganis::ffi("src/android")]
extern "Kotlin" {
    // Keep this declaration so dx collects and bundles the Gradle plugin.
    // Runtime construction below deliberately avoids the generated FindClass
    // constructor and uses Activity.getClassLoader().loadClass() instead.
    pub type FazoraMediaPlugin;
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EngineSnapshot {
    pub engine_available: bool,
    pub is_playing: bool,
    pub play_when_ready: bool,
    pub playback_state: i32,
    pub position_ms: i64,
    pub duration_ms: i64,
    pub volume: f32,
    pub error: Option<String>,
}

struct NativeMediaPlugin {
    inner: manganis::jni::objects::GlobalRef,
}

impl NativeMediaPlugin {
    fn new() -> Result<Self, String> {
        let result = manganis::android::with_activity(|mut env, activity| {
            let class_loader = match env
                .call_method(activity, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])
                .and_then(|value| value.l())
            {
                Ok(value) => value,
                Err(error) => {
                    return Some(Err(format!("Unable to get Android class loader: {error:?}")))
                }
            };

            let class_name = match env.new_string("com.fazora.media.FazoraMediaPlugin") {
                Ok(value) => value,
                Err(error) => {
                    return Some(Err(format!("Unable to allocate plugin class name: {error:?}")))
                }
            };

            let class_object = match env
                .call_method(
                    &class_loader,
                    "loadClass",
                    "(Ljava/lang/String;)Ljava/lang/Class;",
                    &[manganis::jni::objects::JValue::Object(&class_name)],
                )
                .and_then(|value| value.l())
            {
                Ok(value) => value,
                Err(error) => {
                    return Some(Err(format!(
                        "Application ClassLoader could not load FazoraMediaPlugin: {error:?}"
                    )))
                }
            };
            let class: manganis::jni::objects::JClass = class_object.into();

            let instance = match env.new_object(
                &class,
                "(Landroid/app/Activity;)V",
                &[manganis::jni::objects::JValue::Object(activity)],
            ) {
                Ok(value) => value,
                Err(error) => {
                    return Some(Err(format!("Unable to construct Media3 plugin: {error:?}")))
                }
            };

            match env.new_global_ref(&instance) {
                Ok(global) => Some(Ok(Self { inner: global })),
                Err(error) => Some(Err(format!("Unable to retain Media3 plugin: {error:?}"))),
            }
        });

        match result {
            Some(result) => result,
            None => Err("Android Activity/JNI environment is unavailable".to_string()),
        }
    }

    fn invoke_json(&self, request: &str) -> Result<String, String> {
        let result = manganis::android::with_activity(|mut env, _activity| {
            let request_string = match env.new_string(request) {
                Ok(value) => value,
                Err(error) => {
                    return Some(Err(format!("Unable to allocate JNI request: {error:?}")))
                }
            };

            let response = match env.call_method(
                self.inner.as_obj(),
                "invokeJson",
                "(Ljava/lang/String;)Ljava/lang/String;",
                &[manganis::jni::objects::JValue::Object(&request_string)],
            ) {
                Ok(value) => value,
                Err(error) => return Some(Err(format!("Native Media3 call failed: {error:?}"))),
            };

            let response_object = match response.l() {
                Ok(value) if !value.is_null() => value,
                Ok(_) => return Some(Err("Native Media3 returned a null response".to_string())),
                Err(error) => {
                    return Some(Err(format!("Invalid native Media3 response: {error:?}")))
                }
            };
            let response_string: manganis::jni::objects::JString = response_object.into();
            match env.get_string(&response_string) {
                Ok(value) => {
                    let value: String = value.into();
                    Some(Ok(value))
                }
                Err(error) => Some(Err(format!("Unable to read native response: {error:?}"))),
            }
        });

        match result {
            Some(result) => result,
            None => Err("Android Activity/JNI environment is unavailable".to_string()),
        }
    }
}

pub struct AndroidMediaEngine {
    plugin: NativeMediaPlugin,
}

impl AndroidMediaEngine {
    pub fn new() -> Result<Self, String> {
        let plugin = NativeMediaPlugin::new()?;
        let engine = Self { plugin };
        engine.invoke("initialize", None)?;
        Ok(engine)
    }

    fn invoke(&self, method: &str, value: Option<&str>) -> Result<Value, String> {
        let request = match value {
            Some(value) => json!({ "method": method, "value": value }),
            None => json!({ "method": method }),
        };
        let response = self.plugin.invoke_json(&request.to_string())?;
        let parsed: Value = serde_json::from_str(&response)
            .map_err(|error| format!("Invalid Media3 response: {error}"))?;

        if parsed.get("ok").and_then(Value::as_bool) == Some(false) {
            return Err(parsed
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("Media3 command failed")
                .to_string());
        }
        Ok(parsed)
    }

    pub fn open_audio_picker(&self) -> Result<(), String> {
        self.invoke("openAudioPicker", None).map(|_| ())
    }

    pub fn take_picked_uri(&self) -> Result<Option<String>, String> {
        let response = self.invoke("takePickedUri", None)?;
        Ok(response
            .get("uri")
            .and_then(Value::as_str)
            .map(ToOwned::to_owned))
    }

    pub fn set_media_uri(&self, uri: &str) -> Result<(), String> {
        self.invoke("setMediaUri", Some(uri)).map(|_| ())
    }

    pub fn play(&self) -> Result<(), String> {
        self.invoke("play", None).map(|_| ())
    }

    pub fn pause(&self) -> Result<(), String> {
        self.invoke("pause", None).map(|_| ())
    }

    pub fn stop(&self) -> Result<(), String> {
        self.invoke("stop", None).map(|_| ())
    }

    pub fn seek_to_ms(&self, position_ms: u64) -> Result<(), String> {
        self.invoke("seekTo", Some(&position_ms.to_string())).map(|_| ())
    }

    pub fn set_volume(&self, volume: f32) -> Result<(), String> {
        if !volume.is_finite() {
            return Err("Volume must be finite".to_string());
        }
        self.invoke("setVolume", Some(&volume.clamp(0.0, 1.0).to_string()))
            .map(|_| ())
    }

    pub fn snapshot(&self) -> Result<EngineSnapshot, String> {
        let response = self.invoke("getState", None)?;
        serde_json::from_value(response)
            .map_err(|error| format!("Invalid Media3 state: {error}"))
    }
}

thread_local! {
    static ENGINE: RefCell<Option<AndroidMediaEngine>> = const { RefCell::new(None) };
}

/// Lazily create the native player after an explicit user gesture.
pub fn initialize_global() -> Result<(), String> {
    ENGINE.with(|slot| {
        if slot.borrow().is_none() {
            let engine = AndroidMediaEngine::new()?;
            *slot.borrow_mut() = Some(engine);
        }
        Ok(())
    })
}

fn with_engine<T>(
    operation: impl FnOnce(&AndroidMediaEngine) -> Result<T, String>,
) -> Result<T, String> {
    ENGINE.with(|slot| {
        let borrowed = slot.borrow();
        let engine = borrowed
            .as_ref()
            .ok_or_else(|| "Android audio engine has not been initialized".to_string())?;
        operation(engine)
    })
}

pub fn open_audio_picker() -> Result<(), String> {
    with_engine(AndroidMediaEngine::open_audio_picker)
}

pub fn take_picked_uri() -> Result<Option<String>, String> {
    with_engine(AndroidMediaEngine::take_picked_uri)
}

pub fn set_media_uri(uri: &str) -> Result<(), String> {
    with_engine(|engine| engine.set_media_uri(uri))
}

pub fn play() -> Result<(), String> {
    with_engine(AndroidMediaEngine::play)
}

pub fn pause() -> Result<(), String> {
    with_engine(AndroidMediaEngine::pause)
}

pub fn stop() -> Result<(), String> {
    with_engine(AndroidMediaEngine::stop)
}

pub fn seek_to_ms(position_ms: u64) -> Result<(), String> {
    with_engine(|engine| engine.seek_to_ms(position_ms))
}

pub fn set_volume(volume: f32) -> Result<(), String> {
    with_engine(|engine| engine.set_volume(volume))
}

pub fn snapshot() -> Result<EngineSnapshot, String> {
    with_engine(AndroidMediaEngine::snapshot)
}
