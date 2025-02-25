use std::{borrow::Cow, marker::PhantomData};

/// A trait to describe pretty bare metadata, inspired by extended m3u, which is the most common format.
pub trait TrackMetadata {
    /// Produce the title
    fn title(&self) -> Cow<str>;
    /// Produce the track length, if present
    fn length(&self) -> Option<u32>;
    /// Produce all info, formatted
    fn info(&self) -> Cow<str>;
}

/// Bare track information for a playlist.
///
/// If not explicitly available, this info can always be inferred.
pub trait Track<T: TrackMetadata> {
    /// Get the number of the track. Or its position in the playlist, if not specified
    fn track_num(&self) -> u32;
    /// Get the filename or URI this track points to
    fn filename(&self) -> Cow<str>;
    /// If present, get the metadata object
    fn metadata(&self) -> T;
    /// Overwrite the metadata object
    fn write_metadata(&mut self, metadata: T);
}

pub trait PlaylistInfo {
    /// If the playlist metadata defines a title or name for the playlist, produce it.
    fn title(&self) -> Option<Cow<str>>;
    /// Provide the filename or URI this playlist is found
    fn filename(&self) -> Cow<str>;
}

#[derive(Clone, Debug, Default)]
pub struct Playlist<P: PlaylistInfo, _T: TrackMetadata, E: Track<_T>> {
    entries: Vec<E>,
    info: P,
    phantom: PhantomData<_T>,
}

impl<P: PlaylistInfo + Default, T: TrackMetadata + Default, E: Track<T> + Default>
    Playlist<P, T, E>
{
    pub fn new(info: P, entries: Vec<E>) -> Self {
        Self {
            entries,
            info,
            phantom: PhantomData,
        }
    }
}
