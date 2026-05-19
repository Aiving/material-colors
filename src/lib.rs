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
)]

#[cfg(all(feature = "quantize", not(feature = "alloc")))]
compile_error!("\"quantize\" feature requires \"alloc\" feature");

#[cfg(all(feature = "std", feature = "libm"))]
compile_error!("features \"std\" and \"libm\" cannot be enabled simultaneously");

#[cfg(all(not(feature = "std"), not(feature = "libm")))]
compile_error!("\"libm\" feature is required");

#[cfg(feature = "alloc")] extern crate alloc;
#[cfg(feature = "std")] extern crate std;

#[cfg(feature = "quantize")]
pub(crate) type IndexMap<K, V> = indexmap::IndexMap<K, V, core::hash::BuildHasherDefault<ahash::AHasher>>;

pub mod blend;
pub mod color;
pub mod contrast;
pub mod dislike;
pub mod dynamic_color;
pub mod error;
pub mod hct;
#[cfg(feature = "quantize")] pub mod image;
pub mod palette;
#[cfg(feature = "quantize")] pub mod quantize;
pub mod scheme;
#[cfg(feature = "quantize")] pub mod score;
pub mod temperature;
pub mod theme;
pub mod utils;

pub use error::Error;
