use bounce::use_atom_setter;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::loading_spinner::LoadingSpinner;
use crate::components::notification::NotificationState;
use crate::helpers::sign_up::sign_up;
use crate::helpers::validate_email::validate_email;

enum FormState {
    Ready,
    Loading,
    Complete,
}

#[function_component(EmailSignUp)]
pub fn email_sign_up() -> Html {
    let dispatch_notification = use_atom_setter::<NotificationState>();

    let form_state = use_state(|| FormState::Ready);

    let handle_submit = {
        let set_form_loading = {
            let form_state = form_state.clone();
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
            let form_state = form_state.clone();
            move || form_state.set(FormState::Complete)
        };
        let set_form_ready = {
            let form_state = form_state.clone();
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

        move |_| {
            set_form_loading();

            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let name_element = document.get_element_by_id("name-input");
            let name_input = name_element.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let name_value = match name_input {
                Some(name) => name.value(),
                None => "".to_string(),
            };

            let email_element = document.get_element_by_id("email-input");
            let email_input = email_element.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let email_value = match email_input {
                Some(email) => email.value(),
                None => "".to_string(),
            };

            match validate_email(&email_value) {
                false => notification_cb.emit(Err("Invalid email address".to_string())),
                true => {
                    let callback = notification_cb.clone();
                    spawn_local(async move {
                        let resp = sign_up(email_value, name_value).await;
                        callback.emit(resp);
                    })
                }
            }
        }
    };

    let form_body = {
        move |form_state: &FormState| match form_state {
            FormState::Ready => {
                html! {
                    <form>
                        <div class="field">
                            <div class="control has-icons-left has-icons-right">
                                <input
                                    id="name-input"
                                    class="input is-medium"
                                    placeholder="Name"
                                    type="text"
                                />
                                <span class="icon is-small is-left p-2">
                                    <img src="/images/user_icon.svg" />
                                </span>
                            </div>
                        </div>
                        <div class="field">
                            <div class="control has-icons-left has-icons-right">
                                <input
                                    id="email-input"
                                    class="input is-medium"
                                    placeholder="Email"
                                    type="email"
                                />
                                <span class="icon is-small is-left p-2">
                                    <img src="/images/email.svg" />
                                </span>
                            </div>
                        </div>
                        <div class="has-text-centered">
                            <button class="button is-primary is-medium" onclick={handle_submit} type="button" >
                                {"Sign up"}
                            </button>
                        </div>
                    </form>
                }
            }
            FormState::Loading => {
                html! {
                    <div class="container is-vcentered mb-6" style="display: grid;">
                        <LoadingSpinner size={100} />
                    </div>
                }
            }
            FormState::Complete => {
                html! {
                    <div class="container is-vcentered mb-6" style="display: grid;">
                        <h2 class="title is-2 has-text-centered">
                            {"Thanks for signing up!"}
                        </h2>
                    </div>
                }
            }
        }
    };

    html!(
        <div class="container" style="max-width: 400px;">
            <h4 class="title is-4 has-text-centered">
                {"Subscribe to email alerts"}
            </h4>
            {form_body(&form_state)}
        </div>
    )
}
