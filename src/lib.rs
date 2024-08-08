#![doc = include_str!("../README.md")]
#![no_std]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    // pedantic lints
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::many_single_char_names,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::unreadable_literal,
    clippy::used_underscore_binding,
    clippy::similar_names,
    // pedantic lints for later
    clippy::doc_markdown,
    clippy::too_many_lines,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::implicit_hasher,  // we use ahash on Scheme
    // nursery lints for later
    clippy::while_float,
    clippy::cognitive_complexity,
    clippy::derive_ord_xor_partial_ord,
    // cargo lints
    clippy::negative_feature_names
)]

#[cfg(all(feature = "image", not(feature = "std")))]
compile_error!("\"image\" feature requires \"std\" feature");

#[cfg(all(feature = "std", feature = "libm"))]
compile_error!("features \"std\" and \"libm\" cannot be enabled simultaneously");

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!("\"no-std\" requires \"libm\" feature");

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
pub(crate) use ahash::HashMap as Map;
#[cfg(not(feature = "std"))]
pub(crate) use alloc::collections::BTreeMap as Map;

pub(crate) type IndexMap<K, V> =
    indexmap::IndexMap<K, V, core::hash::BuildHasherDefault<ahash::AHasher>>;

pub mod blend;
pub mod color;
pub mod contrast;
pub mod dislike;
pub mod dynamic_color;
pub mod error;
pub mod hct;
#[cfg(feature = "image")]
pub mod image;
pub mod palette;
pub mod quantize;
pub mod scheme;
pub mod score;
pub mod temperature;
pub mod theme;
pub mod utils;

pub use error::Error;
