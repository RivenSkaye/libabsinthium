use std::{borrow::Cow, cell::UnsafeCell, marker::PhantomData, path::PathBuf};

/// A trait to describe pretty bare metadata, inspired by extended m3u, which is the most common format.
pub trait EntryMetadata {
    /// Produce the title
    fn title(&self) -> Cow<str>;
    /// Produce the entry length, if present
    fn length(&self) -> Option<u32>;
    /// Produce all info, formatted
    fn info(&self) -> Cow<str>;
}

/// Bare entry information for a playlist.
///
/// If not explicitly available, this info can always be inferred.
pub trait Entry<M: EntryMetadata + Default> {
    /// Get the number of the entry. Or its position in the playlist, if not specified
    fn entry_num(&self) -> u32;
    /// Get the filename or URI this entry points to
    fn filename(&self) -> Cow<str>;
    /// If present, get the metadata object
    fn metadata(&self) -> M;
    /// Overwrite the metadata object
    fn write_metadata(&mut self, metadata: M);
}

pub trait PlaylistInfo {
    /// If the playlist metadata defines a title or name for the playlist, produce it.
    fn title(&self) -> Option<Cow<str>>;
    /// Provide the filename or URI this playlist is found
    fn filename(&self) -> Cow<str>;
}

pub trait PlalistFormat<
    P: PlaylistInfo + Default,
    M: EntryMetadata + Default,
    E: Entry<M> + Default,
>
{
    fn new(uri: impl Into<PathBuf>) -> Self;
    fn parse_entry<S: AsRef<str>>(text: impl Into<S>) -> E;
    fn parse_entry_metadata<S: AsRef<str>>(text: impl Into<S>) -> M;
    fn parse_playlist_info<S: AsRef<str>>(text: impl Into<S>) -> P;
}

#[derive(Debug, Default)]
pub struct Playlist<P: PlaylistInfo + Default, T: EntryMetadata + Default, E: Entry<T> + Default> {
    entries: UnsafeCell<Vec<E>>,
    info: UnsafeCell<P>,
    phantom: PhantomData<T>,
}

impl<P: PlaylistInfo + Default, T: EntryMetadata + Default, E: Entry<T> + Default>
    Playlist<P, T, E>
{
    pub fn from_parts(info: P, entries: Vec<E>) -> Self {
        Self {
            entries: UnsafeCell::new(entries),
            info: UnsafeCell::from(info),
            phantom: PhantomData,
        }
    }

    fn add_entry(&self, entry: E) {
        unsafe { self.entries.get().as_mut().unwrap() }.push(entry);
    }
}
