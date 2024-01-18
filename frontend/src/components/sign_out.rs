use bounce::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::loading_spinner::LoadingSpinner;
use crate::helpers::auth_state::AuthState;
use crate::Route;

#[function_component(SignOut)]
pub fn sign_out() -> Html {
    let auth_setter = use_atom_setter::<AuthState>();
    let navigator = use_navigator().unwrap();

    use_effect(move || {
        auth_setter(AuthState {
            user_data: None,
        });
        navigator.push(&Route::Home);
    });

    html! {
        <section class="hero texture-light is-fullheight">
            <div class="container is-vcentered is-flex is-flex-direction-column is-justify-content-center mb-6" style="display: grid;">
                <LoadingSpinner size={200} />
            </div>
        </section>
    }
}
