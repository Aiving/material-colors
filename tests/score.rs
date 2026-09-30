#![cfg(feature = "quantize")]

use material_colors::{color::Rgb, score::Score};

type IndexMap<K, V> = indexmap::IndexMap<K, V, core::hash::BuildHasherDefault<ahash::AHasher>>;

#[test]
fn test_prioritizes_chroma() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([(Rgb::from_u32(0x000000), 1), (Rgb::from_u32(0xFFFFFF), 1), (Rgb::from_u32(0x0000FF), 1)]);

    let ranked = Score::score(&rgb_to_population, None, None, None);

    assert_eq!(ranked.len(), 1);
    assert_eq!(ranked[0], Rgb::from_u32(0x0000FF));
}

#[test]
fn test_prioritizes_chroma_when_proportions_equal() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([(Rgb::from_u32(0xFF0000), 1), (Rgb::from_u32(0x00FF00), 1), (Rgb::from_u32(0x0000FF), 1)]);

    let ranked = Score::score(&rgb_to_population, None, None, None);

    assert_eq!(ranked.len(), 3);
    assert_eq!(ranked[0], Rgb::from_u32(0xFF0000));
    assert_eq!(ranked[1], Rgb::from_u32(0x00FF00));
    assert_eq!(ranked[2], Rgb::from_u32(0x0000FF));
}

#[test]
fn test_generates_gblue_when_no_colors_available() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([(Rgb::from_u32(0x000000), 1)]);

    let ranked = Score::score(&rgb_to_population, None, None, None);

    assert_eq!(ranked.len(), 1);
    assert_eq!(ranked[0], Rgb::from_u32(0x4285F4));
}

#[test]
fn test_dedupes_nearby_hues() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([(Rgb::from_u32(0x008772), 1), (Rgb::from_u32(0x318477), 1)]);

    let ranked = Score::score(&rgb_to_population, None, None, None);

    assert_eq!(ranked.len(), 1);
    assert_eq!(ranked[0], Rgb::from_u32(0x008772));
}

#[test]
fn test_maximizes_hue_distance() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([(Rgb::from_u32(0x008772), 1), (Rgb::from_u32(0x008587), 1), (Rgb::from_u32(0x007EBC), 1)]);

    let ranked = Score::score(&rgb_to_population, Some(2), None, None);

    assert_eq!(ranked.len(), 2);
    assert_eq!(ranked[0], Rgb::from_u32(0x007EBC));
    assert_eq!(ranked[1], Rgb::from_u32(0x008772));
}

#[test]
fn test_generated_scenario_one() {
    let rgb_to_population: IndexMap<Rgb, u32> =
        IndexMap::from_iter([(Rgb::from_u32(0x7EA16D), 67), (Rgb::from_u32(0xD8CCAE), 67), (Rgb::from_u32(0x835C0D), 49)]);

    let ranked = Score::score(&rgb_to_population, Some(3), Some(Rgb::from_u32(0x8D3819)), Some(false));

    assert_eq!(ranked.len(), 3);
    assert_eq!(ranked[0], Rgb::from_u32(0x7EA16D));
    assert_eq!(ranked[1], Rgb::from_u32(0xD8CCAE));
    assert_eq!(ranked[2], Rgb::from_u32(0x835C0D));
}

#[test]
fn test_generated_scenario_two() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
        (Rgb::from_u32(0xD33881), 14),
        (Rgb::from_u32(0x3205CC), 77),
        (Rgb::from_u32(0x0B48CF), 36),
        (Rgb::from_u32(0xA08F5D), 81),
    ]);

    let ranked = Score::score(&rgb_to_population, None, Some(Rgb::from_u32(0x7D772B)), None);

    assert_eq!(ranked.len(), 3);
    assert_eq!(ranked[0], Rgb::from_u32(0x3205CC));
    assert_eq!(ranked[1], Rgb::from_u32(0xA08F5D));
    assert_eq!(ranked[2], Rgb::from_u32(0xD33881));
}

#[test]
fn test_generated_scenario_three() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
        (Rgb::from_u32(0xBE94A6), 23),
        (Rgb::from_u32(0xC33FD7), 42),
        (Rgb::from_u32(0x899F36), 90),
        (Rgb::from_u32(0x94C574), 82),
    ]);

    let ranked = Score::score(&rgb_to_population, Some(3), Some(Rgb::from_u32(0xAA79A4)), None);

    assert_eq!(ranked.len(), 3);
    assert_eq!(ranked[0], Rgb::from_u32(0x94C574));
    assert_eq!(ranked[1], Rgb::from_u32(0xC33FD7));
    assert_eq!(ranked[2], Rgb::from_u32(0xBE94A6));
}

