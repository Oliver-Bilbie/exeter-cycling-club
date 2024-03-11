use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::loading_spinner::LoadingSpinner;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;
use crate::helpers::form_state::RequestState;
use crate::helpers::request_unsubscribe::request_unsubscribe;

#[derive(Serialize, Deserialize)]
struct UnsubscribeQuery {
    id: String,
}

#[function_component(Unsubscribe)]
pub fn unsubscribe() -> Html {
    let location = use_location();
    let user_id = match location {
        Some(location) => location
            .query::<UnsubscribeQuery>()
            .unwrap_or(UnsubscribeQuery { id: String::new() }),
        None => UnsubscribeQuery { id: String::new() },
    };

    let unsubscribe_status = use_state_eq(|| RequestState::Loading);

    {
        let unsubscribe_status = unsubscribe_status.clone();
        let user_id = user_id.id.clone();
        let status_callback =
            Callback::from(move |response: RequestState| unsubscribe_status.set(response));

        // Request unsubscribe only once
        use_effect_with(user_id.clone(), move |_| {
            if user_id.is_empty() {
                status_callback.emit(RequestState::Failure);
            } else {
                spawn_local(async move {
                    let resp = request_unsubscribe(user_id).await;
                    status_callback.emit(resp);
                });
            }
        });
    }

    let page_body = {
        move |unsubscribe_status: &RequestState| match unsubscribe_status {
            RequestState::Success => html! {
                <h2 class="title is-2 has-text-centered">
                    {"You have been unsubscribed successfully"}
                </h2>
            },
            RequestState::Failure => html! {
                <h2 class="title is-2 has-text-centered">
                    {"An error occurred while trying to unsubscribe. Please try again later, or contact us if this persists."}
                </h2>
            },
            RequestState::Loading => html! {
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
                        {page_body(&unsubscribe_status)}
                    </div>
                </div>
            </div>

            <Footer />
        </section>
    }
}
