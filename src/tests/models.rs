//! Model tests — pure logic, no Dioxus signals required.

use crate::models::*;

#[test]
fn track_formatted_duration() {
    let track = Track {
        id: "t1".to_string(),
        title: "Test Track".to_string(),
        artist: "Test Artist".to_string(),
        artist_id: None,
        album: None,
        album_id: None,
        genre: None,
        year: None,
        track_number: None,
        duration_sec: Some(125),
        path: "/music/test.mp3".to_string(),
        artwork: None,
        play_count: 0,
        rating: 0,
        added_at: 0,
        last_played: None,
        bitrate: None,
        sample_rate: None,
        channels: None,
        lyrics_synced: None,
        lyrics: None,
    };

    assert_eq!(track.formatted_duration(), Some("2:05".to_string()));
}

#[test]
fn track_formatted_duration_none() {
    let track = Track {
        id: "t1".to_string(),
        title: "Test Track".to_string(),
        artist: "Test Artist".to_string(),
        artist_id: None,
        album: None,
        album_id: None,
        genre: None,
        year: None,
        track_number: None,
        duration_sec: None,
        path: "/music/test.mp3".to_string(),
        artwork: None,
        play_count: 0,
        rating: 0,
        added_at: 0,
        last_played: None,
        bitrate: None,
        sample_rate: None,
        channels: None,
        lyrics_synced: None,
        lyrics: None,
    };

    assert_eq!(track.formatted_duration(), None);
}

#[test]
fn track_subtitle_full() {
    let track = Track {
        id: "t1".to_string(),
        title: "Test Track".to_string(),
        artist: "The Beatles".to_string(),
        artist_id: None,
        album: Some("Abbey Road".to_string()),
        album_id: None,
        genre: None,
        year: None,
        track_number: None,
        duration_sec: None,
        path: "/music/test.mp3".to_string(),
        artwork: None,
        play_count: 0,
        rating: 0,
        added_at: 0,
        last_played: None,
        bitrate: None,
        sample_rate: None,
        channels: None,
        lyrics_synced: None,
        lyrics: None,
    };

    assert_eq!(track.subtitle(), "The Beatles · Abbey Road");
}

#[test]
fn track_subtitle_artist_only() {
    let track = Track {
        id: "t1".to_string(),
        title: "Test Track".to_string(),
        artist: "The Beatles".to_string(),
        artist_id: None,
        album: None,
        album_id: None,
        genre: None,
        year: None,
        track_number: None,
        duration_sec: None,
        path: "/music/test.mp3".to_string(),
        artwork: None,
        play_count: 0,
        rating: 0,
        added_at: 0,
        last_played: None,
        bitrate: None,
        sample_rate: None,
        channels: None,
        lyrics_synced: None,
        lyrics: None,
    };

    assert_eq!(track.subtitle(), "The Beatles");
}

#[test]
fn track_subtitle_unknown() {
    let track = Track {
        id: "t1".to_string(),
        title: "Test Track".to_string(),
        artist: "Unknown Artist".to_string(),
        artist_id: None,
        album: None,
        album_id: None,
        genre: None,
        year: None,
        track_number: None,
        duration_sec: None,
        path: "/music/test.mp3".to_string(),
        artwork: None,
        play_count: 0,
        rating: 0,
        added_at: 0,
        last_played: None,
        bitrate: None,
        sample_rate: None,
        channels: None,
        lyrics_synced: None,
        lyrics: None,
    };

    assert_eq!(track.subtitle(), "Unknown Artist");
}

#[test]
fn album_subtitle_with_year() {
    let album = Album {
        id: "a1".to_string(),
        title: "Abbey Road".to_string(),
        artist: "The Beatles".to_string(),
        artist_id: None,
        year: Some(1969),
        genre: Some("Rock".to_string()),
        track_count: Some(17),
        duration_sec: None,
        artwork: None,
        added_at: 0,
    };

    assert_eq!(album.subtitle(), "The Beatles · 1969");
}

