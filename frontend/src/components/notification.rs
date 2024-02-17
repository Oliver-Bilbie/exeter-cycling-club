use bounce::prelude::*;
use yew::prelude::*;

#[derive(Atom, PartialEq, Default, Debug)]
pub struct NotificationState {
    pub visible: bool,
    pub message: String,
    pub color: String,
}

#[function_component(Notification)]
pub fn notification() -> Html {
    let notification_state = use_atom::<NotificationState>();

    let visible = notification_state.visible;

    if visible {
        let on_close = {
            let notification_state = notification_state.clone();
            Callback::from(move |_| {
                notification_state.set(NotificationState {
                    visible: false,
                    message: "".to_string(),
                    color: "".to_string(),
                });
            })
        };

        html! {
            <NotificationBody
                message={notification_state.message.clone()}
                color={notification_state.color.clone()}
                on_close={on_close.clone()}
            />
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
pub struct NotificationBodyProps {
    message: String,
    color: String,
    on_close: Callback<MouseEvent>,
}

#[function_component(NotificationBody)]
pub fn notification_body(props: &NotificationBodyProps) -> Html {
    html! {
        <div
            class={format!("notification is-{}", props.color)}
            style="position: fixed; top: 20px; left: 5%; z-index: 999; width: 90%; border-radius: 16px;"
        >
            <button class="delete is-medium" onclick={props.on_close.clone()} />
            <h5 class="title is-5 has-text-centered">
                {props.message.clone()}
            </h5>
        </div>
    }
}
