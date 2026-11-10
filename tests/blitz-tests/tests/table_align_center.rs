//! A table's `align="center"` centres the table (auto inline margins), and doesn't centre the
//! text in it; `left` and `right` float it (HTML rendering §15.3.8, tables).
//!
//! Regression test: Edmunds' mail puts its 768px content in a `<table width="768"
//! align="center">`; it sat at the left of a wider window, as in no browser.

use blitz_test_harness::Harness;
use style::values::specified::TextAlignKeyword;

fn page(table: &str) -> Harness {
    let mut harness = Harness::from_html(&format!(
        r#"<body style="margin:0">{table}<tr><td id="cell">text</td></tr></table></body>"#
    ));
    harness.set_viewport_size(800, 600);
    harness.pump();
    harness
}

fn table_x(harness: &Harness) -> f32 {
    let id = harness.node("table");
    harness
        .base()
        .get_node(id)
        .unwrap()
        .final_layout()
        .location
        .x
}

#[test]
fn a_centred_table_is_in_the_middle() {
    let harness = page(r#"<table width="400" align="center">"#);
    assert_eq!(table_x(&harness), 200.0);
}

#[test]
fn the_attribute_is_matched_without_case() {
    let harness = page(r#"<table width="400" align="CENTER">"#);
    assert_eq!(table_x(&harness), 200.0);
}

#[test]
fn a_centred_tables_text_isnt_centred() {
    let harness = page(r#"<table width="400" align="center">"#);
    let id = harness.node("#cell");
    let doc = harness.base();
    let styles = doc.get_node(id).unwrap().primary_styles().unwrap();
    assert_ne!(styles.clone_text_align(), TextAlignKeyword::MozCenter);
}

#[test]
fn a_table_without_the_attribute_stays_at_the_start() {
    let harness = page(r#"<table width="400">"#);
    assert_eq!(table_x(&harness), 0.0);
}
