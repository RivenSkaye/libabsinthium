//! # Absinthium (lib)
//!
//! A library for managing and mangling playlist files. It's still very much early days,
//! but the basic building blocks exist.
//!
//! This library exports a few traits from the top-level, and has several modules for
//! further format specifications. The idea here is that it should be possible for anyone,
//! anywhere to produce different playlist format handlers that should mostly be
//! plug-and-play. Write up the `impl`, slap it onto the [`Playlist`] struct, add whatever
//! specialized methods you like, and profit.

use std::{borrow::Cow, cell::RefCell, marker::PhantomData, ops::Deref};
use uriparse::uri;

pub mod m3u;
pub mod plaintext;

pub fn uri_is_file(uri: impl Deref<Target = str>) -> bool {
    false
}

/// A trait to describe the barest metadata reasonably present on a playlist entry.
///
/// The minimalism is inspired by extended m3u, the most common format in the wild.
pub trait EntryMetadata: PartialEq {
    /// If present, return the title or name set for the playlist entry.
    ///
    /// A sensible fallback implementation may also return the base filename if a title or
    /// name field isn't present on the playlist entry itself.
    fn title(&self) -> impl Deref<Target = str> + PartialEq;
    /// Produce the entry length, if present
    fn len(&self) -> Option<u32>;
    /// Produce all known info for this playlist entry, formatted as text.
    fn info(&self) -> impl Deref<Target = str> + PartialEq;
}

/// Basic entry information for a playlist.
///
/// If not explicitly available, this info can always be inferred.
pub trait Entry<M: EntryMetadata> {
    /// Get the number of the entry. Or its position in the playlist, if not specified
    fn entry_num(&self) -> u32;
    /// Get the filename or URI this entry points to
    fn filename(&self) -> Cow<str>;
    /// If present, get the metadata object
    fn metadata(&self) -> Option<M>;
    /// Overwrite the metadata object
    fn write_metadata(&self, metadata: M);
}

/// A trait to describe basic metadata on the playlist itself.
pub trait PlaylistInfo {
    /// If the playlist metadata defines a title or name for the playlist, return it.
    ///
    /// A sensible default to prevent [`None`] returns could very well be to return
    /// the base filename.
    fn title(&self) -> Option<impl Deref<Target = str>>;
    /// Provide the filename or URI this playlist is found. Can be relative or absolute.
    fn filename(&self) -> Cow<str>;
}

pub trait PlaylistFormat<P: PlaylistInfo, M: EntryMetadata, E: Entry<M>> {
    /// Read the resource from the given URI into a playlist.
    fn from_uri(uri: impl Deref<Target = str>) -> Self;
    /// Read the file from the given path into a playlist.
    fn from_path(path: impl Deref<Target = str>) -> Self;
    /// Parse a singular playlist entry.
    fn parse_entry<S: AsRef<str>>(text: impl Into<S>) -> E;
    /// Parse the metadata part of a playlist entry.
    fn parse_entry_metadata<S: AsRef<str>>(text: impl Into<S>) -> M;
    /// Parse metadata about the playlist itself.
    fn parse_playlist_info<S: AsRef<str>>(text: impl Into<S>) -> P;
    /// Deduplicate the entries in the playlist.
    ///
    /// This should match and deduplicate based on whatever equality is defined
    /// for the specified [`Entry`]. Should return the amount of entries that
    /// were removed from the playlist.
    fn dedup_entries(&self) -> usize;
    /// Change the path on the playlist file.
    fn rename(&self, new_name: impl Deref<Target = str>);
    /// Save the playlist.
    fn save(&self, path: impl Deref<Target = str>);
    /// Save the playlist to a specified path.
    fn save_to(&self, path: impl Deref<Target = str>);
    /// Create a playlist from its constituent parts.
    fn from_parts(info: P, entries: Vec<E>) -> Self;
    /// Get the metadata object for a playlist.
    fn get_metadata(&self) -> P;
    /// Add an entry to the end of the playlist.
    fn add_entry(&self, entry: E);
    /// Add an entry to a specific point in the playlist.
    fn add_entry_at(&self, entry: E, index: usize);
    /// Remove an entry from the playlist at a specific index.
    fn remove_entry(&self, entry: usize) -> E;
    /// Return a count of the amount of elements in the playlist.
    fn count(&self) -> usize;
    ///
    fn merge(&self, other: Self) -> Self;
}

pub struct Playlist<P: PlaylistInfo, M: EntryMetadata, E: Entry<M>> {
    /// Playlist entries, kept in an [`RefCell`] for interior mutability purposes
    entries: RefCell<Vec<E>>,
    /// Playlist info, kept in an [`RefCell`] for mutability purposes
    info: RefCell<P>,
    #[doc(hidden)]
    phantom: PhantomData<M>,
}

impl<P: PlaylistInfo + Clone, M: EntryMetadata + Clone, E: Entry<M> + Clone>
    Playlist<P, M, E>
{
    /// Creates a playlist from a block of metadata and a Vec of entries
    pub fn from_parts(info: P, entries: Vec<E>) -> Self {
        Self { entries: RefCell::new(entries), info: RefCell::new(info), phantom: PhantomData }
    }

    pub fn get_metadata(&self) -> P {
        self.info.borrow().clone()
    }

    pub fn add_entry(&self, entry: E) {
        self.entries.borrow_mut().push(entry)
    }

    pub fn remove_entry(&self, entry: usize) -> E {
        self.entries.borrow_mut().remove(entry)
    }

    pub fn count(&self) -> usize {
        self.entries.borrow().len()
    }

    pub fn merge(&self, other: Self) -> Self {
        let new_list = self
            .entries
            .borrow()
            .iter()
            .chain(other.entries.borrow().iter())
            .cloned()
            .collect();
        Self {
            entries: RefCell::new(new_list),
            info: RefCell::clone(&self.info),
            phantom: self.phantom,
        }
    }
}
