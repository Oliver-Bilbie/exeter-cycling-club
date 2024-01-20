use bounce::BounceRoot;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

use crate::components::contact_us::ContactUs;
use crate::components::home::Home;
use crate::components::not_found::NotFound;
use crate::components::redirect::Redirect;
use crate::components::ride_page::RidePage;
use crate::components::route_cancel::RouteCancel;
use crate::components::route_select::RouteSelect;
use crate::components::set_status::SetStatus;
use crate::components::sign_in::SignIn;
use crate::components::sign_out::SignOut;
use crate::components::unsubscribe::Unsubscribe;

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
    #[at("/signout")]
    SignOut,
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
        Route::SignOut => html! { <SignOut /> },
        Route::Redirect => html! { <Redirect /> },
        Route::RouteSelect => html! { <RouteSelect /> },
        Route::RouteCancel => html! { <RouteCancel /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    wasm_logger::init(wasm_logger::Config::default());
    html! {
        <BrowserRouter>
            <BounceRoot>
                <Switch<Route> render={switch} />
            </BounceRoot>
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
            <BounceRoot>
                <Switch<Route> render={switch} />
            </BounceRoot>
        </Router>
    }
}