#[test]
fn album_subtitle_no_year() {
    let album = Album {
        id: "a1".to_string(),
        title: "Abbey Road".to_string(),
        artist: "The Beatles".to_string(),
        artist_id: None,
        year: None,
        genre: None,
        track_count: None,
        duration_sec: None,
        artwork: None,
        added_at: 0,
    };

    assert_eq!(album.subtitle(), "The Beatles");
}

#[test]
fn video_formatted_duration() {
    let video = Video {
        id: "v1".to_string(),
        title: "Test Video".to_string(),
        path: "/video/test.mp4".to_string(),
        thumbnail: None,
        duration_sec: Some(3661),
        width: Some(1920),
        height: Some(1080),
        watched_position_sec: None,
        watched: false,
        favorite: false,
        added_at: 0,
        codec: None,
        fps: None,
    };

    assert_eq!(video.formatted_duration(), Some("61:01".to_string()));
}

#[test]
fn video_progress_ratio() {
    let video = Video {
        id: "v1".to_string(),
        title: "Test Video".to_string(),
        path: "/video/test.mp4".to_string(),
        thumbnail: None,
        duration_sec: Some(100),
        width: None,
        height: None,
        watched_position_sec: Some(50),
        watched: false,
        favorite: false,
        added_at: 0,
        codec: None,
        fps: None,
    };

    assert_eq!(video.progress_ratio(), 0.5);
}

#[test]
fn video_progress_ratio_no_duration() {
    let video = Video {
        id: "v1".to_string(),
        title: "Test Video".to_string(),
        path: "/video/test.mp4".to_string(),
        thumbnail: None,
        duration_sec: None,
        width: None,
        height: None,
        watched_position_sec: None,
        watched: false,
        favorite: false,
        added_at: 0,
        codec: None,
        fps: None,
    };

    assert_eq!(video.progress_ratio(), 0.0);
}

#[test]
fn video_resolution_label() {
    let video = Video {
        id: "v1".to_string(),
        title: "Test Video".to_string(),
        path: "/video/test.mp4".to_string(),
        thumbnail: None,
        duration_sec: None,
        width: Some(1920),
        height: Some(1080),
        watched_position_sec: None,
        watched: false,
        favorite: false,
        added_at: 0,
        codec: None,
        fps: None,
    };

    assert_eq!(video.resolution_label(), Some("1920x1080".to_string()));
}

#[test]
fn playlist_track_count() {
    let playlist = Playlist {
        id: "p1".to_string(),
        title: "My Playlist".to_string(),
        description: None,
        track_ids: vec!["t1".to_string(), "t2".to_string(), "t3".to_string()],
        artwork: None,
        created_at: 0,
        modified_at: 0,
        favorite: false,
    };

    assert_eq!(playlist.track_count(), 3);
}

#[test]
fn playlist_track_count_empty() {
    let playlist = Playlist {
        id: "p1".to_string(),
        title: "Empty Playlist".to_string(),
        description: None,
        track_ids: vec![],
        artwork: None,
        created_at: 0,
        modified_at: 0,
        favorite: false,
    };

    assert_eq!(playlist.track_count(), 0);
}

#[test]
fn library_find_track() {
    let library = Library {
        tracks: vec![
            Track {
                id: "t1".to_string(),
                title: "Track One".to_string(),
                artist: "Artist One".to_string(),
                artist_id: None,
                album: None,
                album_id: None,
                genre: None,
                year: None,
                track_number: None,
                duration_sec: None,
                path: "/music/t1.mp3".to_string(),
                artwork: None,
                play_count: 0,
                rating: 0,
                added_at: 0,
                last_played: None,
                bitrate: None,
                sample_rate: None,
                channels: None,
                lyrics_synced: None,
                lyrics: None,
            },
            Track {
                id: "t2".to_string(),
                title: "Track Two".to_string(),
                artist: "Artist Two".to_string(),
                artist_id: None,
                album: None,
                album_id: None,
                genre: None,
                year: None,
                track_number: None,
                duration_sec: None,
                path: "/music/t2.mp3".to_string(),
                artwork: None,
                play_count: 0,
                rating: 0,
                added_at: 0,
                last_played: None,
                bitrate: None,
                sample_rate: None,
                channels: None,
                lyrics_synced: None,
                lyrics: None,
            },
        ],
        albums: vec![],
        artists: vec![],
        videos: vec![],
        playlists: vec![],
        stats: LibraryStats::default(),
    };

    assert!(library.find_track("t1").is_some());
    assert_eq!(library.find_track("t1").unwrap().title, "Track One");
    assert!(library.find_track("t99").is_none());
}

