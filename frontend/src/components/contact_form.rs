use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub struct MessageData {
    pub email: String,
    pub message: String,
}

#[derive(Properties, PartialEq)]
pub struct ContactFormProps {
    pub on_submit: Callback<MessageData>,
}

#[function_component(ContactForm)]
pub fn contact_form(props: &ContactFormProps) -> Html {
    let handle_submit = {
        let on_submit = props.on_submit.clone();

        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let email_element = document.get_element_by_id("email-input");
            let email_input = email_element.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let email_value = match email_input {
                Some(email) => email.value(),
                None => "".to_string(),
            };

            let message_element = document.get_element_by_id("message-input");
            let message_input =
                message_element.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            let message_value = match message_input {
                Some(message) => message.value(),
                None => "".to_string(),
            };

            on_submit.emit(MessageData {
                email: email_value.clone(),
                message: message_value.clone(),
            });
        })
    };

    html!(
        <form>
            <div class="field">
                <label class="label is-size-5">
                    {"Your email address"}
                </label>
                <div class="control has-icons-left has-icons-right">
                    <input
                        id="email-input"
                        class="input is-medium"
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
                    {"Message"}
                </label>
                <div class="control">
                    <textarea
                        id="message-input"
                        class="textarea has-fixed-size is-medium"
                        placeholder="Message"
                        style="height: 300px;"
                    />
                </div>
            </div>

            <div class="has-text-centered">
                <button class="button is-primary is-medium" onclick={handle_submit} type="button" >
                    {"Send"}
                </button>
            </div>
        </form>
    )
}
