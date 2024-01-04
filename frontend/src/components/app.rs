use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    home::Home, 
    ride_page::RidePage,
    contact_us::ContactUs,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/contact")]
    Contact,
    #[at("/ride")]
    RidePage,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::About => html! { <h1>{ "About" }</h1> },
        Route::Contact => html! { <ContactUs /> },
        Route::RidePage => html! { <RidePage /> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
