use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RouteFormProps {
    pub initial_name: String,
    pub on_submit: Callback<RouteFormData>,
}

pub struct RouteFormData {
    pub name: String,
    pub message: String,
}

#[function_component(RouteForm)]
pub fn route_form(props: &RouteFormProps) -> Html {
    let initial_name = props.initial_name.clone();

    let handle_submit = {
        let initial_name = props.initial_name.clone();
        let on_submit = props.on_submit.clone();

        Callback::from(move |_| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            let name_element = document.get_element_by_id("name-input");
            let name_input = name_element.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            let name_value = match name_input {
                Some(name) => name.value(),
                None => initial_name.clone(),
            };

            let message_element = document.get_element_by_id("message-input");
            let message_input =
                message_element.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            let message_value = match message_input {
                Some(message) => message.value(),
                None => String::new(),
            };

            on_submit.emit(RouteFormData {
                name: name_value,
                message: message_value,
            });
        })
    };

    html!(
        <form>
            <div class="field">
                <label class="label is-size-5">
                    {"Ride name"}
                </label>
                <div class="control">
                    <input
                        id="name-input"
                        class="input is-medium"
                        value={initial_name}
                        placeholder="Ride name"
                        type="text"
                    />
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
