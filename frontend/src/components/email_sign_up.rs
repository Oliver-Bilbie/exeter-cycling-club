use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::loading_spinner::LoadingSpinner;
use crate::components::notification::Notification;
use crate::helpers::sign_up::sign_up;
use crate::helpers::validate_email::validate_email;

#[derive(Clone)]
struct SignUpData {
    email: String,
    name: String,
}

#[derive(Clone)]
struct NotificationData {
    message: String,
    color: String,
    visible: bool,
}

enum FormState {
    Ready,
    Loading,
    Complete,
}

#[function_component(EmailSignUp)]
pub fn email_sign_up() -> Html {
    let form_data = use_state(|| SignUpData {
        email: String::new(),
        name: String::new(),
    });

    let notification_data = use_state(|| NotificationData {
        message: String::new(),
        color: String::new(),
        visible: false,
    });

    let form_state = use_state(|| FormState::Ready);

    let handle_update_email = {
        let form_data = form_data.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let user_input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = user_input {
                form_data.set(SignUpData {
                    email: input.value(),
                    name: form_data.name.clone(),
                });
            }
        })
    };

    let handle_update_message = {
        let form_data = form_data.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let user_input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = user_input {
                form_data.set(SignUpData {
                    email: form_data.email.clone(),
                    name: input.value(),
                });
            }
        })
    };

    let handle_submit = {
        let set_form_loading = {
            let form_state = form_state.clone();
            let notification_data = notification_data.clone();
            move || {
                form_state.set(FormState::Loading);
                notification_data.set(NotificationData {
                    message: String::new(),
                    color: "primary".to_string(),
                    visible: false,
                });
            }
        };
        let set_form_complete = {
            let form_state = form_state.clone();
            move || form_state.set(FormState::Complete)
        };
        let set_form_ready = {
            let form_state = form_state.clone();
            move || form_state.set(FormState::Ready)
        };

        let form_data = SignUpData {
            email: form_data.email.clone(),
            name: form_data.name.clone(),
        };
        let notification_data = notification_data.clone();

        let notification_cb =
            Callback::from(move |response: Result<String, String>| match response {
                Ok(_) => {
                    notification_data.set(NotificationData {
                        message: String::new(),
                        color: String::new(),
                        visible: false,
                    });
                    set_form_complete();
                }
                Err(message) => {
                    notification_data.set(NotificationData {
                        message,
                        color: "primary".to_string(),
                        visible: true,
                    });
                    set_form_ready();
                }
            });

        move |_| {
            set_form_loading();
            match validate_email(form_data.email.clone()) {
                false => notification_cb.emit(Err("Invalid email address".to_string())),
                true => {
                    let submit_data = form_data.clone();
                    let callback = notification_cb.clone();
                    spawn_local(async move {
                        let resp = sign_up(submit_data.email, submit_data.name).await;
                        callback.emit(resp);
                    })
                }
            }
        }
    };

    let handle_hide_notification = {
        let notification_data = notification_data.clone();
        Callback::from(move |_| {
            notification_data.set(NotificationData {
                message: String::new(),
                color: String::new(),
                visible: false,
            });
        })
    };

    let form_body = {
        move |form_state: &FormState| match form_state {
            FormState::Ready => { html! {
                <form>
                    <div class="field">
                        <div class="control">
                            <input
                                class="input is-medium"
                                onchange={handle_update_message}
                                placeholder="Name"
                                type="text"
                            />
                        </div>
                    </div>
                    <div class="field">
                        <div class="control has-icons-left has-icons-right">
                            <input
                                class="input is-medium"
                                onchange={handle_update_email}
                                placeholder="Email"
                                type="email"
                            />
                            <span class="icon is-small is-left p-2">
                                <img src="images/email.svg" />
                            </span>
                        </div>
                    </div>
                    <div class="has-text-centered">
                        <button class="button is-primary is-medium" onclick={handle_submit} type="button" >
                            {"Sign up"}
                        </button>
                    </div>
                </form>
            }},
            FormState::Loading => { html! {
                <div class="container is-vcentered mb-6" style="display: grid;">
                    <LoadingSpinner size={100} />
                </div>
            }},
            FormState::Complete => { html! {
                <div class="container is-vcentered mb-6" style="display: grid;">
                    <h2 class="title is-2 has-text-centered">
                        {"Thanks for signing up!"}
                    </h2>
                </div>
            }},
        }
    };

    html!(
        <div class="container" style="max-width: 400px;">
            <h4 class="title is-4 has-text-centered">
                {"Subscribe to email alerts"}
            </h4>

            {form_body(&form_state)}

            if notification_data.visible {
                <Notification
                    message={notification_data.message.clone()}
                    color={notification_data.color.clone()}
                    on_close={handle_hide_notification}
                />
            }
        </div>
    )
}
