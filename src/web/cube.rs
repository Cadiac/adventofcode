use std::collections::HashMap;

use yew::prelude::*;

use crate::solution::{
    day22::{Coords, Day22, Direction, Facet, Tile},
    Solution,
};

pub struct Cube {
    facets: HashMap<Coords, Facet>,
    size: i64,
    facet: Coords,
    position: Coords,
}

pub enum Msg {}

impl Component for Cube {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let (world, path, rows, _) = Day22::parse(Day22.default_input()).unwrap();

        let size = rows.values().map(|(min, max)| max - min + 1).min().unwrap();

        let mut facets: HashMap<Coords, Facet> = HashMap::new();
        let leftmost_open: Coords = Day22::find_leftmost_open(&world, &rows).unwrap();
        let starting_facet = Day22::find_cube_facets(size, world, &mut facets, leftmost_open);

        Day22::fold_cube(&mut facets);

        let (facet, position, _) =
            Day22::follow_path(starting_facet, path, &facets, size).unwrap();

        Self {
            facets,
            size,
            facet,
            position,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {}

    fn destroy(&mut self, _: &Context<Self>) {}

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        // Treat the current facet as the front.
        let front_facet = self.facets.get(&self.facet).unwrap();
        let (bottom_coords, _) = front_facet.neighbours.get(&Direction::Down).unwrap();
        let (right_coords, _) = front_facet.neighbours.get(&Direction::Right).unwrap();
        let (left_coords, _) = front_facet.neighbours.get(&Direction::Left).unwrap();
        let (top_coords, top_dir) = front_facet.neighbours.get(&Direction::Up).unwrap();

        // TODO: Draw the path taken

        // TODO: wrap this in some function

        let front = (0..self.size)
            .map(|y| {
                (0..self.size)
                    .map(|x| match front_facet.tiles.get(&(x, y)) {
                        Some((Tile::Open, _)) => {
                            if self.position.0 == x && self.position.1 == y {
                                '■'
                            } else {
                                '.'
                            }
                        }
                        Some((Tile::Solid, _)) => '#',
                        None => '?',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        let bottom_facet = self.facets.get(bottom_coords).unwrap();
        let bottom = (0..self.size)
            .map(|y| {
                (0..self.size)
                    .map(|x| match bottom_facet.tiles.get(&(x, y)) {
                        Some((Tile::Open, _)) => '.',
                        Some((Tile::Solid, _)) => '#',
                        None => '?',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        let right_facet = self.facets.get(right_coords).unwrap();
        let right = (0..self.size)
            .map(|y| {
                (0..self.size)
                    .map(|x| match right_facet.tiles.get(&(x, y)) {
                        Some((Tile::Open, _)) => '.',
                        Some((Tile::Solid, _)) => '#',
                        None => '?',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        let left_facet = self.facets.get(left_coords).unwrap();
        let left = (0..self.size)
            .map(|y| {
                (0..self.size)
                    .map(|x| match left_facet.tiles.get(&(x, y)) {
                        Some((Tile::Open, _)) => '.',
                        Some((Tile::Solid, _)) => '#',
                        None => '?',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        let top_facet = self.facets.get(top_coords).unwrap();
        let top = (0..self.size)
            .map(|y| {
                (0..self.size)
                    .map(|x| match top_facet.tiles.get(&(x, y)) {
                        Some((Tile::Open, _)) => '.',
                        Some((Tile::Solid, _)) => '#',
                        None => '?',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        let (back_coords, _) = top_facet.neighbours.get(&top_dir.reverse()).unwrap();
        let back_facet = self.facets.get(back_coords).unwrap();
        let back = (0..self.size)
            .map(|y| {
                (0..self.size)
                    .map(|x| match back_facet.tiles.get(&(x, y)) {
                        Some((Tile::Open, _)) => '.',
                        Some((Tile::Solid, _)) => '#',
                        None => '?',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        // TODO: Rotate these based on directions

        html! {
            <>
                <h2>{"-- Day 22 --"}</h2>
                <div class="scene">
                    <div class="cube">
                        <pre class="cube__face cube__face--front">{ front.clone() }</pre>
                        <pre class="cube__face cube__face--back">{ back.clone() }</pre>
                        <pre class="cube__face cube__face--right">{ right.clone() }</pre>
                        <pre class="cube__face cube__face--left">{ left.clone() }</pre>
                        <pre class="cube__face cube__face--top">{ top.clone() }</pre>
                        <pre class="cube__face cube__face--bottom">{ bottom.clone() }</pre>
                    </div>
                </div>
                <p class="success">{ format!("End position: x={}, y={}", self.position.0, self.position.1) }</p>
            </>
        }
    }
}
