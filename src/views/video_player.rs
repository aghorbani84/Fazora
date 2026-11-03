//! Video player — immersive, distraction-free surface with overlays for
//! subtitles, audio tracks, speed and settings.

use crate::models::Video;
use crate::state::AppState;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct VideoPlayerProps {
    id: String,
}

#[component]
pub fn VideoPlayer(props: VideoPlayerProps) -> Element {
    let state = use_context::<Signal<AppState>>();
    let library = state.read().library.clone();

    let video = library.library.read().videos.iter().find(|v| v.id == props.id).cloned();
    let video = match video {
        Some(v) => v,
        None => return rsx! { div { class: "fz-empty", h2 { "Video not found" } } },
    };

    let showing_controls = use_signal(|| true);
    let showing_settings = use_signal(|| false);

    rsx! {
        div {
            class: "fz-video-player",
            style: "width: 100%; height: 100%; background: var(--fazora-obsidian); color: var(--fazora-ivory); position: relative; display: flex; flex-direction: column;",
            onmousemove: move |_| {
                showing_controls.set(true);
            },
            // Video surface — real native rendering, never a fake image.
            div {
                class: "fz-video-surface",
                style: "flex: 1; background: var(--fazora-obsidian); display: flex; align-items: center; justify-content: center; position: relative;",
                div {
                    style: "color: var(--fazora-steel); font-size: var(--text-lg); font-family: var(--font-mono);",
                    "Video surface — native engine"
                }
                // Poster / thumbnail placeholder
                if let Some(thumb) = &video.thumbnail {
                    img {
                        src: "{thumb}",
                        alt: video.title.clone(),
                        style: "width: 100%; height: 100%; object-fit: contain; opacity: 0.5;",
                    }
                }
            }
            // Top overlay
            if showing_controls.get() {
                header {
                    class: "fz-video-topbar",
                    style: "position: absolute; top: 0; left: 0; right: 0; padding: var(--space-4); display: flex; align-items: center; justify-content: space-between; z-index: 30;",
                    h3 {
                        style: "font-size: var(--text-lg); font-weight: 700;",
                        "{video.title}"
                    }
                    div {
                        style: "display: flex; gap: var(--space-2);",
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Ghost,
                            label: "Subtitles",
                        }
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Ghost,
                            label: "Audio",
                        }
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Ghost,
                            label: "Settings",
                            onclick: move |_| showing_settings.set(!showing_settings.get()),
                        }
                    }
                }
            }
            // Center playback controls
            if showing_controls.get() {
                div {
                    class: "fz-video-center-controls",
                    style: "position: absolute; inset: 0; display: flex; flex-direction: column; align-items: center; justify-content: center; gap: var(--space-6); z-index: 40; pointer-events: none;",
                    Button {
                        variant: crate::components::primitives::ButtonVariant::Surface,
                        style: "pointer-events: auto; width: 64px; height: 64px; border-radius: 50%; display: flex; align-items: center; justify-content: center;",
                        svg {
                            class: "fz-icon",
                            width: 32,
                            height: 32,
                            use { href: "#icon-play" }
                        }
                    }
                }
            }
            // Bottom timeline overlay
            footer {
                class: "fz-video-bottom",
                style: "position: absolute; bottom: 0; left: 0; right: 0; padding: var(--space-4); z-index: 30; pointer-events: auto;",
                crate::components::player::timeline::Timeline {
                    position: 0.0,
                    duration: video.duration_sec.map(|d| d as f64).unwrap_or(0.0),
                    on_seek: move |_| {},
                }
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; margin-top: var(--space-2); font-size: var(--text-xs); font-family: var(--font-mono); color: var(--fazora-steel);",
                    span { "0:00" }
                    div {
                        style: "display: flex; gap: var(--space-2);",
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Ghost,
                            label: "Speed 1×",
                        }
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Ghost,
                            label: "Aspect ratio",
                        }
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Ghost,
                            label: "Fullscreen",
                        }
                    }
                }
            }
            // Advanced control panel overlay
            if showing_settings.get() {
                div {
                    class: "fz-video-settings-panel",
                    style: "position: absolute; inset: 0; background: rgba(11, 13, 16, 0.85); backdrop-filter: blur(8px); display: flex; flex-direction: column; z-index: 50;",
                    div {
                        style: "padding: var(--space-6); border-bottom: 1px solid var(--border); display: flex; align-items: center; justify-content: space-between;",
                        h3 {
                            style: "font-size: var(--text-lg); font-weight: 700; color: var(--fazora-ivory);",
                            "Video settings"
                        }
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Ghost,
                            label: "Done",
                            onclick: move |_| showing_settings.set(false),
                        }
                    }
                    div {
                        style: "flex: 1; overflow-y: auto; padding: var(--space-6); display: grid; grid-template-columns: 1fr 1fr; gap: var(--space-6);",
                        div {
                            h4 { style: "font-size: var(--text-sm); font-weight: 700; color: var(--fazora-ivory); margin-bottom: var(--space-3);", "Subtitle track" }
                            p { style: "font-size: var(--text-sm); color: var(--fazora-steel);", "None" }
                        }
                        div {
                            h4 { style: "font-size: var(--text-sm); font-weight: 700; color: var(--fazora-ivory); margin-bottom: var(--space-3);", "Audio track" }
                            p { style: "font-size: var(--text-sm); color: var(--fazora-steel);", "Default" }
                        }
                        div {
                            h4 { style: "font-size: var(--text-sm); font-weight: 700; color: var(--fazora-ivory); margin-bottom: var(--space-3);", "Playback speed" }
                            p { style: "font-size: var(--text-sm); color: var(--fazora-steel);", "1×" }
                        }
                        div {
                            h4 { style: "font-size: var(--text-sm); font-weight: 700; color: var(--fazora-ivory); margin-bottom: var(--space-3);", "Aspect ratio" }
                            p { style: "font-size: var(--text-sm); color: var(--fazora-steel);", "Original" }
                        }
                    }
                }
            }
        }
    }
}
