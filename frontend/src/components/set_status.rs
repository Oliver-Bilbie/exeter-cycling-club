use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::loading_spinner::LoadingSpinner;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;
use crate::helpers::put_status::{put_status, AttendanceStatus};

#[derive(Serialize, Deserialize)]
struct SetStatusQuery {
    id: String,
    status: String,
}

#[function_component(SetStatus)]
pub fn set_status() -> Html {
    let location = use_location();
    let status_query = match location {
        Some(location) => location
            .query::<SetStatusQuery>()
            .unwrap_or(SetStatusQuery {
                id: String::new(),
                status: String::new(),
            }),
        None => SetStatusQuery {
            id: String::new(),
            status: String::new(),
        },
    };

    let request_status = use_state_eq(|| AttendanceStatus::Loading);

    {
        let request_status = request_status.clone();
        let user_id = status_query.id.clone();
        let user_status = status_query.status.clone();
        let status_callback =
            Callback::from(move |response: AttendanceStatus| request_status.set(response));

        // Set status only once
        use_effect_with(user_id.clone(), move |_| {
            if validate_put_status_request(&user_id, &user_status) {
                spawn_local(async move {
                    let resp = put_status(user_id, user_status).await;
                    status_callback.emit(resp);
                });
            } else {
                status_callback.emit(AttendanceStatus::Failure);
            }
        });
    }

    let page_body = {
        move |request_status: &AttendanceStatus| match request_status {
            AttendanceStatus::Success => html! {
                <h2 class="title is-2 has-text-centered">
                    { match status_query.status.as_str() {
                        "Y" => "Attendance confirmed, see you there!",
                        "N" => "Attendance cancelled, see you next time!",
                        "M" => "Tenative attendance confirmed, hopefully see you there!",
                        _ => "Attendance status updated!",
                    }}
                </h2>
            },
            AttendanceStatus::Failure => html! {
                <h2 class="title is-2 has-text-centered">
                    {"An error occurred while trying to update your status. Please try again later, or contact us if this persists."}
                </h2>
            },
            AttendanceStatus::Loading => html! {
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
                        {page_body(&request_status)}
                    </div>
                </div>
            </div>

            <Footer />
        </section>
    }
}

fn validate_put_status_request(id: &String, status: &String) -> bool {
    let valid_statuses = vec!["Y", "N", "M"];
    let status_is_valid = valid_statuses.iter().any(|s| s == &status);
    let id_is_valid = id.len() == 36;
    status_is_valid && id_is_valid
}
