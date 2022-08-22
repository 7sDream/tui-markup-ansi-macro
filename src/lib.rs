extern crate proc_macro;

use std::collections::HashMap;

use ansi_term::{ANSIStrings, Style};
use tui_markup::{
    compile_with,
    generator::{
        ansi::{ANSIStringsGenerator, ANSITermTagConvertor},
        helper::{CustomTagParser, FlattenableStyle},
        TagConvertor,
    },
};

use litrs::Literal;
use proc_macro::{TokenStream, TokenTree};
use proc_macro_error::{abort, proc_macro_error};

#[derive(Default)]
struct AnsiMacroCustomTags(HashMap<String, Style>);

impl AnsiMacroCustomTags {
    pub fn insert(&mut self, tag: String, style: Style) {
        self.0.insert(tag, style);
    }
}

impl CustomTagParser for AnsiMacroCustomTags {
    type Output = Style;

    fn parse(&mut self, s: &str) -> Option<Self::Output> {
        self.0.get(s).copied()
    }
}

#[proc_macro_error]
#[proc_macro]
pub fn ansi(ts: TokenStream) -> TokenStream {
    let mut tsi = ts.into_iter();

    let tt = match tsi.next() {
        Some(tt) => tt,
        None => panic!("no markup source provided"),
    };

    let mut span = tt.span();

    let literal = match litrs::Literal::try_from(tt) {
        Ok(Literal::String(s)) => s,
        Ok(_) => {
            abort!(span, "except a string literal");
        }
        Err(e) => return e.to_compile_error(),
    };

    let mut tag_convertor = ANSITermTagConvertor::<AnsiMacroCustomTags>::default();

    let mut custom_tags = AnsiMacroCustomTags::default();

    loop {
        // ,
        match tsi.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => (),
            Some(_) => abort!(span, "expect comma after here"),
            None => break,
        };

        let tag = match tsi.next() {
            Some(tt) => tt,
            None => break,
        };
        span = tag.span();

        let tag_lit = match litrs::Literal::try_from(tag) {
            Ok(Literal::String(s)) => s,
            Ok(_) => {
                abort!(span, "expect a string literal for custom tag");
            }
            Err(e) => return e.to_compile_error(),
        };

        // =
        let next = tsi.next();
        match &next {
            Some(TokenTree::Punct(p)) if p.as_char() == '=' => (),
            _ => abort!(span, "expect arrow after custom tag"),
        };

        // >
        let next = tsi.next();
        let gt = match &next {
            Some(TokenTree::Punct(p)) if p.as_char() == '>' => next.unwrap(),
            _ => abort!(span, "expect arrow after custom tag"),
        };

        span = gt.span();

        let style = match tsi.next() {
            Some(tt) => tt,
            None => abort!(span, "expect style after arrow"),
        };
        let style_lit = match litrs::Literal::try_from(style) {
            Ok(Literal::String(s)) => s,
            Ok(_) => {
                abort!(span, "except a string literal for style");
            }
            Err(e) => return e.to_compile_error(),
        };

        let style_tags = style_lit.value().split(',').collect::<Vec<_>>();
        let final_style = style_tags.into_iter().fold(Style::default(), |style, s| {
            if let Some(tag_style) = tag_convertor.convert_tag(s) {
                style.patch(tag_style.into())
            } else {
                style
            }
        });

        custom_tags.insert(tag_lit.value().to_string(), final_style);
    }

    let gen = ANSIStringsGenerator::new(custom_tags);

    let result = match compile_with(literal.value(), gen) {
        Ok(result) => result,
        Err(e) => abort!(span, e.to_string()),
    };

    let output = ANSIStrings(&result).to_string();

    TokenStream::from(TokenTree::Literal(proc_macro::Literal::string(&output)))
}
