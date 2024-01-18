use bounce::prelude::*;
use serde::{Deserialize, Serialize};
use yew::platform::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::loading_spinner::LoadingSpinner;
use crate::helpers::auth_state::AuthState;
use crate::helpers::handle_auth::{handle_auth, UserData};
use crate::Route;

#[derive(Serialize, Deserialize)]
struct OauthParameters {
    state: String,
    code: String,
    scope: String,
}

#[function_component(Redirect)]
pub fn redirect() -> Html {
    let auth_state = use_atom::<AuthState>();
    let navigator = use_navigator().unwrap();
    let location = use_location();
    let oauth_params = match location {
        Some(location) => location
            .query::<OauthParameters>()
            .unwrap_or(OauthParameters {
                state: String::new(),
                code: String::new(),
                scope: String::new(),
            }),
        None => OauthParameters {
            state: String::new(),
            code: String::new(),
            scope: String::new(),
        },
    };

    {
        let auth_state = auth_state.clone();
        let oauth_code = oauth_params.code.clone();
        let status_callback = Callback::from(move |response: Result<UserData, String>| {
            match response {
                Ok(resp) => {
                    auth_state.set(AuthState {
                        user_data: Some(resp),
                    });
                }
                Err(_) => {
                    auth_state.set(AuthState { user_data: None });
                }
            };
            navigator.push(&Route::Home);
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
        <section class="hero texture-light is-fullheight">
            <div class="container is-vcentered is-flex is-flex-direction-column is-justify-content-center mb-6" style="display: grid;">
                <LoadingSpinner size={200} />
            </div>
        </section>
    }
}
