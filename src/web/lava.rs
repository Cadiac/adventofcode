use std::collections::HashMap;

use itertools::Itertools;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, Window};
use yew::prelude::*;

use crate::solution::{
    day18::{Day18, Label},
    Solution,
};

pub enum Msg {
    KeyPress(char),
}

pub struct Lava {
    keyboard_listener: Option<Closure<dyn Fn(KeyboardEvent)>>,
    cubes: HashMap<(i32, i32, i32), Label>,
    bounds: ((i32, i32), (i32, i32), (i32, i32)),
    z: i32,
}

const ALLOWED_KEYS: [char; 2] = ['W', 'S'];

impl Component for Lava {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let lava_cubes = Day18::parse(Day18.default_input()).unwrap();

        let (labeled, bounds) = Day18::label_cubes(lava_cubes);

        Self {
            keyboard_listener: None,
            cubes: labeled,
            bounds,
            z: 0,
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
                self.z = match c {
                    'W' => i32::min(self.z + 1, self.bounds.2 .1),
                    'S' => i32::max(self.z - 1, self.bounds.2 .0),
                    _ => unreachable!(),
                };
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let output = (self.bounds.1 .0..self.bounds.1 .1)
            .map(|y| {
                (self.bounds.0 .0..self.bounds.0 .1)
                    .map(|x| match self.cubes.get(&(x, y, self.z)) {
                        Some(Label::Exterior) => '.',
                        Some(Label::Lava) => '#',
                        Some(Label::Pocket) => 'x',
                        None => '?',
                    })
                    .collect::<String>()
            })
            .join("\n");

        let current_z = format!("Z: {}", self.z);

        let link = ctx.link();

        html! {
            <>
                <h2>{"-- Day 18 --"}</h2>
                <p>
                    {"Move slice UP and DOWN with "}
                    <a class="link" role="button" href={"javascript:void(0)"} onclick={link.callback(|_| Msg::KeyPress('W'))}>{"W"}</a>
                    {" and "}
                    <a class="link" role="button" href={"javascript:void(0)"} onclick={link.callback(|_| Msg::KeyPress('S'))}>{"S"}</a>
                    {"."}
                </p>
                <pre>
                    <code>{ output }</code>
                </pre>
                <p class="success">{ current_z }</p>
            </>
        }
    }
}
