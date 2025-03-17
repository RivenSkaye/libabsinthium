//! M3U and EXT-M3U
//!
//! M3U as a format is _very_ barebones. Literally [`plaintext`] with a name.
//! The extended format version, EXT-M3U, is a de facto standard. There is no official
//! specification, but the format is so well-known and widespread that we know what to
//! expect and what is actually out there in the wild. That said, I'm always open for
//! playlist files to further the possibilities that Absinthium can handle.

use super::*;
