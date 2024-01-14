use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct MessageData {
    pub email: String,
    pub message: String,
}

#[derive(Properties, PartialEq)]
pub struct ContactFormProps {
    pub form_data: UseStateHandle<MessageData>,
    pub on_submit: Callback<()>,
}

#[function_component(ContactForm)]
pub fn contact_form(props: &ContactFormProps) -> Html {
    let handle_update_email = {
        let message_data = props.form_data.clone();
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
        let message_data = props.form_data.clone();
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
        let on_submit = props.on_submit.clone();
        Callback::from(move |_| {
            on_submit.emit(());
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
                    {"Message"}
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
                <button class="button is-primary is-medium" onclick={handle_submit} type="button" >
                    {"Send"}
                </button>
            </div>
        </form>
    )
}
