//! Settings hub — categories, searchable.

use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let navigator = use_navigator();

    let sections = [
        (
            "General",
            "Language, theme, startup behavior, UI density",
            "general",
        ),
        (
            "Appearance",
            "Light/dark/system, accent color, typography, layout",
            "appearance",
        ),
        (
            "Audio",
            "Playback engine, output device, equalizer, DSP",
            "audio",
        ),
        (
            "Video",
            "Hardware decoding, subtitles, aspect ratio, filters",
            "video",
        ),
        (
            "Library",
            "Media directories, indexing, metadata, artwork",
            "library",
        ),
        (
            "Playback",
            "Resume position, autoplay, repeat, shuffle, sleep timer",
            "playback",
        ),
        (
            "Advanced",
            "Engine diagnostics, cache, performance, logs",
            "advanced",
        ),
        (
            "About",
            "Version, build information, licenses, credits",
            "about",
        ),
    ];

    rsx! {
        div {
            class: "fz-page",
            div {
                class: "fz-section-header",
                style: "margin-bottom: var(--space-8);",
                h1 {
                    style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary);",
                    "Settings"
                }
            }
            div {
                style: "max-width: 700px;",
                for (title, description, section) in sections {
                    div {
                        key: "{section}",
                        class: "fz-card",
                        style: "padding: var(--space-5); margin-bottom: var(--space-4); cursor: pointer;",
                        onclick: move |_| {
                            navigator.push(crate::routing::Route::SettingsSection {
                                section: section.to_string(),
                            });
                        },
                        div {
                            style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary);",
                            "{title}"
                        }
                        p {
                            style: "font-size: var(--text-sm); color: var(--text-secondary); margin-top: var(--space-1);",
                            "{description}"
                        }
                        div {
                            style: "margin-top: var(--space-3); font-size: var(--text-xs); color: var(--text-muted); text-align: right;",
                            "Open"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SettingsSection(section: String) -> Element {
    rsx! {
        div {
            class: "fz-page",
            h2 {
                style: "font-size: var(--text-2xl); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-6);",
                "{section}"
            }
            p { style: "color: var(--text-muted);", "Settings for {section} — implement per category." }
        }
    }
}
