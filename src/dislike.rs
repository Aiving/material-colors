use crate::hct::Hct;
#[cfg(not(feature = "std"))]
#[allow(unused_imports)]
use crate::utils::no_std::FloatExt;

pub fn is_disliked(hct: &Hct) -> bool {
    let (hue_passes, chroma_passes, tone_passes) = (
        (90.0..=111.0).contains(&hct.get_hue().round()),
        hct.get_chroma().round() > 16.0,
        hct.get_tone().round() < 65.0,
    );

    hue_passes && chroma_passes && tone_passes
}

/// If `hct` is disliked, lighten it to make it likable.
pub fn fix_if_disliked(hct: Hct) -> Hct {
    if is_disliked(&hct) {
        return Hct::from(hct.get_hue(), hct.get_chroma(), 70.0);
    }

    hct
}
