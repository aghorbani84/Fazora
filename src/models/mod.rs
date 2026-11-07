//! Media domain models.
//! Track, Album, Artist, Video, Playlist and library aggregates.

pub mod album;
pub mod artist;
pub mod library;
pub mod playlist;
pub mod track;
pub mod video;

pub use album::Album;
pub use artist::Artist;
pub use library::{Library, LibraryStats};
pub use playlist::Playlist;
pub use track::{LyricLine, Track};
pub use video::Video;

#[allow(unused_imports, reason = "re-exported by the library crate's public model surface")]
pub use crate::search::SearchResults;
