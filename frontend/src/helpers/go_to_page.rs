use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

pub fn go_to_page(navigator: Navigator, page: Route) -> Callback<MouseEvent> {
    Callback::from(move |_| navigator.push(&page))
}
