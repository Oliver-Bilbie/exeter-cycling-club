use yew::prelude::*;

use crate::components::notification::Notification;

#[derive(Clone)]
struct NotificationData {
    message: String,
    color: String,
    visible: bool,
}

enum FormState {
    Ready,
    Loading,
    Complete,
}

#[function_component(EmailSignUp)]
pub fn email_sign_up() -> Html {
    // let form_data = use_state(|| MessageData {
    //     email: "".to_string(),
    //     message: "".to_string(),
    // });

    // let notification_data = use_state(|| NotificationData {
    //     message: "".to_string(),
    //     color: "".to_string(),
    //     visible: false,
    // });

    // let form_state = use_state(|| FormState::Ready);

    html!(
        <div></div>
    )
}
