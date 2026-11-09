# MailCraft's Blitz

This branch is Blitz `v0.3.0-beta.4` with the fixes MailCraft's HTML mail reading needs, one
commit per fix. Each is reported upstream; a fix leaves this list once Blitz has it.

| Commit | Fix | Upstream |
|---|---|---|
| `8047b2041` | A table's `cellspacing` and `cellpadding` attributes are read (presentational hints) | on `main`: [#1366](https://github.com/DioxusLabs/blitz/pull/1366) |
| `039227343` | A resource whose URL can't be resolved is skipped instead of panicking | on `main`: [#1374](https://github.com/DioxusLabs/blitz/pull/1374) |
| `ebc52fe5e` | A style sheet that fails to load no longer blocks layout (`NetHandler::error`) | on `main`: [#1318](https://github.com/DioxusLabs/blitz/pull/1318) |

The three above are upstream's own fixes, backported to beta.4 (`git cherry-pick -x`); they
leave this list with the next Blitz release.

Gaps reported upstream that MailCraft still works around in its own crate (`crates/html`):
[#1430](https://github.com/DioxusLabs/blitz/issues/1430) anonymous table boxes,
[#1431](https://github.com/DioxusLabs/blitz/issues/1431) a cell's height as a minimum,
[#1432](https://github.com/DioxusLabs/blitz/issues/1432) shrink-to-fit with a `width:100%` picture,
[#1433](https://github.com/DioxusLabs/blitz/issues/1433) `min-width` beside `height:0`,
[#1434](https://github.com/DioxusLabs/blitz/issues/1434) an inline element holding a block,
[#1435](https://github.com/DioxusLabs/blitz/issues/1435) percentage cells.