#[test]
fn test_generated_scenario_four() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
        (Rgb::from_u32(0xDF241C), 85),
        (Rgb::from_u32(0x685859), 44),
        (Rgb::from_u32(0xD06D5F), 34),
        (Rgb::from_u32(0x561C54), 27),
        (Rgb::from_u32(0x713090), 88),
    ]);

    let ranked = Score::score(&rgb_to_population, Some(5), Some(Rgb::from_u32(0x58C19C)), Some(false));

    assert_eq!(ranked.len(), 2);
    assert_eq!(ranked[0], Rgb::from_u32(0xDF241C));
    assert_eq!(ranked[1], Rgb::from_u32(0x561C54));
}

#[test]
fn test_generated_scenario_five() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
        (Rgb::from_u32(0xBE66F8), 41),
        (Rgb::from_u32(0x4BBDA9), 88),
        (Rgb::from_u32(0x80F6F9), 44),
        (Rgb::from_u32(0xAB8017), 43),
        (Rgb::from_u32(0xE89307), 65),
    ]);

    let ranked = Score::score(&rgb_to_population, Some(3), Some(Rgb::from_u32(0x916691)), Some(false));

    assert_eq!(ranked.len(), 3);
    assert_eq!(ranked[0], Rgb::from_u32(0xAB8017));
    assert_eq!(ranked[1], Rgb::from_u32(0x4BBDA9));
    assert_eq!(ranked[2], Rgb::from_u32(0xBE66F8));
}

#[test]
fn test_generated_scenario_seven() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
        (Rgb::from_u32(0x2E05ED), 23),
        (Rgb::from_u32(0x153E55), 90),
        (Rgb::from_u32(0x9AB220), 23),
        (Rgb::from_u32(0x153379), 66),
        (Rgb::from_u32(0x68BCC3), 81),
    ]);

    let ranked = Score::score(&rgb_to_population, Some(2), Some(Rgb::from_u32(0xF588DC)), None);

    assert_eq!(ranked.len(), 2);
    assert_eq!(ranked[0], Rgb::from_u32(0x2E05ED));
    assert_eq!(ranked[1], Rgb::from_u32(0x9AB220));
}

#[test]
fn test_generated_scenario_six() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
        (Rgb::from_u32(0x18EA8F), 93),
        (Rgb::from_u32(0x327593), 18),
        (Rgb::from_u32(0x066A18), 74),
        (Rgb::from_u32(0xFA8A23), 62),
        (Rgb::from_u32(0x04CA1F), 65),
    ]);

    let ranked = Score::score(&rgb_to_population, Some(2), Some(Rgb::from_u32(0x4C377A)), Some(false));

    assert_eq!(ranked.len(), 2);
    assert_eq!(ranked[0], Rgb::from_u32(0x18EA8F));
    assert_eq!(ranked[1], Rgb::from_u32(0xFA8A23));
}

#[test]
fn test_generated_scenario_eight() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
        (Rgb::from_u32(0x816EC5), 24),
        (Rgb::from_u32(0x6DCB94), 19),
        (Rgb::from_u32(0x3CAE91), 98),
        (Rgb::from_u32(0x5B542F), 25),
    ]);

    let ranked = Score::score(&rgb_to_population, Some(1), Some(Rgb::from_u32(0x84B0FD)), Some(false));

    assert_eq!(ranked.len(), 1);
    assert_eq!(ranked[0], Rgb::from_u32(0x3CAE91));
}

#[test]
fn test_generated_scenario_nine() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
        (Rgb::from_u32(0x206F86), 52),
        (Rgb::from_u32(0x4A620D), 96),
        (Rgb::from_u32(0xF51401), 85),
        (Rgb::from_u32(0x2B8EBF), 3),
        (Rgb::from_u32(0x277766), 59),
    ]);

    let ranked = Score::score(&rgb_to_population, Some(3), Some(Rgb::from_u32(0x02B415)), None);

    assert_eq!(ranked.len(), 3);
    assert_eq!(ranked[0], Rgb::from_u32(0xF51401));
    assert_eq!(ranked[1], Rgb::from_u32(0x4A620D));
    assert_eq!(ranked[2], Rgb::from_u32(0x2B8EBF));
}

#[test]
fn test_generated_scenario_ten() {
    let rgb_to_population: IndexMap<Rgb, u32> = IndexMap::from_iter([
        (Rgb::from_u32(0x8B1D99), 54),
        (Rgb::from_u32(0x27EFFE), 43),
        (Rgb::from_u32(0x6F558D), 2),
        (Rgb::from_u32(0x77FDF2), 78),
    ]);

    let ranked = Score::score(&rgb_to_population, None, Some(Rgb::from_u32(0x5E7A10)), None);

    assert_eq!(ranked.len(), 3);
    assert_eq!(ranked[0], Rgb::from_u32(0x27EFFE));
    assert_eq!(ranked[1], Rgb::from_u32(0x8B1D99));
    assert_eq!(ranked[2], Rgb::from_u32(0x6F558D));
}
