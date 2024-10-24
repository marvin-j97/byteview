// Copyright (c) 2024-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

//! An immutable byte slice that may be inlined, and can be partially cloned without heap allocation.
//!
//! The length is limited to 2^32 bytes (4 GiB).
//!
//! ```
//! # use byteview::ByteView;
//! let slice = ByteView::from("helloworld_thisisaverylongstring");
//!
//! // No heap allocation - increases the ref count like an Arc<[u8]>
//! let full_copy = slice.clone();
//!
//! assert_eq!(2, slice.ref_count());
//! ```

#![deny(clippy::all, missing_docs, clippy::cargo)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::indexing_slicing)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::indexing_slicing
)]

mod byteview;
mod strview;

pub use {byteview::ByteView, strview::StrView};
