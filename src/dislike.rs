use crate::Hct;

pub fn is_disliked(hct: &Hct) -> bool {
    let hue_passes = (90.0..=111.0).contains(&hct.get_hue().round());
    let chroma_passes = hct.get_chroma().round() > 16.0;
    let tone_passes = hct.get_tone().round() < 65.0;

    hue_passes && chroma_passes && tone_passes
}

/// If [hct] is disliked, lighten it to make it likable.
pub fn fix_if_disliked(hct: Hct) -> Hct {
    if is_disliked(&hct) {
        return Hct::from(hct.get_hue(), hct.get_chroma(), 70.0);
    }

    hct
}
