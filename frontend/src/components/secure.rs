use yew_router::prelude::*;
use yew::prelude::*;

use crate::components::app::Route;

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Upcoming Ride" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}
