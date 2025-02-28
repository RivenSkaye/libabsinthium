use std::{borrow::Cow, cell::RefCell, marker::PhantomData, path::PathBuf};

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
pub trait Entry<M: EntryMetadata> {
    /// Get the number of the entry. Or its position in the playlist, if not specified
    fn entry_num(&self) -> u32;
    /// Get the filename or URI this entry points to
    fn filename(&self) -> Cow<str>;
    /// If present, get the metadata object
    fn metadata(&self) -> &M;
    /// Overwrite the metadata object
    fn write_metadata(&mut self, metadata: M);
}

pub trait PlaylistInfo {
    /// If the playlist metadata defines a title or name for the playlist, produce it.
    fn title(&self) -> Option<Cow<str>>;
    /// Provide the filename or URI this playlist is found
    fn filename(&self) -> Cow<str>;
}

pub trait PlalistFormat<P: PlaylistInfo, M: EntryMetadata, E: Entry<M>> {
    /// Read a file or URI into a playlist
    fn from_uri(uri: impl Into<PathBuf>) -> Self;
    /// Parse a singular playlist entry
    fn parse_entry<S: AsRef<str>>(text: impl Into<S>) -> E;
    /// Parse the metadata part of a playlist entry
    fn parse_entry_metadata<S: AsRef<str>>(text: impl Into<S>) -> M;
    /// Parse metadata about the playlist itself
    fn parse_playlist_info<S: AsRef<str>>(text: impl Into<S>) -> P;
}

pub struct Playlist<P: PlaylistInfo, M: EntryMetadata, E: Entry<M>> {
    /// Playlist entries, kept in an [`RefCell`] for interior mutability purposes
    entries: RefCell<Vec<E>>,
    /// Playlist info, kept in an [`RefCell`] for mutability purposes
    info: RefCell<P>,
    #[doc(hidden)]
    phantom: PhantomData<M>,
}

impl<P: PlaylistInfo + Clone, M: EntryMetadata + Clone, E: Entry<M> + Clone> Playlist<P, M, E> {
    /// Creates a playlist from a block of metadata and a Vec of entries
    pub fn from_parts(info: P, entries: Vec<E>) -> Self {
        Self {
            entries: RefCell::new(entries),
            info: RefCell::new(info),
            phantom: PhantomData,
        }
    }

    pub fn add_entry(&self, entry: E) {
        self.entries.borrow_mut().push(entry);
    }

    pub fn get_metadata(&self) -> P {
        self.info.borrow().clone()
    }

    pub fn count(&self) -> usize {
        self.entries.borrow().len()
    }
}
