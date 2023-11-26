use yew::prelude::*;
use yew_router::components::Link;

use crate::Route;

#[derive(Properties, PartialEq)]
pub struct NavLinkProps {
    pub text: String,
    pub route: Route,
    pub current: Route,
}

#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProps) -> Html {
    let class_name = if props.route == props.current {
        "active-link"
    } else {
        ""
    };

    html! {
        <Link<Route>
            classes={classes!(class_name)}
            to={props.route.clone()}
        >
            { format!("[{}]", props.text) }
        </Link<Route>>
    }
}
