use yew::prelude::*;

use crate::{year::Year, NavLink, Route};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub year: u32,
    pub route: Route,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <header>
            <Year current={props.year} />
            <nav class="links">
                {
                    match props.year {
                        2023 => {
                            html! {
                                <>
                                    <NavLink route={Route::Solution { year: 2023, day: 1 }} current={props.route.clone()} text={"1"}/>
                                    <NavLink route={Route::Solution { year: 2023, day: 2 }} current={props.route.clone()} text={"2"}/>
                                    <NavLink route={Route::Solution { year: 2023, day: 3 }} current={props.route.clone()} text={"3"}/>
                                    <NavLink route={Route::Solution { year: 2023, day: 4 }} current={props.route.clone()} text={"4"}/>
                                    <NavLink route={Route::Solution { year: 2023, day: 5 }} current={props.route.clone()} text={"5"}/>
                                    <NavLink route={Route::Solution { year: 2023, day: 6 }} current={props.route.clone()} text={"6"}/>
                                    <NavLink route={Route::Solution { year: 2023, day: 7 }} current={props.route.clone()} text={"7"}/>
                                    <NavLink route={Route::Solution { year: 2023, day: 8 }} current={props.route.clone()} text={"8"}/>
                                    <NavLink route={Route::Solution { year: 2023, day: 9 }} current={props.route.clone()} text={"9"}/>
                                    <NavLink route={Route::Solution { year: 2023, day: 10 }} current={props.route.clone()} text={"10"}/>
                                </>
                            }
                        },
                        2022 => {
                            html! {
                                <>
                                    <NavLink route={Route::Solution { year: 2022, day: 1 }} current={props.route.clone()} text={"1"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 2 }} current={props.route.clone()} text={"2"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 3 }} current={props.route.clone()} text={"3"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 4 }} current={props.route.clone()} text={"4"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 5 }} current={props.route.clone()} text={"5"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 6 }} current={props.route.clone()} text={"6"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 7 }} current={props.route.clone()} text={"7"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 8 }} current={props.route.clone()} text={"8"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 9 }} current={props.route.clone()} text={"9"}/>
                                    <NavLink route={Route::Rope} current={props.route.clone()} text={"9+"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 10 }} current={props.route.clone()} text={"10"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 11 }} current={props.route.clone()} text={"11"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 12 }} current={props.route.clone()} text={"12"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 13 }} current={props.route.clone()} text={"13"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 14 }} current={props.route.clone()} text={"14"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 15 }} current={props.route.clone()} text={"15"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 16 }} current={props.route.clone()} text={"16"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 17 }} current={props.route.clone()} text={"17"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 18 }} current={props.route.clone()} text={"18"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 19 }} current={props.route.clone()} text={"19"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 20 }} current={props.route.clone()} text={"20"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 21 }} current={props.route.clone()} text={"21"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 22 }} current={props.route.clone()} text={"22"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 23 }} current={props.route.clone()} text={"23"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 24 }} current={props.route.clone()} text={"24"}/>
                                    <NavLink route={Route::Solution { year: 2022, day: 25 }} current={props.route.clone()} text={"25"}/>
                                    <NavLink route={Route::Lava} current={props.route.clone()} text={"18+"}/>
                                    <NavLink route={Route::Cube} current={props.route.clone()} text={"22+"}/>
                                </>
                            }
                        },
                        2021 => {
                            html! {
                                <>
                                    <NavLink route={Route::Solution { year: 2021, day: 1 }} current={props.route.clone()} text={"1"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 2 }} current={props.route.clone()} text={"2"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 3 }} current={props.route.clone()} text={"3"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 4 }} current={props.route.clone()} text={"4"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 5 }} current={props.route.clone()} text={"5"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 6 }} current={props.route.clone()} text={"6"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 7 }} current={props.route.clone()} text={"7"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 8 }} current={props.route.clone()} text={"8"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 9 }} current={props.route.clone()} text={"9"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 10 }} current={props.route.clone()} text={"10"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 11 }} current={props.route.clone()} text={"11"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 12 }} current={props.route.clone()} text={"12"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 13 }} current={props.route.clone()} text={"13"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 14 }} current={props.route.clone()} text={"14"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 15 }} current={props.route.clone()} text={"15"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 16 }} current={props.route.clone()} text={"16"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 17 }} current={props.route.clone()} text={"17"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 18 }} current={props.route.clone()} text={"18"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 21 }} current={props.route.clone()} text={"21"}/>
                                    <NavLink route={Route::Solution { year: 2021, day: 22 }} current={props.route.clone()} text={"22"}/>
                                </>
                            }
                        },
                        2020 => {
                            html! {
                                <>
                                    <NavLink route={Route::Solution { year: 2020, day: 1 }} current={props.route.clone()} text={"1"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 2 }} current={props.route.clone()} text={"2"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 3 }} current={props.route.clone()} text={"3"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 4 }} current={props.route.clone()} text={"4"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 5 }} current={props.route.clone()} text={"5"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 6 }} current={props.route.clone()} text={"6"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 7 }} current={props.route.clone()} text={"7"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 8 }} current={props.route.clone()} text={"8"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 9 }} current={props.route.clone()} text={"9"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 10 }} current={props.route.clone()} text={"10"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 11 }} current={props.route.clone()} text={"11"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 12 }} current={props.route.clone()} text={"12"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 13 }} current={props.route.clone()} text={"13"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 14 }} current={props.route.clone()} text={"14"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 15 }} current={props.route.clone()} text={"15"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 16 }} current={props.route.clone()} text={"16"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 17 }} current={props.route.clone()} text={"17"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 18 }} current={props.route.clone()} text={"18"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 19 }} current={props.route.clone()} text={"19"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 20 }} current={props.route.clone()} text={"20"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 21 }} current={props.route.clone()} text={"21"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 22 }} current={props.route.clone()} text={"22"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 23 }} current={props.route.clone()} text={"23"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 24 }} current={props.route.clone()} text={"24"}/>
                                    <NavLink route={Route::Solution { year: 2020, day: 25 }} current={props.route.clone()} text={"25"}/>
                                </>
                            }
                        },
                        _ => html! {}
                    }
                }
            </nav>
        </header>
    }
}
