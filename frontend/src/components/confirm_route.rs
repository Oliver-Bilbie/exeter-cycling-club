use bounce::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::loading_spinner::LoadingSpinner;
use crate::components::notification::NotificationState;
use crate::components::route_form::{RouteForm, RouteFormData};
use crate::helpers::auth_state::AuthState;
use crate::helpers::set_route::{set_route, SetRouteData};
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct ConfirmRouteProps {
    pub name: String,
    pub route_id: String,
}

#[function_component(ConfirmRoute)]
pub fn confirm_route(props: &ConfirmRouteProps) -> Html {
    let dispatch_notification = use_atom_setter::<NotificationState>();
    let auth_state = use_atom_value::<AuthState>();
    let navigator = use_navigator().unwrap();

    let access_token = match auth_state.user_data {
        Some(ref user_data) => user_data.access_token.clone(),
        None => String::new(),
    };

    let is_loading = use_state_eq(|| false);

    let initial_name = props.name.clone();
    let route_id = props.route_id.clone();

    let handle_submit = {
        let set_form_loading = {
            let is_loading = is_loading.clone();
            move || is_loading.set(true)
        };
        let set_form_complete = {
            let dispatch_notification = dispatch_notification.clone();
            let navigator = navigator.clone();
            move || {
                dispatch_notification(NotificationState {
                    message: "Route set successfully".to_string(),
                    color: "success".to_string(),
                    visible: true,
                });
                navigator.push(&Route::Home);
            }
        };
        let set_form_ready = {
            let is_loading = is_loading.clone();
            move || is_loading.set(false)
        };

        let notification_cb =
            Callback::from(move |response: Result<String, String>| match response {
                Ok(_) => {
                    dispatch_notification(NotificationState {
                        message: String::new(),
                        color: "primary".to_string(),
                        visible: false,
                    });
                    set_form_complete();
                }
                Err(message) => {
                    dispatch_notification(NotificationState {
                        message,
                        color: "primary".to_string(),
                        visible: true,
                    });
                    set_form_ready();
                }
            });

        move |route_form_data: RouteFormData| {
            let notification_cb = notification_cb.clone();
            let route_id = route_id.clone();
            let access_token = access_token.clone();
            set_form_loading();
            spawn_local(async move {
                let resp = set_route(SetRouteData {
                    id: route_id,
                    name: route_form_data.name,
                    message: route_form_data.message,
                    is_private: route_form_data.is_private,
                    access_token: access_token.to_owned(),
                })
                .await;
                notification_cb.emit(resp);
            });
        }
    };

    html! {
        <div class="hero texture-light is-flex-grow-5" style="min-height: 600px;">
            <div class="container">
                <div class="my-6 mx-4">
                    {match *is_loading {
                        true => html! {
                            <LoadingSpinner size={200} />
                        },
                        false => html!(
                            <RouteForm
                                initial_name={initial_name}
                                on_submit={handle_submit}
                            />
                        ),
                    }}
                </div>
            </div>
        </div>
    }
}
