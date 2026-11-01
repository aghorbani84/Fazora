//! Media domain models.
//! Track, Album, Artist, Video, Playlist and library aggregates.

pub mod track;
pub mod album;
pub mod artist;
pub mod video;
pub mod playlist;
pub mod library;

pub use album::Album;
pub use artist::Artist;
pub use library::{Library, LibraryStats};
pub use playlist::Playlist;
pub use track::{LyricLine, Track};
pub use video::Video;
