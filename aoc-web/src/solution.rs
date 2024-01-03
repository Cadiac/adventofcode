use yew::platform::spawn_local;
use yew::prelude::*;
use yew_agent::oneshot::use_oneshot_runner;

use aoc_solver::{solution::Solver, y2020::Y2020, y2021::Y2021, y2022::Y2022, y2023::Y2023};

use crate::runner::SolutionTask;
use crate::syntax::SyntaxHighlightTask;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub day: u8,
    pub year: u32,
}

#[function_component(SourceViewer)]
pub fn source_viewer(props: &Props) -> Html {
    let highlighted = use_state(|| "".to_string());

    let syntax_task = use_oneshot_runner::<SyntaxHighlightTask>();

    let run_syntax_highlight = {
        let highlighted = highlighted.clone();

        move |source: String| {
            let syntax_agent = syntax_task.clone();
            let highlighted = highlighted.clone();
            highlighted.set("".to_string());

            spawn_local(async move {
                let output_value = syntax_agent.run(source).await;
                highlighted.set(output_value);
            });
        }
    };

    let year = props.year;
    let day = props.day;

    use_effect_with((props.day, props.year), move |_| {
        let source = match year {
            2020 => Y2020::get_source(day),
            2021 => Y2021::get_source(day),
            2022 => Y2022::get_source(day),
            2023 => Y2023::get_source(day),
            _ => "",
        };

        run_syntax_highlight(source.to_owned())
    });

    let div = gloo::utils::document().create_element("div").unwrap();
    div.set_class_name("highlighted fade-in");
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
