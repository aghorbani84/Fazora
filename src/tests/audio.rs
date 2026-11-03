//! Audio service tests.
//!
//! `AudioService::init()` creates `Signal`s via `use_signal()`, which must
//! run inside a component scope, so each test mounts the service through the
//! `VirtualDom` harness defined in `mod.rs`.

use crate::audio::{AudioDevice, AudioService, DeviceKind, EqBand, EqPreset};
use dioxus::prelude::*;

service_harness!(AudioHarness, AUDIO_SLOT, AudioService, AudioService::init);

#[test]
fn audio_initial_state() {
    let audio = with_service();

    assert_eq!(audio.state.read().preset, EqPreset::Flat);
    assert_eq!(audio.state.read().preamp_db, 0.0);
    assert!(!audio.state.read().enabled);
    assert!(audio.state.read().output_device.is_none());
    assert!(audio.state.read().devices.is_empty());
    assert_eq!(audio.state.read().crossfade_sec, 0.0);
    assert!(audio.state.read().gapless);

    // 10 default bands
    assert_eq!(audio.state.read().bands.len(), 10);
    assert_eq!(audio.state.read().bands[0].frequency_hz, 32.0);
    assert_eq!(audio.state.read().bands[0].gain_db, 0.0);
    assert_eq!(audio.state.read().bands[9].frequency_hz, 16000.0);
    assert_eq!(audio.state.read().bands[9].gain_db, 0.0);
}

#[test]
fn audio_set_band_gain() {
    let audio = with_service();

    audio.set_band_gain(60.0, 5.0);

    // Preset changes to Custom when a band is modified
    assert_eq!(audio.state.read().preset, EqPreset::Custom);

    let state = audio.state.read();
    let band = state
        .bands
        .iter()
        .find(|b| b.frequency_hz == 64.0) // closest to 60.0
        .unwrap();
    assert_eq!(band.gain_db, 5.0);
}

#[test]
fn audio_set_band_gain_clamps() {
    let audio = with_service();

    audio.set_band_gain(1000.0, 20.0); // above max
    let state = audio.state.read();
    let band = state
        .bands
        .iter()
        .find(|b| b.frequency_hz == 1000.0)
        .unwrap();
    assert_eq!(band.gain_db, 12.0); // clamped to max

    audio.set_band_gain(1000.0, -20.0); // below min
    let state = audio.state.read();
    let band = state
        .bands
        .iter()
        .find(|b| b.frequency_hz == 1000.0)
        .unwrap();
    assert_eq!(band.gain_db, -12.0); // clamped to min
}

#[test]
fn audio_set_band_gain_unknown_frequency_noop() {
    let audio = with_service();

    audio.set_band_gain(99999.0, 5.0); // no band at this frequency

    // All bands should still have 0.0 gain
    for band in &audio.state.read().bands {
        assert_eq!(band.gain_db, 0.0);
    }
}

#[test]
fn audio_set_preamp_clamps() {
    let audio = with_service();

    audio.set_preamp(20.0);
    assert_eq!(audio.state.read().preamp_db, 12.0); // clamped

    audio.set_preamp(-20.0);
    assert_eq!(audio.state.read().preamp_db, -12.0); // clamped

    audio.set_preamp(5.0);
    assert_eq!(audio.state.read().preamp_db, 5.0);
}

#[test]
fn audio_set_preset_flat_resets_bands() {
    let audio = with_service();

    // First modify a band
    audio.set_band_gain(1000.0, 5.0);
    assert_eq!(audio.state.read().preset, EqPreset::Custom);

    // Then set Flat preset
    audio.set_preset(EqPreset::Flat);
    assert_eq!(audio.state.read().preset, EqPreset::Flat);

    // All bands should be reset to 0.0
    for band in &audio.state.read().bands {
        assert_eq!(band.gain_db, 0.0);
    }
}

#[test]
fn audio_set_enabled() {
    let audio = with_service();

    assert!(!audio.state.read().enabled);
    audio.set_enabled(true);
    assert!(audio.state.read().enabled);
    audio.set_enabled(false);
    assert!(!audio.state.read().enabled);
}

#[test]
fn audio_set_output_device() {
    let audio = with_service();

    assert!(audio.state.read().output_device.is_none());
    audio.set_output_device("dev1");
    assert_eq!(audio.state.read().output_device, Some("dev1".to_string()));
}

#[test]
fn audio_preset_variants() {
    assert_eq!(EqPreset::default(), EqPreset::Flat);
    assert_eq!(EqPreset::Flat as u32, 0);
    assert_eq!(EqPreset::BassBoost as u32, 1);
    assert_eq!(EqPreset::Vocal as u32, 2);
    assert_eq!(EqPreset::Classical as u32, 3);
    assert_eq!(EqPreset::Electronic as u32, 4);
    assert_eq!(EqPreset::Jazz as u32, 5);
    assert_eq!(EqPreset::Rock as u32, 6);
    assert_eq!(EqPreset::Custom as u32, 7);
}

#[test]
fn audio_device_creation() {
    let device = AudioDevice {
        id: "dev1".to_string(),
        name: "Default Output".to_string(),
        is_default: true,
        kind: DeviceKind::Output,
    };

    assert_eq!(device.id, "dev1");
    assert_eq!(device.name, "Default Output");
    assert!(device.is_default);
    assert_eq!(device.kind, DeviceKind::Output);
}

#[test]
fn device_kind_variants() {
    assert_eq!(DeviceKind::Output as u32, 0);
    assert_eq!(DeviceKind::Input as u32, 1);
}

#[test]
fn eq_band_creation() {
    let band = EqBand {
        frequency_hz: 1000.0,
        gain_db: 3.5,
        q: 1.0,
    };

    assert_eq!(band.frequency_hz, 1000.0);
    assert_eq!(band.gain_db, 3.5);
    assert_eq!(band.q, 1.0);
}
