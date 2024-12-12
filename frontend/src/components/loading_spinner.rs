use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoadingSpinnerProps {
    pub size: u16,
}

#[function_component(LoadingSpinner)]
pub fn loading_spinner(props: &LoadingSpinnerProps) -> Html {
    html! {
        <figure
            class={format!("image is-{}x{}", props.size, props.size)}
            style={format!("justify-self: center; max-width: {}px; max-height: {}px;", props.size, props.size)}
        >
            <img class="is-rounded spin" src="/images/logo.png" alt="Loading" />
        </figure>
    }
}
