//! Equalizer UI — refined technical interface connected to AudioService.

use crate::audio::{AudioService, EqBand, EqPreset};
use crate::components::primitives::{Button, Slider, Toggle};
use dioxus::prelude::*;

#[derive(Props)]
pub struct EqualizerProps {
    service: AudioService,
}

#[component]
pub fn Equalizer(props: EqualizerProps) -> Element {
    let state = props.service.state.read();
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
                    h4 {
                        style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary);",
                        "Equalizer"
                    }
                    p {
                        style: "font-size: var(--text-sm); color: var(--text-secondary);",
                        "DSP processing"
                    }
                }
                div {
                    style: "display: flex; align-items: center; gap: var(--space-2);",
                    span {
                        style: "font-size: var(--text-xs); color: var(--text-muted); font-family: var(--font-mono);",
                        "Enabled"
                    }
                    Toggle {
                        checked: enabled,
                        onchange: move |v| props.service.set_enabled(v),
                    }
                }
            }
            div {
                style: "display: flex; gap: var(--space-3); align-items: flex-end; height: 180px;",
                for band in &state.bands {
                    EqBandControl {
                        key: "{band.frequency_hz}",
                        band: band.clone(),
                        enabled,
                        on_change: move |gain| props.service.set_band_gain(band.frequency_hz, gain),
                    }
                }
            }
            div {
                style: "display: flex; align-items: center; gap: var(--space-4);",
                div {
                    style: "flex: 1;",
                    label {
                        style: "display: block; font-size: var(--text-xs); color: var(--text-secondary); margin-bottom: var(--space-1);",
                        "Preamp"
                    }
                    Slider {
                        value: state.preamp_db as f64 / 12.0 + 0.5,
                        min: 0.0,
                        max: 1.0,
                        step: 0.01,
                        onchange: move |v| props.service.set_preamp(((v - 0.5) * 24.0 - 12.0) as f32),
                        aria_label: "Preamp gain",
                    }
                }
                div {
                    style: "display: flex; flex-direction: column; gap: var(--space-2);",
                    span {
                        style: "font-size: var(--text-xs); color: var(--text-secondary);",
                        "{preset_label}"
                    }
                    select {
                        style: "background: var(--surface-tertiary); border: 1px solid var(--border); color: var(--text-primary); border-radius: var(--radius-medium); padding: 4px 8px; font-size: var(--text-xs); width: 200px;",
                        for preset_name in [
                            EqPreset::Flat,
                            EqPreset::BassBoost,
                            EqPreset::Vocal,
                            EqPreset::Classical,
                            EqPreset::Electronic,
                            EqPreset::Jazz,
                            EqPreset::Rock,
                            EqPreset::Custom,
                        ] {
                            option {
                                value: "{format!("{:?}", preset_name)}",
                                selected: preset_name == state.preset,
                                {match preset_name {
                                    EqPreset::Flat => "Flat".to_string(),
                                    EqPreset::BassBoost => "Bass Boost".to_string(),
                                    EqPreset::Vocal => "Vocal".to_string(),
                                    EqPreset::Classical => "Classical".to_string(),
                                    EqPreset::Electronic => "Electronic".to_string(),
                                    EqPreset::Jazz => "Jazz".to_string(),
                                    EqPreset::Rock => "Rock".to_string(),
                                    EqPreset::Custom => "Custom".to_string(),
                                }}
                            }
                        }
                        onchange: move |evt: FormEvent| {
                            let selected = match evt.value().as_str() {
                                "BassBoost" => EqPreset::BassBoost,
                                "Vocal" => EqPreset::Vocal,
                                "Classical" => EqPreset::Classical,
                                "Electronic" => EqPreset::Electronic,
                                "Jazz" => EqPreset::Jazz,
                                "Rock" => EqPreset::Rock,
                                "Custom" => EqPreset::Custom,
                                _ => EqPreset::Flat,
                            };
                            props.service.set_preset(selected);
                        }
                    }
                }
            }
            div {
                style: "display: flex; gap: var(--space-2);",
                Button {
                    variant: crate::components::primitives::ButtonVariant::Surface,
                    onclick: move |_| {
                        props.service.set_preset(EqPreset::Flat);
                        for b in &state.bands {
                            props.service.set_band_gain(b.frequency_hz, 0.0);
                        }
                    },
                    label: "Reset",
                }
            }
        }
    }
}

#[derive(Props)]
struct EqBandControlProps {
    band: EqBand,
    enabled: bool,
    on_change: EventHandler<f32>,
}

#[component]
fn EqBandControl(props: EqBandControlProps) -> Element {
    let gain = props.band.gain_db;
    let normalized = ((gain + 12.0) / 24.0).clamp(0.0, 1.0);

    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; gap: var(--space-2);",
            div {
                style: "width: 26px; height: 140px; background: rgba(244, 241, 233, 0.10); border-radius: var(--radius-medium); position: relative;",
                div {
                    style: format!(
                        "width: 100%; height: {}%; background: {}; border-radius: var(--radius-medium); bottom: 0; position: absolute; transform-origin: bottom; transition: height var(--transition-base), background var(--transition-base);",
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
                onchange: move |v| props.on_change.call(((v - 0.5) * 24.0 - 12.0) as f32),
                aria_label: format!("{} Hz band", props.band.frequency_hz),
            }
            div {
                style: "writing-mode: vertical-rl; text-orientation: mixed; font-size: var(--text-xs); color: var(--text-muted); font-family: var(--font-mono);",
                "{props.band.frequency_hz}Hz"
            }
            div {
                style: "font-size: var(--text-xs); font-family: var(--font-mono); color: var(--text-secondary);",
                "{gain:+.1} dB"
            }
        }
    }
}