#[test]
fn library_find_album() {
    let library = Library {
        tracks: vec![],
        albums: vec![Album {
            id: "a1".to_string(),
            title: "Album One".to_string(),
            artist: "Artist One".to_string(),
            artist_id: None,
            year: None,
            genre: None,
            track_count: None,
            duration_sec: None,
            artwork: None,
            added_at: 0,
        }],
        artists: vec![],
        videos: vec![],
        playlists: vec![],
        stats: LibraryStats::default(),
    };

    assert!(library.find_album("a1").is_some());
    assert!(library.find_album("a99").is_none());
}

#[test]
fn library_find_artist() {
    let library = Library {
        tracks: vec![],
        albums: vec![],
        artists: vec![Artist {
            id: "ar1".to_string(),
            name: "Artist One".to_string(),
            album_count: None,
            track_count: None,
            artwork: None,
            genre: None,
            added_at: 0,
        }],
        videos: vec![],
        playlists: vec![],
        stats: LibraryStats::default(),
    };

    assert!(library.find_artist("ar1").is_some());
    assert!(library.find_artist("ar99").is_none());
}

#[test]
fn library_find_video() {
    let library = Library {
        tracks: vec![],
        albums: vec![],
        artists: vec![],
        videos: vec![Video {
            id: "v1".to_string(),
            title: "Video One".to_string(),
            path: "/video/v1.mp4".to_string(),
            thumbnail: None,
            duration_sec: None,
            width: None,
            height: None,
            watched_position_sec: None,
            watched: false,
            favorite: false,
            added_at: 0,
            codec: None,
            fps: None,
        }],
        playlists: vec![],
        stats: LibraryStats::default(),
    };

    assert!(library.find_video("v1").is_some());
    assert!(library.find_video("v99").is_none());
}

#[test]
fn library_find_playlist() {
    let library = Library {
        tracks: vec![],
        albums: vec![],
        artists: vec![],
        videos: vec![],
        playlists: vec![Playlist {
            id: "pl1".to_string(),
            title: "Playlist One".to_string(),
            description: None,
            track_ids: vec![],
            artwork: None,
            created_at: 0,
            modified_at: 0,
            favorite: false,
        }],
        stats: LibraryStats::default(),
    };

    assert!(library.find_playlist("pl1").is_some());
    assert!(library.find_playlist("pl99").is_none());
}

#[test]
fn library_default_stats() {
    let stats = LibraryStats::default();
    assert_eq!(stats.track_count, 0);
    assert_eq!(stats.album_count, 0);
    assert_eq!(stats.artist_count, 0);
    assert_eq!(stats.video_count, 0);
    assert_eq!(stats.playlist_count, 0);
    assert_eq!(stats.total_duration_sec, 0);
    assert_eq!(stats.indexed_dirs, 0);
    assert!(stats.last_scan.is_none());
}

#[test]
fn library_default() {
    let library = Library::default();
    assert!(library.tracks.is_empty());
    assert!(library.albums.is_empty());
    assert!(library.artists.is_empty());
    assert!(library.videos.is_empty());
    assert!(library.playlists.is_empty());
}

#[test]
fn lyric_line_creation() {
    let line = LyricLine {
        time: 1.5,
        text: "Hello world".to_string(),
    };

    assert_eq!(line.time, 1.5);
    assert_eq!(line.text, "Hello world");
}
