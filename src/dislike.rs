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
    use crate::color::Argb;
    use crate::dislike::fix_if_disliked;
    use crate::hct::Hct;

    use super::is_disliked;

    #[test]
    fn test_monk_skin_tone_scale_colors() {
        // From https://skintone.google#/get-started
        let monk_skin_tone_scale_colors = [
            Argb::from_u32(0xfff6ede4),
            Argb::from_u32(0xfff3e7db),
            Argb::from_u32(0xfff7ead0),
            Argb::from_u32(0xffeadaba),
            Argb::from_u32(0xffd7bd96),
            Argb::from_u32(0xffa07e56),
            Argb::from_u32(0xff825c43),
            Argb::from_u32(0xff604134),
            Argb::from_u32(0xff3a312a),
            Argb::from_u32(0xff292420),
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
