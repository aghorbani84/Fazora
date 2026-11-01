//! Audio service — equalizer, output devices and DSP configuration.

use crate::models::Playlist;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EqPreset {
    #[default]
    Flat,
    BassBoost,
    Vocal,
    Classical,
    Electronic,
    Jazz,
    Rock,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EqBand {
    pub frequency_hz: f32,
    pub gain_db: f32,
    pub q: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioState {
    pub preset: EqPreset,
    pub preamp_db: f32,
    pub bands: Vec<EqBand>,
    pub enabled: bool,
    pub output_device: Option<String>,
    pub devices: Vec<AudioDevice>,
    pub crossfade_sec: f32,
    pub gapless: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub kind: DeviceKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeviceKind {
    Output,
    Input,
}

const DEFAULT_BANDS: [(f32, f32, f32); 10] = [
    (32.0, 0.0, 1.0),
    (64.0, 0.0, 1.0),
    (125.0, 0.0, 1.0),
    (250.0, 0.0, 1.0),
    (500.0, 0.0, 1.0),
    (1000.0, 0.0, 1.0),
    (2000.0, 0.0, 1.0),
    (4000.0, 0.0, 1.0),
    (8000.0, 0.0, 1.0),
    (16000.0, 0.0, 1.0),
];

#[derive(Clone)]
pub struct AudioService {
    pub state: Signal<AudioState>,
}

impl AudioService {
    pub fn init() -> Self {
        Self {
            state: use_signal(|| AudioState {
                preset: EqPreset::Flat,
                preamp_db: 0.0,
                bands: DEFAULT_BANDS
                    .iter()
                    .map(|(f, g, q)| EqBand {
                        frequency_hz: *f,
                        gain_db: *g,
                        q: *q,
                    })
                    .collect(),
                enabled: false,
                output_device: None,
                devices: Vec::new(),
                crossfade_sec: 0.0,
                gapless: true,
            }),
        }
    }

    pub fn set_band_gain(&self, frequency_hz: f32, gain_db: f32) {
        self.state.write().with(|s| {
            if let Some(band) = s
                .bands
                .iter_mut()
                .find(|b| (b.frequency_hz - frequency_hz).abs() < f32::EPSILON)
            {
                band.gain_db = gain_db.clamp(-12.0, 12.0);
            }
            s.preset = EqPreset::Custom;
        });
    }

    pub fn set_preamp(&self, db: f32) {
        self.state.write().with(|s| s.preamp_db = db.clamp(-12.0, 12.0));
    }

    pub fn set_preset(&self, preset: EqPreset) {
        self.state.write().with(|s| {
            s.preset = preset;
            if preset == EqPreset::Flat {
                for band in &mut s.bands {
                    band.gain_db = 0.0;
                }
            }
        });
    }

    pub fn set_enabled(&self, enabled: bool) {
        self.state.write().with(|s| s.enabled = enabled);
    }

    pub fn set_output_device(&self, device_id: &str) {
        self.state.write().with(|s| {
            s.output_device = Some(device_id.to_string());
        });
    }

    pub fn save_preset(&self, name: String) -> Playlist {
        // Placeholder: EQ presets are stored as user presets in settings.
        let _ = name;
        todo!("EQ preset persistence is wired through SettingsService")
    }
}
