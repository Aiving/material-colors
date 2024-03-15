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
    clippy::too_many_lines,
    clippy::doc_markdown,
    clippy::missing_panics_doc,
    clippy::implicit_hasher,  // we use ahash on Scheme
    clippy::suboptimal_flops, // some more cases can be optimized
    // nursery lints for later
    clippy::large_stack_frames,
    clippy::cognitive_complexity
)]

pub mod blend;
pub mod color;
pub mod contrast;
pub mod dislike;
pub mod dynamic_color;
pub mod hct;
#[cfg(feature = "image")]
pub mod image;
pub mod palettes;
pub mod quantize;
pub mod scheme;
pub mod score;
pub mod temperature;
pub mod theme;
pub mod utils;

pub use hct::Hct;

pub use scheme::{
    variant::{
        SchemeContent, SchemeExpressive, SchemeFidelity, SchemeFruitSalad, SchemeMonochrome,
        SchemeNeutral, SchemeRainbow, SchemeTonalSpot, SchemeVibrant,
    },
    Scheme,
};

pub use color::{Argb, Rgb};

pub use theme::{CustomColorGroup, Theme};

#[cfg(feature = "image")]
pub use image::{FilterType, Image, ImageReader};

pub use palettes::{CorePalette, TonalPalette};
