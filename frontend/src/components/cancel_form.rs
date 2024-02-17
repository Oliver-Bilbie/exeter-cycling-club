use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CancelFormProps {
    pub cancellation_message: UseStateHandle<String>,
    pub on_submit: Callback<()>,
}

#[function_component(CancelForm)]
pub fn cancel_form(props: &CancelFormProps) -> Html {
    let handle_update_message = {
        let cancellation_message = props.cancellation_message.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let user_input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = user_input {
                cancellation_message.set(input.value());
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
                    {"Cancellation Message"}
                </label>
                <div class="control">
                    <textarea
                        class="textarea has-fixed-size is-medium"
                        onchange={handle_update_message}
                        placeholder="This message will be displayed on the ride page and sent out to the mailing list."
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
