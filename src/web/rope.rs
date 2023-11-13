use std::{collections::HashSet, iter::FromIterator};

use itertools::Itertools;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, Window};
use yew::prelude::*;

use crate::{utils::Coords, y2022::day09::Day09};

pub enum Msg {
    KeyPress(char),
}

pub struct Rope {
    keyboard_listener: Option<Closure<dyn Fn(KeyboardEvent)>>,
    rope: Vec<Coords<i32>>,
    visited: HashSet<Coords<i32>>,
}

const ALLOWED_KEYS: [char; 4] = ['W', 'A', 'S', 'D'];

impl Component for Rope {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            keyboard_listener: None,
            rope: vec![Coords { x: 0, y: 0 }; 10],
            visited: HashSet::from_iter(vec![Coords { x: 0, y: 0 }]),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }

        let window: Window = window().expect("window not available");

        let cb = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key().chars().count() == 1 {
                match e.key().to_uppercase().chars().next() {
                    Some(key) => {
                        if ALLOWED_KEYS.contains(&key)
                            && !e.ctrl_key()
                            && !e.alt_key()
                            && !e.meta_key()
                        {
                            e.prevent_default();
                            Some(Msg::KeyPress(key))
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            } else {
                None
            }
        });

        let listener =
            Closure::<dyn Fn(KeyboardEvent)>::wrap(Box::new(move |e: KeyboardEvent| cb.emit(e)));

        window
            .add_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
            .unwrap();
        self.keyboard_listener = Some(listener);
    }

    fn destroy(&mut self, _: &Context<Self>) {
        // Remove the keyboard listener
        if let Some(listener) = self.keyboard_listener.take() {
            let window: Window = window().expect("window not available");
            window
                .remove_event_listener_with_callback("keydown", listener.as_ref().unchecked_ref())
                .unwrap();
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyPress(c) => {
                let direction = match c {
                    'W' => Coords { x: 0, y: -1 },
                    'A' => Coords { x: -1, y: 0 },
                    'S' => Coords { x: 0, y: 1 },
                    'D' => Coords { x: 1, y: 0 },
                    _ => unreachable!(),
                };

                let (visited, rope) = Day09::simulate(
                    vec![(direction, 1)],
                    std::mem::take(&mut self.rope),
                    std::mem::take(&mut self.visited),
                );
                self.rope = rope;
                self.visited = visited;
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let output = (-10..10)
            .map(|y| {
                (-15..15)
                    .map(|x| {
                        if let Some(knot) = self
                            .rope
                            .iter()
                            .position(|coords| coords.x == x && coords.y == y)
                        {
                            if knot == 0 {
                                "H".to_string()
                            } else if knot == self.rope.len() - 1 {
                                "T".to_string()
                            } else {
                                format!("{knot}")
                            }
                        } else if self.visited.contains(&Coords { x, y }) {
                            "#".to_string()
                        } else {
                            ".".to_string()
                        }
                    })
                    .collect::<String>()
            })
            .join("\n");

        let visited = format!("Positions tail visited: {}", self.visited.len());

        let link = ctx.link();

        html! {
            <>
                <h2>{"-- Day 9 --"}</h2>
                <p>
                    {"Move around with "}
                    <a class="link" role="button" href={"javascript:void(0)"} onclick={link.callback(|_| Msg::KeyPress('W'))}>{"W"}</a>
                    {", "}
                    <a class="link" role="button" href={"javascript:void(0)"} onclick={link.callback(|_| Msg::KeyPress('A'))}>{"A"}</a>
                    {", "}
                    <a class="link" role="button" href={"javascript:void(0)"} onclick={link.callback(|_| Msg::KeyPress('S'))}>{"S"}</a>
                    {" and "}
                    <a class="link" role="button" href={"javascript:void(0)"} onclick={link.callback(|_| Msg::KeyPress('D'))}>{"D"}</a>
                    {"."}</p>
                <pre>
                    <code>{ output }</code>
                </pre>
                <p class="success">{ visited }</p>
            </>
        }
    }
}
