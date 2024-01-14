use yew::prelude::*;
use yew::platform::spawn_local;

use crate::components::email_sign_up::EmailSignUp;
use crate::components::footer::Footer;
use crate::components::loading_spinner::LoadingSpinner;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;
use crate::components::route_display::RouteDisplay;
use crate::helpers::get_route::{get_route, RouteStatus};

#[function_component(RidePage)]
pub fn ride_page() -> Html {
    let route_status = use_state_eq(|| RouteStatus::Loading);

    {
        let route_status = route_status.clone();
        let status_callback =
            Callback::from(move |response: RouteStatus| route_status.set(response));

        spawn_local(async move {
            let resp = get_route().await;
            status_callback.emit(resp);
        });
    }

    let page_body = {
        move |route_status: &RouteStatus| match route_status {
            RouteStatus::Ready(route_data) => html! {<RouteDisplay route_data={route_data.clone()} />},
            RouteStatus::Unavailable(message) => html! { <NoRouteDisplay message={message.clone()} /> },
            RouteStatus::Cancelled(message) => html! { <NoRouteDisplay message={message.clone()} /> },
            RouteStatus::Error(message) => html! { <NoRouteDisplay message={message.clone()} /> },
            RouteStatus::Loading => html! { 
                <div class="container is-vcentered mb-6" style="display: grid;">
                    <LoadingSpinner size={200} />
                </div>
            }
        }
    };

    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={false} />
            <PageHeader title="Upcoming ride" />

            <section class="section texture-light pt-8 is-flex-grow-5">
                {page_body(&route_status)}
                <EmailSignUp />
            </section>

            <Footer />
        </section>
    }
}

#[derive(Properties, PartialEq)]
pub struct NoRouteDisplayProps {
    pub message: String,
}

#[function_component(NoRouteDisplay)]
pub fn no_route_display(props: &NoRouteDisplayProps) -> Html {
    html! {
        <div class="container is-vcentered mb-6">
            {props.message.split("\n").map(|paragraph| html! {
                <h2 class="title is-2 has-text-centered">
                    {paragraph}
                </h2>
            }).collect::<Html>()}
        </div>
    }
}
