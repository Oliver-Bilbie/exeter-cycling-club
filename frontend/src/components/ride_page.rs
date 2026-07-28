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
use crate::helpers::route_state::RouteState;

#[function_component(RidePage)]
pub fn ride_page() -> Html {
    let auth_state = use_atom_value::<AuthState>();
    let route_state = use_atom_value::<RouteState>();
    let set_route_state = use_atom_setter::<RouteState>();

    {
        let auth_state = auth_state.clone();
        let cached_status = route_state.status.clone();
        let set_route_state = set_route_state.clone();

        // Show cached data immediately; refresh in the background and replace if stale.
        use_effect_with(auth_state.clone(), move |_| {
            let access_token = auth_state
                .user_data
                .as_ref()
                .map(|user| user.access_token.clone());
            spawn_local(async move {
                let status = get_route(access_token).await;
                if status != cached_status {
                    set_route_state(RouteState { status });
                }
            });
            || ()
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
                <div class="page-center mb-6">
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
                {page_body(&route_state.status)}
                <EmailSignUp />
            </section>

            <Footer />
        </section>
    }
}
