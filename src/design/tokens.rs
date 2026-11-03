//! Rust-side mirror of the CSS design tokens.
//! Used by canvas-based visualisers, custom layout math and hit testing.
//! Values must stay in sync with `src/design/tokens.css`.

/// Brand colours (premultiplied sRGB, 0..=1).
pub const FAZORA_OBSIDIAN: (f32, f32, f32) = (0.043, 0.051, 0.063); // #0B0D10
pub const FAZORA_CARBON: (f32, f32, f32) = (0.082, 0.098, 0.122); // #15191F
pub const FAZORA_GOLD: (f32, f32, f32) = (0.847, 0.710, 0.416); // #D8B56A
pub const FAZORA_IVORY: (f32, f32, f32) = (0.957, 0.945, 0.914); // #F4F1E9
pub const FAZORA_STEEL: (f32, f32, f32) = (0.537, 0.569, 0.616); // #89919D
pub const FAZORA_TEAL: (f32, f32, f32) = (0.471, 0.812, 0.757); // #78CFC1

/// Layout metrics (logical pixels).
pub const SIDEBAR_WIDTH: f32 = 260.0;
pub const SIDEBAR_COMPACT_WIDTH: f32 = 72.0;
pub const TOPBAR_HEIGHT: f32 = 56.0;
pub const PLAYER_DOCK_HEIGHT: f32 = 88.0;
pub const SECONDARY_PANEL_WIDTH: f32 = 360.0;

/// Border radii.
pub const RADIUS_SMALL: f32 = 6.0;
pub const RADIUS_MEDIUM: f32 = 10.0;
pub const RADIUS_LARGE: f32 = 16.0;
pub const RADIUS_XL: f32 = 22.0;

/// Motion durations.
pub const MOTION_FAST: f32 = 0.12; // 120ms
pub const MOTION_BASE: f32 = 0.18; // 180ms
pub const MOTION_SLOW: f32 = 0.24; // 240ms

pub fn fazora_gold_u32() -> u32 {
    0xFF_D8B56A
}

pub fn fazora_ivory_u32() -> u32 {
    0xFF_F4F1E9
}
