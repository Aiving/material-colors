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
    use crate::{color::Rgb, dislike::fix_if_disliked, hct::Hct};

    #[test]
    fn test_monk_skin_tone_scale_colors() {
        // From https://skintone.google#/get-started
        let monk_skin_tone_scale_colors = [
            Rgb::from_u32(0xF6EDE4),
            Rgb::from_u32(0xF3E7DB),
            Rgb::from_u32(0xF7EAD0),
            Rgb::from_u32(0xEADABA),
            Rgb::from_u32(0xD7BD96),
            Rgb::from_u32(0xA07E56),
            Rgb::from_u32(0x825C43),
            Rgb::from_u32(0x604134),
            Rgb::from_u32(0x3A312A),
            Rgb::from_u32(0x292420),
        ];

        for color in monk_skin_tone_scale_colors {
            assert!(!is_disliked(&color.into()));
        }
    }

    #[test]
    fn test_bile_colors_disliked() {
        let unlikable = [
            Rgb::from_u32(0x95884B),
            Rgb::from_u32(0x716B40),
            Rgb::from_u32(0xB08E00),
            Rgb::from_u32(0x4C4308),
            Rgb::from_u32(0x464521),
        ];

        for color in unlikable {
            assert!(is_disliked(&color.into()));
        }
    }

    #[test]
    fn test_bile_colors_became_likable() {
        let unlikable = [
            Rgb::from_u32(0x95884B),
            Rgb::from_u32(0x716B40),
            Rgb::from_u32(0xB08E00),
            Rgb::from_u32(0x4C4308),
            Rgb::from_u32(0x464521),
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
        assert_eq!(Rgb::from(fix_if_disliked(color)), Rgb::from(color));
    }
}
