use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::loading_spinner::LoadingSpinner;
use crate::helpers::handle_auth::{handle_auth, UserData};

// http://localhost:8000/redirect?state=&code=643368b762e18fb4dcdd99183ae91f1348ea91ca&scope=read
#[derive(Serialize, Deserialize)]
struct OauthParameters {
    state: String,
    code: String,
    scope: String,
}

#[function_component(Redirect)]
pub fn redirect() -> Html {
    let location = use_location();
    let oauth_params = match location {
        Some(location) => location.query::<OauthParameters>().unwrap_or(
            OauthParameters { state: String::new(), code: String::new(), scope: String::new() },
        ),
        None => OauthParameters { state: String::new(), code: String::new(), scope: String::new() },
    };

    // TODO: Remove this
    let user_data = use_state(|| UserData {
        id: String::new(),
        name: String::new(),
        access_token: String::new(),
        admin: false,
    });

    let is_loading = use_state(|| true);

    {
        // TODO: Remove this
        let user_data = user_data.clone();

        let oauth_code = oauth_params.code.clone();
        let is_loading = is_loading.clone();
        let status_callback =
            Callback::from(move |response: Result<UserData, String>| {
                match response {
                    Ok(resp) => {
                        user_data.set(resp);
                        is_loading.set(false);
                    },
                    Err(_) => {
                        is_loading.set(false);
                    },
                }
            });

        // Authenticate only once
        use_effect_with(oauth_code.clone(), move |_| {
            if oauth_code.is_empty() {
                status_callback.emit(Err(String::from("No authentication code provided.")));
            } else {
                spawn_local(async move {
                    let resp = handle_auth(oauth_code).await;
                    status_callback.emit(resp);
                });
            }
        });
    }

    html! {
        <section class="hero is-fullheight">
            <div class="hero texture-light is-flex-grow-5">
                <div class="container is-flex is-flex-direction-column is-justify-content-center">
                    <div class="my-6 mx-4 is-flex is-flex-direction-column">
                        {if *is_loading { html! {
                            <div class="container is-vcentered mb-6" style="display: grid;">
                                <LoadingSpinner size={200} />
                            </div>
                        }} else { html! {
                            <h1 class="title is-1 has-text-centered">
                                {format!("Welcome, {}!", user_data.name)}
                            </h1>
                        }}}
                    </div>
                </div>
            </div>
        </section>
    }
}
