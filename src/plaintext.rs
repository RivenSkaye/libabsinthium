//! The really bare ones are just `ls -1`!
//!
//! Plaintext file listings are a valid playlist format. And coincidentally it's also the
//! format for the original `.m3u` playlists. This is part of why there is no official
//! specification for it, and it's instead just a de facto standard.  \
//! For more information, see [the Wikipedia page](https://en.m.wikipedia.org/wiki/M3U)
//!
//! As for the bare format being separate, this has two uses. First, the
//! [`m3u`][crate::m3u] module can safely offload non-extended format files here with no
//! change to the output whatsoever. Second, the world is a scary place and humans aren't
//! always that smart. So you may encounter playlists in the wild with seemingly random
//! extensions (or sometimes just `playlist` as the filename) and their contents are a
//! file listing.  \
//! This setup allows specifying the expected file to have this layout, which also makes
//! it a viable fallback if we fail to detect what kind of playlist file we actually are
//! dealing with. Worst case, this causes a GIGO ("Garbage In, Garbage Out") situation.

use std::cell::RefCell;

use super::*;

pub struct PlainEntry<'a> {
    pub num: u32,
    pub fname: Cow<'a, str>,
    pub metadata: RefCell<Option<PlainMetadata<'a>>>,
}

impl<'a> Entry<PlainMetadata<'a>> for PlainEntry<'a> {
    fn entry_num(&self) -> u32 {
        todo!()
    }

    fn filename(&self) -> Cow<str> {
        todo!()
    }

    fn metadata(&self) -> Option<PlainMetadata<'a>> {
        self.metadata.try_borrow().ok().map(|m| m.clone()).flatten()
    }

    /// Replaces the currently stored metadata
    ///
    /// ## Panics
    /// As this uses [`RefCell::replace`] under the hood, this will panic if there's
    /// active borrows of the inner Metadata object (though there shouldn't be).
    fn write_metadata(&self, metadata: PlainMetadata<'a>) {
        self.metadata.replace(Some(metadata)).map(drop).unwrap_or_default()
    }
}

#[derive(Clone)]
pub struct PlainMetadata<'a> {
    parent: &'a PlainEntry<'a>,
}

impl PartialEq for PlainMetadata<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.info() == other.info() && std::ptr::eq(self.parent, other.parent)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl EntryMetadata for PlainMetadata<'_> {
    fn title(&self) -> impl Deref<Target = str> + PartialEq {
        Cow::from("")
    }

    fn len(&self) -> Option<u32> {
        None
    }

    fn info(&self) -> impl Deref<Target = str> + PartialEq {
        ""
    }
}
