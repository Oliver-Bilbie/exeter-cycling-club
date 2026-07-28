use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;
use crate::helpers::go_to_page::go_to_page;
use crate::Route;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    let navigator = use_navigator().unwrap();

    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={false} />
            <PageHeader title="Exeter Cycling Club" />

            <div class="hero texture-light is-flex-grow-5">
                <div class="container page-center">
                    <div class="page-center-stack my-6 mx-4">
                        <h2 class="title is-2 has-text-centered">
                            {"Page not found"}
                        </h2>
                        <button
                            class="button is-primary m-2"
                            onclick={
                                go_to_page(navigator.clone(), Route::Home)
                            }
                        >
                            { "Home" }
                        </button>
                    </div>
                </div>
            </div>

            <Footer />
        </section>
    }
}
