//! In the automatic table layout algorithm, a table with a width wider than its columns shares
//! the extra width out as in the fixed one: over the `auto` columns when there are any, and over
//! columns with a length width, in proportion to it, when there are none.
//!
//! Regression test: Intuit's mail has a `width="500"` table whose one column is `width="50"`
//! (an icon above the text): the column stayed 50px wide and the text below ran a word a line.

use blitz_dom::DocumentConfig;
use blitz_html::{HtmlDocument, HtmlProvider};
use blitz_traits::shell::{ColorScheme, Viewport};
use std::sync::Arc;

/// The widths of the cells with ids `a`, `b`, ... in an automatic 300px wide table with the
/// given row.
fn widths(row: &str, ids: &[&str]) -> Vec<f32> {
    let mut doc = HtmlDocument::from_html(
        &format!(
            r#"<body style="margin:0"><table style="width:300px;border-spacing:0"><tr>{row}</tr></table></body>"#
        ),
        DocumentConfig {
            viewport: Some(Viewport::new(800, 600, 1.0, ColorScheme::Light)),
            html_parser_provider: Some(Arc::new(HtmlProvider) as _),
            ..Default::default()
        },
    );
    doc.resolve(0.0);
    ids.iter()
        .map(|id| {
            let node = doc.query_selector(&format!("#{id}")).unwrap().expect(id);
            doc.get_node(node).unwrap().final_layout().size.width
        })
        .collect()
}

#[test]
fn one_column_with_a_width_takes_the_whole_table() {
    let w = widths(
        r#"<td id="a" width="50" style="padding:0">some text</td>"#,
        &["a"],
    );
    assert_eq!(w, [300.0]);
}

#[test]
fn columns_with_widths_share_the_extra_in_proportion() {
    let w = widths(
        r#"<td id="a" style="width:100px;padding:0"></td><td id="b" style="width:50px;padding:0"></td>"#,
        &["a", "b"],
    );
    assert_eq!(w, [200.0, 100.0]);
}

#[test]
fn an_auto_column_takes_the_extra_alone() {
    let w = widths(
        r#"<td id="a" width="100" style="padding:0"></td><td id="b" style="padding:0"></td>"#,
        &["a", "b"],
    );
    assert_eq!(w, [100.0, 200.0]);
}
