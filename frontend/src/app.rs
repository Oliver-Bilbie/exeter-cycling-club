use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::{
    contact_us::ContactUs, home::Home, not_found::NotFound, ride_page::RidePage,
    unsubscribe::Unsubscribe,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[at("/ride")]
    RidePage,
    #[at("/contact")]
    Contact,
    #[at("/signin/:id")]
    SignIn { id: String },
    #[at("/redirect/:id")]
    Redirect { id: String },
    #[at("/unsubscribe")]
    Unsubscribe,
    #[at("/status/:id")]
    SetStatus { id: String },
    #[at("/select")]
    RouteSelect,
    #[at("/cancel")]
    RouteCancel,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home header_visible={true} /> },
        Route::About => html! { <Home header_visible={false} /> },
        Route::Contact => html! { <ContactUs /> },
        Route::RidePage => html! { <RidePage /> },
        Route::Unsubscribe => html! { <Unsubscribe /> },
        Route::NotFound => html! { <NotFound /> },
        _ => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    }
}
