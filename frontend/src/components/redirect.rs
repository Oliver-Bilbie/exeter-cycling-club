use serde::{Deserialize, Serialize};
use bounce::prelude::*;
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::loading_spinner::LoadingSpinner;
use crate::helpers::handle_auth::{handle_auth, UserData};
use crate::helpers::auth_state::AuthState;

#[derive(Serialize, Deserialize)]
struct OauthParameters {
    state: String,
    code: String,
    scope: String,
}

#[function_component(Redirect)]
pub fn redirect() -> Html {
    let auth_state = use_atom::<AuthState>();
    let location = use_location();
    let oauth_params = match location {
        Some(location) => location.query::<OauthParameters>().unwrap_or(
            OauthParameters { state: String::new(), code: String::new(), scope: String::new() },
        ),
        None => OauthParameters { state: String::new(), code: String::new(), scope: String::new() },
    };
    let is_loading = use_state(|| true);

    {
        let auth_state = auth_state.clone();
        let oauth_code = oauth_params.code.clone();
        let is_loading = is_loading.clone();
        let status_callback =
            Callback::from(move |response: Result<UserData, String>| {
                match response {
                    Ok(resp) => {
                        auth_state.set(AuthState { user_data: Some(resp) });
                        is_loading.set(false);
                    },
                    Err(_) => {
                        auth_state.set(AuthState { user_data: None });
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
                                {format!(
                                    "Welcome, {}!",
                                    match auth_state.user_data {
                                        Some(ref user_data) => user_data.name.clone(),
                                        None => String::from("unknown user"),
                                    }
                                )}
                            </h1>
                        }}}
                    </div>
                </div>
            </div>
        </section>
    }
}
