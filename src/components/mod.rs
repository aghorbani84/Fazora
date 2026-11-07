//! FAZORA component library.
//! Primitive UI elements, branding, layout, media, player, audio and dialogs.

pub mod audio;
pub mod branding;
pub mod layout;
#[allow(dead_code, reason = "P2 will wire the SVG sprite into icon components")]
pub const ICONS: &str = include_str!("../../assets/icons.svg");
pub mod media;
pub mod player;
pub mod primitives;
