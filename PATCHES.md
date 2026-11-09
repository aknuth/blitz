# MailCraft's Blitz

This branch is Blitz `v0.3.0-beta.4` with the fixes MailCraft's HTML mail reading needs, one
commit per fix. Each is reported upstream; a fix leaves this list once Blitz has it.

| Commit | Fix | Upstream |
|---|---|---|

None yet.

Gaps reported upstream that MailCraft still works around in its own crate (`crates/html`):
[#1430](https://github.com/DioxusLabs/blitz/issues/1430) anonymous table boxes,
[#1431](https://github.com/DioxusLabs/blitz/issues/1431) a cell's height as a minimum,
[#1432](https://github.com/DioxusLabs/blitz/issues/1432) shrink-to-fit with a `width:100%` picture,
[#1433](https://github.com/DioxusLabs/blitz/issues/1433) `min-width` beside `height:0`,
[#1434](https://github.com/DioxusLabs/blitz/issues/1434) an inline element holding a block,
[#1435](https://github.com/DioxusLabs/blitz/issues/1435) percentage cells.
