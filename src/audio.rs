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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EqBand {
    pub frequency_hz: f32,
    pub gain_db: f32,
    pub q: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Clone, PartialEq)]
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
        let mut state = self.state;
        let mut current = state.write();
        if let Some(band) = current
            .bands
            .iter_mut()
            .find(|b| (b.frequency_hz - frequency_hz).abs() < f32::EPSILON)
        {
            band.gain_db = gain_db.clamp(-12.0, 12.0);
        }
        current.preset = EqPreset::Custom;
    }

    pub fn set_preamp(&self, db: f32) {
        let mut state = self.state;
        state.write().preamp_db = db.clamp(-12.0, 12.0);
    }

    pub fn set_preset(&self, preset: EqPreset) {
        let mut state = self.state;
        let mut current = state.write();
        current.preset = preset;
        if preset == EqPreset::Flat {
            for band in &mut current.bands {
                band.gain_db = 0.0;
            }
        }
    }

    pub fn set_enabled(&self, enabled: bool) {
        let mut state = self.state;
        state.write().enabled = enabled;
    }

    pub fn set_output_device(&self, device_id: &str) {
        let mut state = self.state;
        state.write().output_device = Some(device_id.to_string());
    }

    pub fn save_preset(&self, _name: String) -> Playlist {
        Playlist {
            id: String::new(),
            title: String::new(),
            description: None,
            track_ids: Vec::new(),
            artwork: None,
            created_at: 0,
            modified_at: 0,
            favorite: false,
        }
    }
}
