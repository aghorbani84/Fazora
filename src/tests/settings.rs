//! Settings service tests.
//!
//! Uses Dioxus's testing render scope because `SettingsService::init()`
//! creates `Signal`s via `use_signal()` which requires a component scope.

use crate::settings::{Settings, SettingsService, Theme};

#[test]
fn settings_initial_state() {
    let mut scope = dioxus::testing::RenderScope::new();
    let settings = scope.create_service(|| SettingsService::init());

    assert_eq!(settings.settings.read().theme, Theme::Dark);
    assert_eq!(settings.settings.read().language, "en");
    assert_eq!(settings.settings.read().ui_density, "comfortable");
    assert!(settings.settings.read().animations);
    assert!(settings.settings.read().scan_on_startup);
    assert!(settings.settings.read().autoplay);
    assert_eq!(settings.settings.read().repeat_mode, "all");
    assert!(!settings.settings.read().shuffle);
    assert_eq!(settings.settings.read().playback_speed, 1.0);
    assert!(settings.settings.read().output_device.is_none());
    assert!(!settings.settings.read().equalizer_enabled);
    assert_eq!(settings.settings.read().crossfade_sec, 0.0);
    assert!(settings.settings.read().gapless);
    assert!(settings.settings.read().media_dirs.is_empty());
    assert!(settings.settings.read().recent_queries.is_empty());
    assert!(!settings.settings.read().window_maximised);
}

#[test]
fn settings_default() {
    let settings = Settings::default();
    assert_eq!(settings.theme, Theme::Dark);
    assert_eq!(settings.language, "en");
    assert_eq!(settings.ui_density, "comfortable");
    assert!(settings.animations);
    assert!(settings.scan_on_startup);
    assert!(settings.autoplay);
    assert_eq!(settings.repeat_mode, "all");
    assert!(!settings.shuffle);
    assert_eq!(settings.playback_speed, 1.0);
    assert!(settings.output_device.is_none());
    assert!(!settings.equalizer_enabled);
    assert_eq!(settings.crossfade_sec, 0.0);
    assert!(settings.gapless);
    assert!(settings.media_dirs.is_empty());
    assert!(settings.recent_queries.is_empty());
    assert!(!settings.window_maximised);
}

#[test]
fn settings_add_recent_query() {
    let mut scope = dioxus::testing::RenderScope::new();
    let settings = scope.create_service(|| SettingsService::init());

    settings.add_recent_query("queen".to_string());
    settings.add_recent_query("beatles".to_string());
    settings.add_recent_query("pink floyd".to_string());

    let queries = &settings.settings.read().recent_queries;
    assert_eq!(queries.len(), 3);
    assert_eq!(queries[0], "pink floyd"); // most recent first
    assert_eq!(queries[1], "beatles");
    assert_eq!(queries[2], "queen");
}

#[test]
fn settings_add_recent_query_moves_to_front() {
    let mut scope = dioxus::testing::RenderScope::new();
    let settings = scope.create_service(|| SettingsService::init());

    settings.add_recent_query("queen".to_string());
    settings.add_recent_query("beatles".to_string());
    settings.add_recent_query("queen".to_string()); // duplicate, moves to front

    let queries = &settings.settings.read().recent_queries;
    assert_eq!(queries.len(), 2);
    assert_eq!(queries[0], "queen");
    assert_eq!(queries[1], "beatles");
}

#[test]
fn settings_add_recent_query_truncates_to_10() {
    let mut scope = dioxus::testing::RenderScope::new();
    let settings = scope.create_service(|| SettingsService::init());

    for i in 0..15 {
        settings.add_recent_query(format!("query {}", i));
    }

    let queries = &settings.settings.read().recent_queries;
    assert_eq!(queries.len(), 10);
    assert_eq!(queries[0], "query 14"); // most recent
    assert_eq!(queries[9], "query 5"); // oldest kept
}

#[test]
fn settings_clear_recent_queries() {
    let mut scope = dioxus::testing::RenderScope::new();
    let settings = scope.create_service(|| SettingsService::init());

    settings.add_recent_query("queen".to_string());
    settings.add_recent_query("beatles".to_string());
    assert_eq!(settings.settings.read().recent_queries.len(), 2);

    settings.clear_recent_queries();
    assert!(settings.settings.read().recent_queries.is_empty());
}

#[test]
fn theme_variants() {
    assert_eq!(Theme::default(), Theme::Dark);
    assert_eq!(Theme::Dark as u32, 0);
    assert_eq!(Theme::Light as u32, 1);
    assert_eq!(Theme::System as u32, 2);
}
