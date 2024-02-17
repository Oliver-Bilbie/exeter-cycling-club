use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;
use bounce::prelude::*;

use crate::components::cancel_form::CancelForm;
use crate::components::footer::Footer;
use crate::components::loading_spinner::LoadingSpinner;
use crate::components::nav_bar::NavBar;
use crate::components::notification::NotificationState;
use crate::components::page_header::PageHeader;
use crate::helpers::auth_state::AuthState;
use crate::helpers::cancel_route::{cancel_route, CancelRouteData};
use crate::Route;

#[derive(PartialEq)]
enum FormState {
    Ready,
    Loading,
    Complete,
}

#[function_component(RouteCancel)]
pub fn route_cancel() -> Html {
    let dispatch_notification = use_atom_setter::<NotificationState>();
    let auth_state = use_atom_value::<AuthState>();
    let navigator = use_navigator().unwrap();

    let access_token = match auth_state.user_data {
        Some(ref user_data) => user_data.access_token.clone(),
        None => String::new(),
    };

    let cancellation_message = use_state(|| String::new());
    let form_status = use_state_eq(|| FormState::Ready);

    // Redirect unauthorized users to the home page
    {
        let dispatch_notification = dispatch_notification.clone();
        let auth_state = auth_state.clone();
        use_effect_with(auth_state.clone(), move |_| match auth_state.user_data {
            Some(ref user_data) => {
                let is_admin = user_data.admin;
                if !is_admin {
                    dispatch_notification(NotificationState {
                        message: "You are not authorized to cancel rides.".to_string(),
                        color: "primary".to_string(),
                        visible: true,
                    });
                    navigator.push(&Route::Home);
                }
            }
            None => {
                dispatch_notification(NotificationState {
                    message: "Please sign in before attempting to cancel a ride.".to_string(),
                    color: "primary".to_string(),
                    visible: true,
                });
                navigator.push(&Route::Home);
            }
        });
    }

    let handle_submit = {
        let set_form_loading = {
            let form_state = form_status.clone();
            let dispatch_notification = dispatch_notification.clone();
            move || {
                form_state.set(FormState::Loading);
                dispatch_notification(NotificationState {
                    message: String::new(),
                    color: "primary".to_string(),
                    visible: false,
                });
            }
        };
        let set_form_complete = {
            let form_state = form_status.clone();
            move || form_state.set(FormState::Complete)
        };
        let set_form_ready = {
            let form_state = form_status.clone();
            move || form_state.set(FormState::Ready)
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

        let cancellation_message = cancellation_message.clone();

        move |_| {
            set_form_loading();
            let cancel_route_data = CancelRouteData {
                message: cancellation_message.to_string(),
                access_token: access_token.clone(),
            };
            let callback = notification_cb.clone();
            spawn_local(async move {
                let resp = cancel_route(cancel_route_data).await;
                callback.emit(resp);
            })
        }
    };

    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={false} />
            <PageHeader title="Upcoming ride" />

            <div class="hero texture-light is-flex-grow-5" style="min-height: 600px;">
                <div class="container">
                    <div class="my-6 mx-4">
                        {match *form_status {
                            FormState::Ready => html!(
                                <CancelForm cancellation_message={cancellation_message} on_submit={handle_submit} />
                            ),
                            FormState::Loading => html! {
                                <LoadingSpinner size={128} />
                            },
                            FormState::Complete => html! {
                                <h2 class="title is-2 has-text-centered">
                                    {"Route cancelled successfully"}
                                </h2>
                            },
                        }}
                    </div>
                </div>
            </div>

            <Footer />
        </section>
    }
}
