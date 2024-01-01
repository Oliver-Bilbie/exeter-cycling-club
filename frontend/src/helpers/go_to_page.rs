use yew_router::prelude::*;
use yew::prelude::*;

use crate::components::app::Route;

pub fn go_to_page(navigator: Navigator, page: Route) -> Callback<MouseEvent> {
    Callback::from(move |_| navigator.push(&page))
}
