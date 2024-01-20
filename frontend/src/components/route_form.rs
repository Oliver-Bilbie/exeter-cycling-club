use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;

use crate::helpers::set_route::SetRouteData;

#[derive(Properties, PartialEq)]
pub struct RouteFormProps {
    pub route_data: UseStateHandle<SetRouteData>,
    pub on_submit: Callback<()>,
}

#[function_component(RouteForm)]
pub fn route_form(props: &RouteFormProps) -> Html {
    let handle_update_name = {
        let route_data = props.route_data.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let user_input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
            if let Some(input) = user_input {
                route_data.set(SetRouteData {
                    id: route_data.id.clone(),
                    name: input.value(),
                    message: route_data.message.clone(),
                    access_token: route_data.access_token.clone(),
                });
            }
        })
    };

    let handle_update_message = {
        let route_data = props.route_data.clone();
        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let user_input = target.and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok());
            if let Some(input) = user_input {
                route_data.set(SetRouteData {
                    id: route_data.id.clone(),
                    name: route_data.name.clone(),
                    message: input.value(),
                    access_token: route_data.access_token.clone(),
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
                    {"Ride name"}
                </label>
                <div class="control">
                    <input
                        class="input is-medium"
                        onchange={handle_update_name}
                        value={props.route_data.name.clone()}
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
