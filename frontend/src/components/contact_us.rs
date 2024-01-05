use regex::Regex;
use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

use crate::components::footer::Footer;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;

#[function_component(ContactUs)]
pub fn contact_us() -> Html {
    let email = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());
    let notification_visible = use_state(|| false);

    let on_email_change = {
        let email = email.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = input {
                email.set(input.value());
            }
        })
    };

    let show_message = {
        let message_visible = notification_visible.clone();
        move |_| {
            message_visible.set(true);
        }
    };

    let hide_message = {
        let message_visible = notification_visible.clone();
        move |_| {
            message_visible.set(false);
        }
    };

    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={false} />
            <PageHeader title="Contact us" />

            <div class="hero texture-light">
                <div class="container">
                    <div class="my-6 mx-4">
                        <div class="field">
                            <label class="label is-size-5">{ "Your email address" }</label>
                            <div class="control has-icons-left has-icons-right">
                                <input class="input is-medium" type="email" placeholder="Email" onchange={on_email_change} />
                                <span class="icon is-small is-left p-2">
                                    <img src="images/email.svg" />
                                </span>
                            </div>
                        </div>

                        <div class="field">
                            <label class="label is-size-5">{ "Message" }</label>
                            <div class="control">
                                <textarea class="textarea has-fixed-size is-medium" placeholder="Message" style="height: 300px;" />
                            </div>
                        </div>

                        <div class="has-text-centered">
                            <button class="button is-primary is-medium" onclick={show_message}>
                                { "Send" }
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            if *notification_visible {
                <div class="notification is-primary" style="position: fixed; top: 20px; left: 5%; z-index: 999; width: 90%; border-radius: 16px;">
                    <button class="delete is-medium" onclick={hide_message} />
                    <h5 class="title is-5 has-text-centered">
                        { "This isn't hooked up to anything yet, but pretend that we'll get back to you soon!" }
                    </h5>
                </div>
            }
            <Footer />
        </section>
    }
}

fn validate_email(email: String) -> bool {
    match Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$") {
        Ok(re) => re.is_match(&email),
        Err(_) => false,
    }
}
