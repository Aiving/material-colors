use material_colors::argb_from_hex;
use material_colors::theme_from_source_color;

#[test]
fn test_theme() {
    let theme = theme_from_source_color(argb_from_hex("#AAE5A4"), vec![]);

    // then validate the colors in css yourself (compare with official library result) :jokerge:
    println!("{}", theme.schemes.dark);
}
