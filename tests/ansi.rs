#![cfg(test)]

use tui_markup_ansi_macro::ansi;

#[test]
fn test_ansi_macro() {
    // styled text generation at **compile-time**, notice the 'static lifetime
    let text: &'static str = ansi!("Press <blue Space> to <cyan Jump> over the <bg:yellow,i fox>");
    // It's is equal to:
    let generated = "Press \u{001b}[34mSpace\u{001b}[0m to \u{001b}[36mJump\u{001b}[0m over the \u{001b}[3;43mfox\u{001b}[0m";
    assert_eq!(text, generated);

    println!("{}", text);
}

#[test]
fn test_ansi_macro_with_custom_tag() {
    let text: &'static str = ansi!(
        "Press <kbd Space> to <action Jump> over the <enemy fox>",
        "kbd" => "blue",
        "action" => "cyan",
        "enemy" => "bg:yellow,i",
    );
    let generated = "Press \u{001b}[34mSpace\u{001b}[0m to \u{001b}[36mJump\u{001b}[0m over the \u{001b}[3;43mfox\u{001b}[0m";
    assert_eq!(text, generated);
    println!("{}", text);
}
