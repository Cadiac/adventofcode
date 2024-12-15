use itertools::Itertools;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, Window};
use yew::prelude::*;

use aoc_solver::y2024::day15::{move_robot, parse, Coords, Grid, Tile};

pub enum Msg {
    KeyPress(char),
}

pub struct WarehouseRobot {
    keyboard_listener: Option<Closure<dyn Fn(KeyboardEvent)>>,
    grid: Grid,
    robot: Coords,
}

const ALLOWED_KEYS: [char; 4] = ['W', 'A', 'S', 'D'];

impl Component for WarehouseRobot {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let input = "##########\n\
                 #..O..O.O#\n\
                 #......O.#\n\
                 #.OO..O.O#\n\
                 #..O@..O.#\n\
                 #O#..O...#\n\
                 #O..O..O.#\n\
                 #.OO.O.OO#\n\
                 #....O...#\n\
                 ##########\n\
                 \n\
                 v";

        let (grid, _, robot) = parse(input, true).unwrap();

        Self {
            keyboard_listener: None,
            grid,
            robot,
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
                    'W' => (0, -1),
                    'A' => (-1, 0),
                    'S' => (0, 1),
                    'D' => (1, 0),
                    _ => unreachable!(),
                };

                move_robot(&mut self.robot, direction, &mut self.grid);
            }
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let output = (0..10)
            .map(|y| {
                (0..20)
                    .map(|x| {
                        if (x, y) == self.robot {
                            "@".to_string()
                        } else {
                            match self.grid.get(&(x, y)) {
                                Some(Tile::Wall) => "#".to_string(),
                                Some(Tile::LargeBox(linked)) if linked.0 > x => "[".to_string(),
                                Some(Tile::LargeBox(_)) => "]".to_string(),
                                _ => ".".to_string(),
                            }
                        }
                    })
                    .collect::<String>()
            })
            .join("\n");

        let link = ctx.link();

        html! {
            <>
                <h2>{"-- Day 15 --"}</h2>
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
            </>
        }
    }
}
