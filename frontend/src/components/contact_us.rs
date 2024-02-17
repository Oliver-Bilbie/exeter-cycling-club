use yew::platform::spawn_local;
use yew::prelude::*;
use bounce::use_atom_setter;

use crate::components::contact_form::{ContactForm, MessageData};
use crate::components::footer::Footer;
use crate::components::loading_spinner::LoadingSpinner;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;
use crate::helpers::send_email::send_email;
use crate::helpers::validate_email::validate_email;
use crate::components::notification::NotificationState;

#[derive(Clone)]
struct NotificationData {
    message: String,
    color: String,
    visible: bool,
}

#[derive(PartialEq)]
enum FormState {
    Ready,
    Loading,
    Complete,
}

#[function_component(ContactUs)]
pub fn contact_us() -> Html {
    let dispatch_notification = use_atom_setter::<NotificationState>();

    let form_data = use_state(|| MessageData {
        email: String::new(),
        message: String::new(),
    });
    let form_state = use_state_eq(|| FormState::Ready);

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

        let form_data = MessageData {
            email: form_data.email.clone(),
            message: form_data.message.clone(),
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
            match validate_email(form_data.email.clone()) {
                false => notification_cb.emit(Err("Invalid email address".to_string())),
                true => {
                    let submit_data = form_data.clone();
                    let callback = notification_cb.clone();
                    spawn_local(async move {
                        let resp = send_email(submit_data.email, submit_data.message).await;
                        callback.emit(resp);
                    })
                }
            }
        }
    };

    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={false} />
            <PageHeader title="Contact us" />

            <div class="hero texture-light is-flex-grow-5" style="min-height: 600px;">
                <div class="container">
                    <div class="my-6 mx-4">
                        {match *form_state {
                            FormState::Ready => html!(
                                <ContactForm form_data={form_data} on_submit={handle_submit} />
                            ),
                            FormState::Loading => html! {
                                <LoadingSpinner size={128} />
                            },
                            FormState::Complete => html! {
                                <h2 class="title is-2 has-text-centered">
                                    {"Message sent!"}
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
