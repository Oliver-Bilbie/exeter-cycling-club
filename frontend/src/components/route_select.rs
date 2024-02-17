use wasm_bindgen::JsCast;
use bounce::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use web_sys::{EventTarget, HtmlInputElement};

use crate::components::confirm_route::{ConfirmRoute, ConfirmRouteProps};
use crate::components::footer::Footer;
use crate::components::loading_spinner::LoadingSpinner;
use crate::components::nav_bar::NavBar;
use crate::components::no_route_display::NoRouteDisplay;
use crate::components::notification::NotificationState;
use crate::components::page_header::PageHeader;
use crate::components::route_card::RouteCard;
use crate::helpers::auth_state::AuthState;
use crate::helpers::list_routes::{list_routes, RouteData};
use crate::Route;

#[derive(PartialEq)]
enum FormStatus {
    Loading,
    Ready(Vec<RouteData>),
    Confirm(ConfirmRouteProps),
    Error(String),
}

#[function_component(RouteSelect)]
pub fn route_select() -> Html {
    let dispatch_notification = use_atom_setter::<NotificationState>();
    let auth_state = use_atom_value::<AuthState>();
    let navigator = use_navigator().unwrap();
    let form_status = use_state_eq(|| FormStatus::Loading);
    let search_value = use_state_eq(String::new);

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
                let is_admin = user_data.admin;
                if !is_admin {
                    dispatch_notification(NotificationState {
                        message: "You are not authorized to set routes.".to_string(),
                        color: "primary".to_string(),
                        visible: true,
                    });
                    navigator.push(&Route::Home);
                }

                let id = user_data.id.clone();
                let access_token = user_data.access_token.clone();
                spawn_local(async move {
                    let resp = list_routes(id, access_token).await;
                    status_callback.emit(resp);
                });
            }
            None => {
                dispatch_notification(NotificationState {
                    message: "Please sign in before attempting to set a route.".to_string(),
                    color: "primary".to_string(),
                    visible: true,
                });
                navigator.push(&Route::Home);
            }
        });
    }

    let handle_search = {
        let search_value = search_value.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let user_input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = user_input {
                search_value.set(input.value());
            }
        })
    };

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
                <div>
                    <div class="field container pb-5" style="max-width: 350px;">
                        <label class="label is-size-5">
                            {"Search"}
                        </label>
                        <div class="control">
                            <input
                                class="input is-medium"
                                onchange={handle_search}
                                placeholder="Route name"
                                type="text"
                            />
                        </div>
                    </div>
                    <div class="route-grid">
                        {route_data
                            .iter()
                            .filter(|route| {route.name.to_lowercase().contains(&search_value.to_lowercase())})
                            .map(|route| html! {
                                <a class="route-card-button" onclick={handle_select_route(route.id_str.clone(), route.name.clone())}>
                                    <RouteCard route_data={route.clone()} />
                                </a>
                            }).collect::<Html>()}
                    </div>
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
            <PageHeader title="Set route" />

            <section class="section texture-light pt-8 is-flex is-flex-grow-5 is-flex-direction-column is-align-content-center">
                {page_body(&form_status)}
            </section>

            <Footer />
        </section>
    }
}
