use yew::platform::spawn_local;
use yew::prelude::*;
use yew_agent::oneshot::use_oneshot_runner;

use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxDefinition, SyntaxSetBuilder};

use crate::agent::SolutionTask;
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

        let rust_syntax = include_str!("../static/syntax/rust.sublime-syntax");

        let mut builder = SyntaxSetBuilder::new();
        builder.add(SyntaxDefinition::load_from_str(rust_syntax, true, None).unwrap());

        let syntax_set = builder.build();
        let theme_set = ThemeSet::load_defaults();
        let theme = &theme_set.themes["base16-eighties.dark"];
        let syntax_reference = syntax_set.find_syntax_by_extension("rs").unwrap();

        highlighted_html_for_string(source, &syntax_set, syntax_reference, theme).unwrap()
    });

    let div = gloo::utils::document().create_element("div").unwrap();
    div.set_class_name("highlighted");
    div.set_inner_html(highlighted.as_str());

    Html::VRef(div.into())
}

#[function_component(Solution)]
pub fn solution(props: &Props) -> Html {
    let output = use_state(|| "".to_string());
    let solution_task = use_oneshot_runner::<SolutionTask>();

    let run_solution = {
        let output = output.clone();

        move |input: (u32, u8)| {
            let solution_agent = solution_task.clone();
            let output = output.clone();
            output.set("Running...".to_string());

            spawn_local(async move {
                let (output_value, duration) = solution_agent.run(input).await;
                output.set(format!("{output_value}\n{duration} ms"));
            });
        }
    };

    let input = (props.year, props.day);

    use_effect_with((props.day, props.year), move |_| run_solution(input));

    html! {
        <div class="fade-in">
            <pre>
                <code>{ &*output }</code>
            </pre>
            <SourceViewer year={props.year} day={props.day} />
        </div>
    }
}
