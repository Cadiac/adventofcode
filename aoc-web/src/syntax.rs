use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxDefinition, SyntaxSetBuilder};
use yew_agent::prelude::*;

#[oneshot]
pub async fn SyntaxHighlightTask(source: String) -> String {
    let rust_syntax = include_str!("../static/syntax/rust.sublime-syntax");

    let mut builder = SyntaxSetBuilder::new();
    builder.add(SyntaxDefinition::load_from_str(rust_syntax, true, None).unwrap());

    let syntax_set = builder.build();
    let theme_set = ThemeSet::load_defaults();
    let theme = &theme_set.themes["base16-eighties.dark"];
    let syntax_reference = syntax_set.find_syntax_by_extension("rs").unwrap();

    highlighted_html_for_string(&source, &syntax_set, syntax_reference, theme).unwrap()
}
