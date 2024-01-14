use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NotificationProps {
    pub message: String,
    pub color: String,
    pub on_close: Callback<MouseEvent>,
}

#[function_component(Notification)]
pub fn notification(props: &NotificationProps) -> Html {
    let on_close = props.on_close.clone();
    html! {
        <div
            class={format!("notification is-{}", props.color)}
            style="position: fixed; top: 20px; left: 5%; z-index: 999; width: 90%; border-radius: 16px;"
        >
            <button class="delete is-medium" onclick={on_close} />
            <h5 class="title is-5 has-text-centered">
                {props.message.clone()}
            </h5>
        </div>
    }
}
