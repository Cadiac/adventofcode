use yew::prelude::*;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use aoc_solver::solution::Solver;
use aoc_solver::y2020::Y2020;
use aoc_solver::y2021::Y2021;
use aoc_solver::y2022::Y2022;
use aoc_solver::y2023::Y2023;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub day: u8,
    pub year: u32,
}

use yew::{function_component, Html, Properties};

#[function_component(SourceViewer)]
pub fn source_viewer(props: &Props) -> Html {
    let highlighted = use_memo((props.year, props.day), |(year, day)| {
        let source = match year {
            2020 => Y2020::get_source(*day),
            2021 => Y2021::get_source(*day),
            2022 => Y2022::get_source(*day),
            2023 => Y2023::get_source(*day),
            _ => "",
        };

        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        let theme = &ts.themes["base16-eighties.dark"];
        let sr = ss.find_syntax_by_extension("rs").unwrap();

        highlighted_html_for_string(source, &ss, &sr, theme).unwrap()
    });

    let div = gloo::utils::document().create_element("div").unwrap();
    div.set_class_name("highlighted");
    div.set_inner_html(highlighted.as_str());

    Html::VRef(div.into())
}

#[function_component(Solution)]
pub fn solution(props: &Props) -> Html {
    let output = match props.year {
        2020 => Y2020::run_solution(props.day, None),
        2021 => Y2021::run_solution(props.day, None),
        2022 => Y2022::run_solution(props.day, None),
        2023 => Y2023::run_solution(props.day, None),
        _ => vec!["Unknown year".to_string()],
    };

    html! {
        <div>
            <pre>
                <code>{ output.join("\n") }</code>
            </pre>
            <SourceViewer year={props.year} day={props.day} />
        </div>
    }
}
