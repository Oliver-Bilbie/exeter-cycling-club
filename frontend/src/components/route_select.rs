use bounce::prelude::*;
use log::info;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::confirm_route::{ConfirmRoute, ConfirmRouteProps};
use crate::components::footer::Footer;
use crate::components::loading_spinner::LoadingSpinner;
use crate::components::nav_bar::NavBar;
use crate::components::no_route_display::NoRouteDisplay;
use crate::components::page_header::PageHeader;
use crate::components::route_card::RouteCard;
use crate::helpers::auth_state::AuthState;
use crate::helpers::list_routes::{list_routes, RouteData};

#[derive(PartialEq)]
enum FormStatus {
    Loading,
    Ready(Vec<RouteData>),
    Confirm(ConfirmRouteProps),
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
        use_effect_with(auth_state.clone(), move |_| match auth_state.user_data {
            Some(ref user_data) => {
                let id = user_data.id.clone();
                let access_token = user_data.access_token.clone();
                spawn_local(async move {
                    let resp = list_routes(id, access_token).await;
                    status_callback.emit(resp);
                });
            }
            None => status_callback.emit(Err(
                "You need to log in before setting a new route.".to_string()
            )),
        });
    }

    let handle_select_route = |route_id: String, route_name: String| {
        let route_id = route_id.clone();
        let route_name = route_name.clone();
        let form_status = form_status.clone();
        move |_| {
            form_status.set(FormStatus::Confirm(ConfirmRouteProps {
                route_id: route_id.clone(),
                name: route_name.clone(),
            }))
        }
    };

    let page_body = {
        move |form_status: &FormStatus| match form_status {
            FormStatus::Ready(route_data) => html! {
                <div class="route-grid">
                {
                    route_data.iter().map(|route| html! {
                        <a class="route-card-button" onclick={handle_select_route(route.id_str.clone(), route.name.clone())}>
                            <RouteCard route_data={route.clone()} />
                        </a>
                    }).collect::<Html>()
                }
                </div>
            },
            FormStatus::Confirm(route_info) => {
                html! { <ConfirmRoute name={route_info.name.clone()} route_id={route_info.route_id.clone()} /> }
            }
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

            <section class="section texture-light pt-8 is-flex is-flex-grow-5 is-flex-direction-column is-align-content-center">
                {page_body(&form_status)}
            </section>

            <Footer />
        </section>
    }
}
