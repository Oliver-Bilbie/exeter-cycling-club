use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CancelFormProps {
    pub on_submit: Callback<String>,
}

#[function_component(CancelForm)]
pub fn cancel_form(props: &CancelFormProps) -> Html {
    let handle_submit = {
        let on_submit = props.on_submit.clone();

        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let message_element = document.get_element_by_id("message-input");
            let message_input =
                message_element.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            let message_value = match message_input {
                Some(message) => message.value(),
                None => String::new(),
            };

            on_submit.emit(message_value);
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
                        id="message-input"
                        class="textarea has-fixed-size is-medium"
                        placeholder="This message will be displayed on the ride page and sent out to the mailing list."
                        style="height: 400px;"
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
