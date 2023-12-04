pub(crate) mod blend;
pub(crate) mod contrast;
pub(crate) mod dislike;
pub(crate) mod dynamic_color;
pub(crate) mod hct;
pub(crate) mod palettes;
pub(crate) mod quantize;
pub(crate) mod scheme;
pub(crate) mod score;
pub(crate) mod temperature;
pub(crate) mod utils;

pub use scheme::content::SchemeContent;
pub use scheme::expressive::SchemeExpressive;
pub use scheme::fidelity::SchemeFidelity;
pub use scheme::fruit_salad::SchemeFruitSalad;
pub use scheme::monochrome::SchemeMonochrome;
pub use scheme::neutral::SchemeNeutral;
pub use scheme::rainbow::SchemeRainbow;
pub use scheme::tonal_spot::SchemeTonalSpot;
pub use scheme::vibrant::SchemeVibrant;
pub use scheme::Scheme;

pub use utils::image::source_color_from_image;
pub use utils::string::argb_from_hex;
pub use utils::theme::theme_from_source_color;
