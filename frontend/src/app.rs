use std::collections::HashMap;

use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::contact_us::ContactUs;
use crate::components::home::Home;
use crate::components::not_found::NotFound;
use crate::components::ride_page::RidePage;
use crate::components::set_status::SetStatus;
use crate::components::unsubscribe::Unsubscribe;
use crate::components::sign_in::SignIn;
use crate::components::redirect::Redirect;

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
    #[at("/unsubscribe")]
    Unsubscribe,
    #[at("/status")]
    SetStatus,
    #[at("/signin")]
    SignIn,
    #[at("/redirect")]
    Redirect,
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
        Route::SetStatus => html! { <SetStatus /> },
        Route::Unsubscribe => html! { <Unsubscribe /> },
        Route::SignIn => html! { <SignIn /> },
        Route::Redirect => html! { <Redirect /> },
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
