use yew::platform::spawn_local;
use yew::prelude::*;
use bounce::prelude::*;

use crate::components::email_sign_up::EmailSignUp;
use crate::components::footer::Footer;
use crate::components::loading_spinner::LoadingSpinner;
use crate::components::nav_bar::NavBar;
use crate::components::no_route_display::NoRouteDisplay;
use crate::components::page_header::PageHeader;
use crate::components::route_card::RouteCard;
use crate::helpers::list_routes::{list_routes, RouteData};
use crate::helpers::auth_state::AuthState;

#[derive(PartialEq)]
enum FormStatus {
    Loading,
    Ready(Vec<RouteData>),
    Error(String),
}

#[function_component(RouteSelect)]
pub fn route_select() -> Html {
    let auth_state = use_atom_value::<AuthState>();
    let form_status = use_state_eq(|| FormStatus::Loading);

    {
        let auth_state = auth_state.clone();
        let form_status = form_status.clone();
        let status_callback =
            Callback::from(
                move |response: Result<Vec<RouteData>, String>| match response {
                    Ok(data) => form_status.set(FormStatus::Ready(data)),
                    Err(err) => form_status.set(FormStatus::Error(err)),
                },
            );

        // Load the route data only once
        use_effect_with(auth_state.clone(), move |_| {
            {match auth_state.user_data {
                Some(ref user_data) => {
                    let id = user_data.id.clone();
                    let access_token = user_data.access_token.clone();
                    spawn_local(async move {
                        let resp = list_routes(id, access_token).await;
                        status_callback.emit(resp);
                    });
                },
                None => status_callback.emit(Err("You need to log in before setting a new route.".to_string())),
            }}
        });
    }

    let page_body = {
        move |form_status: &FormStatus| match form_status {
            FormStatus::Ready(route_data) => html! {
                <div class="columns">
                {
                    route_data.iter().map(|route| html! {
                        <RouteCard route_data={route.clone()} />
                    }).collect::<Html>()
                }
                </div>
            },
            FormStatus::Error(message) => html! { <NoRouteDisplay message={message.clone()} /> },
            FormStatus::Loading => html! {
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
                {page_body(&form_status)}
                <EmailSignUp />
            </section>

            <Footer />
        </section>
    }
}
