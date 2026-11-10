//! `bgcolor` is read with the HTML "rules for parsing a legacy colour value": colour names, and
//! `#rgb` with each digit doubled, as well as `#rrggbb`.
//!
//! Regression test: Edmunds' mail paints its content white with `bgcolor="white"` inside a
//! `bgcolor="#f0f0f0"` table; the name was ignored and the content stayed grey.

use blitz_test_harness::Harness;
use style::color::{AbsoluteColor, ColorSpace};

fn background(bgcolor: &str) -> [f32; 4] {
    let harness = Harness::from_html(&format!(
        r#"<body><table id="t" bgcolor="{bgcolor}"><tr><td>x</td></tr></table></body>"#
    ));
    let id = harness.node("#t");
    let doc = harness.base();
    let styles = doc.get_node(id).unwrap().primary_styles().unwrap();
    let color = styles
        .clone_background_color()
        .resolve_to_absolute(&AbsoluteColor::BLACK);
    let c = color.to_color_space(ColorSpace::Srgb);
    let [r, g, b] = *c.raw_components().first_chunk::<3>().unwrap();
    [
        (r * 255.0).round(),
        (g * 255.0).round(),
        (b * 255.0).round(),
        c.alpha,
    ]
}

#[test]
fn a_colour_name_is_read() {
    assert_eq!(background("white"), [255.0, 255.0, 255.0, 1.0]);
    assert_eq!(background("Red"), [255.0, 0.0, 0.0, 1.0]);
}

#[test]
fn short_hex_doubles_each_digit() {
    assert_eq!(background("#abc"), [170.0, 187.0, 204.0, 1.0]);
}

#[test]
fn long_hex_and_hex_without_the_hash_are_read() {
    assert_eq!(background("#f0f0f0"), [240.0, 240.0, 240.0, 1.0]);
    assert_eq!(background("f0f0f0"), [240.0, 240.0, 240.0, 1.0]);
}

#[test]
fn transparent_and_empty_are_no_colour() {
    assert_eq!(background("transparent")[3], 0.0);
    assert_eq!(background("")[3], 0.0);
}
