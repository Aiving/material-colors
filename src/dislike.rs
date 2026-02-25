use crate::hct::Hct;
#[cfg(all(not(feature = "std"), feature = "libm"))]
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

#[cfg(test)]
mod tests {
    use super::is_disliked;
    use crate::{color::Argb, dislike::fix_if_disliked, hct::Hct};

    #[test]
    fn test_monk_skin_tone_scale_colors() {
        // From https://skintone.google#/get-started
        let monk_skin_tone_scale_colors = [
            Argb::from_u32(0xFFF6EDE4),
            Argb::from_u32(0xFFF3E7DB),
            Argb::from_u32(0xFFF7EAD0),
            Argb::from_u32(0xFFEADABA),
            Argb::from_u32(0xFFD7BD96),
            Argb::from_u32(0xFFA07E56),
            Argb::from_u32(0xFF825C43),
            Argb::from_u32(0xFF604134),
            Argb::from_u32(0xFF3A312A),
            Argb::from_u32(0xFF292420),
        ];

        for color in monk_skin_tone_scale_colors {
            assert!(!is_disliked(&color.into()));
        }
    }

    #[test]
    fn test_bile_colors_disliked() {
        let unlikable = [
            Argb::from_u32(0xFF95884B),
            Argb::from_u32(0xFF716B40),
            Argb::from_u32(0xFFB08E00),
            Argb::from_u32(0xFF4C4308),
            Argb::from_u32(0xFF464521),
        ];

        for color in unlikable {
            assert!(is_disliked(&color.into()));
        }
    }

    #[test]
    fn test_bile_colors_became_likable() {
        let unlikable = [
            Argb::from_u32(0xFF95884B),
            Argb::from_u32(0xFF716B40),
            Argb::from_u32(0xFFB08E00),
            Argb::from_u32(0xFF4C4308),
            Argb::from_u32(0xFF464521),
        ];

        for color in unlikable {
            let hct = color.into();

            assert!(is_disliked(&hct));

            let likable = fix_if_disliked(hct);

            assert!(!is_disliked(&likable));
        }
    }

    #[test]
    fn test_tone67_not_disliked() {
        let color = Hct::from(100.0, 50.0, 67.0);

        assert!(!is_disliked(&color));
        assert_eq!(Argb::from(fix_if_disliked(color)), Argb::from(color));
    }
}
