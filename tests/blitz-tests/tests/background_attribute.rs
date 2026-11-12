//! The `background` attribute of `body` and the table elements is a presentational hint for
//! `background-image`, its URL resolved against the document's; on other elements it's nothing.
//! https://html.spec.whatwg.org/multipage/rendering.html#tables-2
//!
//! Regression test: eBay's mail draws its two round badges as `<td background="...gif">`; they
//! weren't drawn at all.

use blitz_test_harness::Harness;

/// The element's computed `background-image`, as Debug text, or `None` for `none`.
fn background_image(html: &str) -> Option<String> {
    let harness = Harness::from_html(html);
    let id = harness.node("#x");
    let doc = harness.base();
    let styles = doc.get_node(id).unwrap().primary_styles().unwrap();
    let layers = &styles.get_background().background_image.0;
    let text = format!("{:?}", layers);
    (text != "[none]").then_some(text)
}

#[test]
fn a_cell_takes_its_background_attribute() {
    let got = background_image(
        r#"<table><tr><td id="x" background="https://pics.example/badge.gif">x</td></tr></table>"#,
    );
    assert!(got.is_some_and(|t| t.contains("https://pics.example/badge.gif")));
}

#[test]
fn table_row_and_body_take_it_too() {
    for html in [
        r#"<table id="x" background="https://pics.example/a.png"><tr><td>x</td></tr></table>"#,
        r#"<table><tr id="x" background="https://pics.example/a.png"><td>x</td></tr></table>"#,
        r#"<body id="x" background="https://pics.example/a.png">x</body>"#,
    ] {
        assert!(
            background_image(html).is_some_and(|t| t.contains("pics.example/a.png")),
            "{html}"
        );
    }
}

#[test]
fn other_elements_and_empty_values_take_nothing() {
    assert_eq!(
        background_image(r#"<div id="x" background="https://pics.example/a.png">x</div>"#),
        None
    );
    assert_eq!(
        background_image(r#"<table><tr><td id="x" background="  ">x</td></tr></table>"#),
        None
    );
}

#[test]
fn a_style_sheet_overrides_it() {
    let got = background_image(
        r#"<style>td { background-image: none }</style>
           <table><tr><td id="x" background="https://pics.example/a.png">x</td></tr></table>"#,
    );
    assert_eq!(got, None);
}

#[test]
fn a_quote_in_the_url_ends_nothing_early() {
    let got = background_image(
        r#"<table><tr><td id="x" background='https://pics.example/a".png'>x</td></tr></table>"#,
    );
    assert!(got.is_some_and(|t| t.contains("pics.example/a%22.png") || t.contains(r#"a\".png"#)));
}
