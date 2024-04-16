// Component for resetting the scroll position to the top of the page when the route changes.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::Route;
use crate::helpers::scroll_to_top::scroll_to_top;

#[derive(Properties, PartialEq, Clone)]
pub struct ResetScrollProps {
    pub children: Children,
}

#[function_component(ResetScroll)]
pub fn reset_scroll(props: &ResetScrollProps) -> Html {
    let route = use_route::<Route>();
    let initial_render = use_state_eq(|| true);

    use_effect_with(route, move |_| {
        let initial_render = initial_render.clone();
        match *initial_render {
            true => {
                initial_render.set(false);
            }
            false => {
                scroll_to_top();
            }
        }
    });

    html! { props.children.clone() }
}
