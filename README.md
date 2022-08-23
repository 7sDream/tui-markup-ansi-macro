# tui markup ansi macro

[![crates.io][badge-crate-version]][crate]
[![changelog][badge-changelog]][changelog]
[![docs.rs][badge-docs-rs]][doc]
![deps state][badge-deps-state]

This create provided a macro to generated terminal styled text in [ANSI escape sequence][ansi-seq-wp] format using a markup language.

This macro compile your source at compile time, so it has no runtime overhead.

## Example

```rust
use tui_markup_ansi_macro::ansi;

let generated = ansi!("Press <blue Space> to <cyan Jump> over the <bg:yellow,i fox>");
let hardcode = "Press \u{001b}[34mSpace\u{001b}[0m to \u{001b}[36mJump\u{001b}[0m over the \u{001b}[3;43mfox\u{001b}[0m";
// Those two are equivalent.
assert_eq!(generated, hardcode);

// Use custom tag
let generated = ansi!(
    "Press <keyboard Space> to <action Jump> over the <enemy fox>", 
    "keyboard" => "blue",
    "action" => "cyan",
    "enemy" => "bg:yellow,i",
);
assert_eq!(generated, hardcode);

println!("{}", generated);
```

Output:

![example-output]

The markup language used here is my [tui markup language][tui-markup], See it's documentation for [full syntax][tui-markup-syntax] and [supported style tags][ansi-tags].

## LICENSE

BSD-3-Clause-Clear, see [LICENSE].

[badge-crate-version]: https://img.shields.io/crates/v/tui-markup-ansi-macro?style=for-the-badge
[badge-changelog]: https://img.shields.io/badge/-CHANGELOG-brightgreen?style=for-the-badge
[badge-docs-rs]: https://img.shields.io/docsrs/tui-markup-ansi-macro?style=for-the-badge
[badge-deps-state]: https://img.shields.io/librariesio/release/cargo/tui-markup-ansi-macro?style=for-the-badge
[crate]: https://crates.io/crates/tui-markup-ansi-macro
[doc]: https://docs.rs/tui-markup-ansi-macro/latest
[changelog]: https://github.com/7sDream/tui-markup-ansi-macro/blob/master/CHANGELOG.md

[ansi-seq-wp]: https://en.wikipedia.org/wiki/ANSI_escape_code
[tui-markup]: https://github.com/7sDream/tui-markup
[example-output]: https://rikka.7sdre.am/files/1ac47e60-6f10-4e5f-b4d2-262afaceecb7.png
[tui-markup-syntax]: https://github.com/7sDream/tui-markup#markup-syntax
[ansi-tags]: https://github.com/7sDream/tui-markup/blob/master/docs/ansi-tags.ebnf
[LICENSE]: https://github.com/7sDream/tui-markup-ansi-macro/blob/master/LICENSE
