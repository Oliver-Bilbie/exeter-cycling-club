use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::app::Route;
use crate::components::footer::Footer;
use crate::components::nav_bar::NavBar;
use crate::helpers::go_to_page::go_to_page;

#[function_component(ContactUs)]
pub fn contact_us() -> Html {
    let navigator = use_navigator().unwrap();

    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={true} />

            <div class="hero-body texture-light">
                <h1 class="title is-1 has-text-light">
                        { "Contact us" }
                </h1>
                <button
                    class="button is-primary m-2"
                    onclick={
                        go_to_page(navigator.clone(), Route::Home)
                    }
                >
                    { "Home" }
                </button>
            </div>

            <Footer />
        </section>
    }
}
