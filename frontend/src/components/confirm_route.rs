use bounce::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::loading_spinner::LoadingSpinner;
use crate::components::route_form::RouteForm;
use crate::helpers::auth_state::AuthState;
use crate::helpers::set_route::{set_route, SetRouteData};

#[derive(PartialEq)]
enum FormState {
    Ready,
    Loading,
    Complete,
}

#[derive(Properties, PartialEq)]
pub struct ConfirmRouteProps {
    pub name: String,
    pub route_id: String,
}

#[function_component(ConfirmRoute)]
pub fn confirm_route(props: &ConfirmRouteProps) -> Html {
    let auth_state = use_atom_value::<AuthState>();
    let access_token = match auth_state.user_data {
        Some(ref user_data) => user_data.access_token.clone(),
        None => String::new(),
    };

    let set_route_data = use_state(|| SetRouteData {
        id: props.route_id.clone(),
        name: props.name.clone(),
        message: String::new(),
        access_token,
    });
    let form_state = use_state_eq(|| FormState::Ready);

    let handle_submit = {
        let set_form_loading = {
            let form_state = form_state.clone();
            move || form_state.set(FormState::Loading)
        };
        let set_form_complete = {
            let form_state = form_state.clone();
            move || form_state.set(FormState::Complete)
        };
        let set_form_ready = {
            let form_state = form_state.clone();
            move || form_state.set(FormState::Ready)
        };

        let set_route_data = SetRouteData {
            id: set_route_data.id.clone(),
            name: set_route_data.name.clone(),
            message: set_route_data.message.clone(),
            access_token: set_route_data.access_token.clone(),
        };

        let notification_cb =
            Callback::from(move |response: Result<String, String>| match response {
                Ok(_) => {
                    set_form_complete();
                }
                Err(_) => {
                    set_form_ready();
                }
            });

        move |_| {
            let set_route_data = set_route_data.clone();
            let notification_cb = notification_cb.clone();
            set_form_loading();
            spawn_local(async move {
                let resp = set_route(set_route_data).await;
                notification_cb.emit(resp);
            });
        }
    };

    html! {
        <div class="hero texture-light is-flex-grow-5" style="min-height: 600px;">
            <div class="container">
                <div class="my-6 mx-4">
                    {match *form_state {
                        FormState::Ready => html!(
                            <RouteForm
                                route_data={set_route_data.clone()}
                                on_submit={handle_submit}
                            />
                        ),
                        FormState::Loading => html! {
                            <LoadingSpinner size={128} />
                        },
                        FormState::Complete => html! {
                            <h2 class="title is-2 has-text-centered">
                                {"Route set successfully"}
                            </h2>
                        },
                    }}
                </div>
            </div>
        </div>
    }
}
