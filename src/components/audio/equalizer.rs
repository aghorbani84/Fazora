//! Equalizer UI — refined technical interface connected to AudioService.

use crate::audio::{AudioService, EqBand, EqPreset};
use crate::components::primitives::{Button, Slider, Toggle};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct EqualizerProps {
    service: AudioService,
}

#[component]
pub fn Equalizer(props: EqualizerProps) -> Element {
    let state = props.service.state.read().clone();
    let service = props.service.clone();
    let enabled = state.enabled;
    let preset = state.preset;
    let preset_label = match preset {
        EqPreset::Flat => "Flat",
        EqPreset::BassBoost => "Bass Boost",
        EqPreset::Vocal => "Vocal",
        EqPreset::Classical => "Classical",
        EqPreset::Electronic => "Electronic",
        EqPreset::Jazz => "Jazz",
        EqPreset::Rock => "Rock",
        EqPreset::Custom => "Custom",
    };

    rsx! {
        div {
            class: "fz-equalizer",
            style: "display: flex; flex-direction: column; gap: var(--space-6);",
            div {
                style: "display: flex; align-items: center; justify-content: space-between; gap: var(--space-4);",
                div {
                    h4 { style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary);", "Equalizer" }
                    p { style: "font-size: var(--text-sm); color: var(--text-secondary);", "DSP processing" }
                }
                div {
                    style: "display: flex; align-items: center; gap: var(--space-2);",
                    span { style: "font-size: var(--text-xs); color: var(--text-muted); font-family: var(--font-mono);", "Enabled" }
                    Toggle {
                        checked: enabled,
                        onchange: {
                            let service = service.clone();
                            move |value| service.set_enabled(value)
                        },
                    }
                }
            }
            div {
                style: "display: flex; gap: var(--space-3); align-items: flex-end; height: 180px;",
                for band in state.bands.clone() {
                    EqBandControl {
                        key: "{band.frequency_hz}",
                        band,
                        enabled,
                        service: service.clone(),
                    }
                }
            }
            div {
                style: "display: flex; align-items: center; gap: var(--space-4);",
                div {
                    style: "flex: 1;",
                    label { style: "display: block; font-size: var(--text-xs); color: var(--text-secondary); margin-bottom: var(--space-1);", "Preamp" }
                    Slider {
                        value: state.preamp_db as f64 / 12.0 + 0.5,
                        min: 0.0,
                        max: 1.0,
                        step: 0.01,
                        onchange: {
                            let service = service.clone();
                            move |value| service.set_preamp(((value - 0.5) * 24.0) as f32)
                        },
                        aria_label: "Preamp gain",
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: var(--space-2);",
                    span { style: "font-size: var(--text-xs); color: var(--text-secondary);", "{preset_label}" }
                    select {
                        style: "background: var(--surface-tertiary); border: 1px solid var(--border); color: var(--text-primary); border-radius: var(--radius-medium); padding: 4px 8px; font-size: var(--text-xs); width: 200px;",
                        for preset_name in [
                            EqPreset::Flat, EqPreset::BassBoost, EqPreset::Vocal,
                            EqPreset::Classical, EqPreset::Electronic, EqPreset::Jazz,
                            EqPreset::Rock, EqPreset::Custom,
                        ] {
                            option {
                                value: format!("{preset_name:?}"),
                                selected: preset_name == state.preset,
                                {match preset_name {
                                    EqPreset::Flat => "Flat",
                                    EqPreset::BassBoost => "Bass Boost",
                                    EqPreset::Vocal => "Vocal",
                                    EqPreset::Classical => "Classical",
                                    EqPreset::Electronic => "Electronic",
                                    EqPreset::Jazz => "Jazz",
                                    EqPreset::Rock => "Rock",
                                    EqPreset::Custom => "Custom",
                                }}
                            }
                        }
                        onchange: {
                            let service = service.clone();
                            move |event: FormEvent| {
                                let selected = match event.value().as_str() {
                                    "BassBoost" => EqPreset::BassBoost,
                                    "Vocal" => EqPreset::Vocal,
                                    "Classical" => EqPreset::Classical,
                                    "Electronic" => EqPreset::Electronic,
                                    "Jazz" => EqPreset::Jazz,
                                    "Rock" => EqPreset::Rock,
                                    "Custom" => EqPreset::Custom,
                                    _ => EqPreset::Flat,
                                };
                                service.set_preset(selected);
                            }
                        }
                    }
                }
            }
            div {
                style: "display: flex; gap: var(--space-2);",
                Button {
                    variant: crate::components::primitives::ButtonVariant::Surface,
                    onclick: {
                        let service = service.clone();
                        let frequencies = state.bands.iter().map(|band| band.frequency_hz).collect::<Vec<_>>();
                        move |_| {
                            service.set_preset(EqPreset::Flat);
                            for frequency in &frequencies {
                                service.set_band_gain(*frequency, 0.0);
                            }
                        }
                    },
                    label: "Reset",
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct EqBandControlProps {
    band: EqBand,
    enabled: bool,
    service: AudioService,
}

#[component]
fn EqBandControl(props: EqBandControlProps) -> Element {
    let gain = props.band.gain_db;
    let frequency = props.band.frequency_hz;
    let service = props.service.clone();
    let on_change = move |value| service.set_band_gain(frequency, value);
    let normalized = ((gain + 12.0) / 24.0).clamp(0.0, 1.0);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; gap: var(--space-2);",
            div {
                style: "width: 26px; height: 140px; background: rgba(244, 241, 233, 0.10); border-radius: var(--radius-medium); position: relative;",
                div {
                    style: format!(
                        "width: 100%; height: {}%; background: {}; border-radius: var(--radius-medium); bottom: 0; position: absolute;",
                        normalized * 100.0,
                        if gain > 0.0 { "var(--fazora-gold)" } else if gain < 0.0 { "var(--fazora-teal)" } else { "var(--text-muted)" }
                    ),
                }
            }
            Slider {
                value: normalized as f64,
                min: 0.0,
                max: 1.0,
                step: 0.01,
                onchange: on_change,
                aria_label: format!("{} Hz band", props.band.frequency_hz),
            }
            div { style: "writing-mode: vertical-rl; text-orientation: mixed; font-size: var(--text-xs); color: var(--text-muted); font-family: var(--font-mono);", "{props.band.frequency_hz}Hz" }
            div { style: "font-size: var(--text-xs); font-family: var(--font-mono); color: var(--text-secondary);", "{gain:+.1} dB" }
        }
    }
}
