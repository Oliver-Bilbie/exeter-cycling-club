// TODO: Add loading spinner
// TODO: Add email validation

use regex::Regex;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::platform::spawn_local;
use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;
use crate::helpers::send_email::send_email;

#[derive(Clone, Debug)]
struct MessageData {
    email: String,
    message: String,
}

struct NotificationData {
    message: String,
    color: String,
    visible: bool,
}

#[function_component(ContactUs)]
pub fn contact_us() -> Html {
    let message_data = use_state(|| MessageData {
        email: "".to_string(),
        message: "".to_string(),
    });

    let notification_data = use_state(|| NotificationData {
        message: "".to_string(),
        color: "".to_string(),
        visible: false,
    });

    let handle_update_email = {
        let message_data = message_data.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let user_input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = user_input {
                message_data.set(MessageData {
                    email: input.value(),
                    message: message_data.message.clone(),
                });
            }
        })
    };

    let handle_update_message = {
        let message_data = message_data.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let user_input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = user_input {
                message_data.set(MessageData {
                    email: message_data.email.clone(),
                    message: input.value(),
                });
            }
        })
    };

    let handle_submit = {
        let message_data = MessageData {
            email: message_data.email.clone(),
            message: message_data.message.clone(),
        };
        let notification_data = notification_data.clone();
        let notification_cb = Callback::from(move |response: NotificationData| {
            notification_data.set(response);
        });

        move |_| {
            fetch_data(message_data.clone(), notification_cb.clone());
        }
    };

    let handle_hide_notification = {
        let notification_data = notification_data.clone();
        Callback::from(move |_| {
            notification_data.set(NotificationData {
                message: "".to_string(),
                color: "".to_string(),
                visible: false,
            });
        })
    };

    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={false} />
            <PageHeader title="Contact us" />

            <div class="hero texture-light">
                <div class="container">
                    <div class="my-6 mx-4">
                        <div class="field">
                            <label class="label is-size-5">
                                { "Your email address" }
                            </label>
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

                        <div class="field">
                            <label class="label is-size-5">
                                { "Message" }
                            </label>
                            <div class="control">
                                <textarea
                                    class="textarea has-fixed-size is-medium"
                                    onchange={handle_update_message}
                                    placeholder="Message"
                                    style="height: 300px;"
                                />
                            </div>
                        </div>

                        <div class="has-text-centered">
                            <button class="button is-primary is-medium" onclick={ handle_submit } >
                                { "Send" }
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            if notification_data.visible {
                <Notification
                    message={notification_data.message.clone()}
                    color={notification_data.color.clone()}
                    on_close={handle_hide_notification}
                />
            }

            <Footer />
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct NotificationProps {
    message: String,
    color: String,
    on_close: Callback<MouseEvent>,
}

#[function_component(Notification)]
fn notification(props: &NotificationProps) -> Html {
    let on_close = props.on_close.clone();
    html! {
        <div
            class={format!("notification is-{}", props.color)}
            style="position: fixed; top: 20px; left: 5%; z-index: 999; width: 90%; border-radius: 16px;"
        >
            <button class="delete is-medium" onclick={ on_close } />
            <h5 class="title is-5 has-text-centered">
                { props.message.clone() }
            </h5>
        </div>
    }
}

fn validate_email(email: String) -> bool {
    match Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$") {
        Ok(re) => re.is_match(&email),
        Err(_) => false,
    }
}

fn fetch_data(message_data: MessageData, notification_cb: Callback<NotificationData>) {
    spawn_local(async move {
        let resp = send_email(message_data.email, message_data.message).await;

        match resp {
            Ok(message) => notification_cb.emit(NotificationData {
                message,
                color: "success".to_string(),
                visible: true,
            }),
            Err(_) => notification_cb.emit(NotificationData {
                message: "Failed to send message".to_string(),
                color: "primary".to_string(),
                visible: true,
            }),
        }
    })
}
