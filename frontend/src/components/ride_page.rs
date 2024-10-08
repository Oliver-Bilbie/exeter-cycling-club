use bounce::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::email_sign_up::EmailSignUp;
use crate::components::footer::Footer;
use crate::components::loading_spinner::LoadingSpinner;
use crate::components::nav_bar::NavBar;
use crate::components::no_route_display::NoRouteDisplay;
use crate::components::page_header::PageHeader;
use crate::components::route_display::RouteDisplay;
use crate::helpers::auth_state::AuthState;
use crate::helpers::get_route::{get_route, RouteStatus};

#[function_component(RidePage)]
pub fn ride_page() -> Html {
    let auth_state = use_atom_value::<AuthState>();

    let route_status = use_state_eq(|| RouteStatus::Loading);

    {
        let auth_state = auth_state.clone();
        let route_status = route_status.clone();
        let status_callback =
            Callback::from(move |response: RouteStatus| route_status.set(response));

        // Load the route data only once
        use_effect_with(auth_state.clone(), move |_| {
            spawn_local(async move {
                let access_token: Option<String> = match auth_state.user_data {
                    Some(ref user_data) => Some(user_data.access_token.clone()),
                    None => None,
                };

                let resp = get_route(access_token).await;
                status_callback.emit(resp);
            });
        });
    }

    let page_body = {
        move |route_status: &RouteStatus| match route_status {
            RouteStatus::Ready(route_data) => {
                html! {<RouteDisplay route_data={route_data.clone()} />}
            }
            RouteStatus::Unavailable(message) => {
                html! { <NoRouteDisplay message={message.clone()} /> }
            }
            RouteStatus::Cancelled(message) => {
                html! { <NoRouteDisplay message={message.clone()} /> }
            }
            RouteStatus::Error(message) => html! { <NoRouteDisplay message={message.clone()} /> },
            RouteStatus::Loading => html! {
                <div class="container is-vcentered mb-6" style="display: grid;">
                    <LoadingSpinner size={200} />
                </div>
            },
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
