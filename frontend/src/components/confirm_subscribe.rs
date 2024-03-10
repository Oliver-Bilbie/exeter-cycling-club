use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::loading_spinner::LoadingSpinner;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;
use crate::helpers::request_confirm_subscribe::{
    request_confirm_subscribe, ConfirmSubscribeStatus,
};

#[derive(Serialize, Deserialize)]
struct ConfirmSubscribeQuery {
    id: String,
}

#[function_component(Confirm)]
pub fn confirm() -> Html {
    let location = use_location();
    let user_id = match location {
        Some(location) => location
            .query::<ConfirmSubscribeQuery>()
            .unwrap_or(ConfirmSubscribeQuery { id: String::new() }),
        None => ConfirmSubscribeQuery { id: String::new() },
    };

    let confirm_subscribe_status = use_state_eq(|| ConfirmSubscribeStatus::Loading);

    {
        let confirm_subscribe_status = confirm_subscribe_status.clone();
        let user_id = user_id.id.clone();
        let status_callback = Callback::from(move |response: ConfirmSubscribeStatus| {
            confirm_subscribe_status.set(response)
        });

        // Request confirmation only once
        use_effect_with(user_id.clone(), move |_| {
            if user_id.is_empty() {
                status_callback.emit(ConfirmSubscribeStatus::Failure);
            } else {
                spawn_local(async move {
                    let resp = request_confirm_subscribe(user_id).await;
                    status_callback.emit(resp);
                });
            }
        });
    }

    let page_body = {
        move |confirm_subscribe_status: &ConfirmSubscribeStatus| match confirm_subscribe_status {
            ConfirmSubscribeStatus::Success => html! {
                <h2 class="title is-2 has-text-centered">
                    {"Subscription confirmed successfully.\nYou will now receive emails about upcoming rides."}
                </h2>
            },
            ConfirmSubscribeStatus::Failure => html! {
                <h2 class="title is-2 has-text-centered">
                    {"Unable to confirm your subscription.\nPlease try subscribing again, or contact us if this persists."}
                </h2>
            },
            ConfirmSubscribeStatus::Loading => html! {
                <div class="container is-vcentered mb-6" style="display: grid;">
                    <LoadingSpinner size={200} />
                </div>
            },
        }
    };

    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={false} />
            <PageHeader title="Exeter Cycling Club" />

            <div class="hero texture-light is-flex-grow-5">
                <div class="container is-flex is-flex-direction-column is-justify-content-center">
                    <div class="my-6 mx-4 is-flex is-flex-direction-column">
                        {page_body(&confirm_subscribe_status)}
                    </div>
                </div>
            </div>

            <Footer />
        </section>
    }
}
