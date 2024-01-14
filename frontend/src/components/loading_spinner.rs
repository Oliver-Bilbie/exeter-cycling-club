use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoadingSpinnerProps {
    pub size: u8,
}

#[function_component(LoadingSpinner)]
pub fn loading_spinner(props: &LoadingSpinnerProps) -> Html {
    html! {
        <figure class={format!("image is-{}x{}", props.size, props.size)}>
            <img class="is-rounded spin" src="images/logo.png" />
        </figure>
    }
}
