# MailCraft's Blitz

This branch is Blitz `v0.3.0-beta.4` with the fixes MailCraft's HTML mail reading needs, one
commit per fix. Each is reported upstream; a fix leaves this list once Blitz has it.

| Commit | Fix | Upstream |
|---|---|---|
| `8047b2041` | A table's `cellspacing` and `cellpadding` attributes are read (presentational hints) | on `main`: [#1366](https://github.com/DioxusLabs/blitz/pull/1366) |
| `039227343` | A resource whose URL can't be resolved is skipped instead of panicking | on `main`: [#1374](https://github.com/DioxusLabs/blitz/pull/1374) |
| `ebc52fe5e` | A style sheet that fails to load no longer blocks layout (`NetHandler::error`) | on `main`: [#1318](https://github.com/DioxusLabs/blitz/pull/1318) |
| `030d78f62` | A replaced element's set width or height stays beside a larger min size of the other axis (`min-width:300px; height:0` is 300×0) | reported: [#1433](https://github.com/DioxusLabs/blitz/issues/1433) |
| `7926eccb7` | A table cell's `height` is the least it may be: its content makes it taller | reported: [#1431](https://github.com/DioxusLabs/blitz/issues/1431) |
| `c6d01fd9b` | In automatic table layout, a percentage column gets its share of a table with a width (`minmax(min-content, N%)`) | reported: [#1435](https://github.com/DioxusLabs/blitz/issues/1435) |
| `1ca8594d7` | `HtmlDocument::from_html_without_sniffing`: HTML parsed as HTML whatever its DOCTYPE (an XHTML DOCTYPE made it XML) | not reported yet |
| `b60101fdb` | In fixed table layout, columns with a length width share out a wider table's extra width when no column is `auto` | not reported yet |

The first three are upstream's own fixes, backported to beta.4 (`git cherry-pick -x`); they
leave this list with the next Blitz release.

[#1432](https://github.com/DioxusLabs/blitz/issues/1432) (shrink-to-fit with a `width:100%` picture) is fixed in
Taffy (DioxusLabs/taffy#1401): MailCraft takes Taffy from its own branch of it, aknuth/taffy
`mailcraft`, until a Taffy release has the fix.

Gaps reported upstream that MailCraft still works around in its own crate (`crates/html`):
[#1430](https://github.com/DioxusLabs/blitz/issues/1430) anonymous table boxes,
[#1434](https://github.com/DioxusLabs/blitz/issues/1434) an inline element holding a block.
