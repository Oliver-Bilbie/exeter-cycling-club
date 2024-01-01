use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::app::Route;
use crate::components::nav_bar::NavBar;
use crate::helpers::go_to_page::go_to_page;

#[function_component(Home)]
pub fn home() -> Html {
    let navigator = use_navigator().unwrap();

    html! {
        <section class="is-fullheight">
            <NavBar />

            <section class="bg hero is-fullheight-with-navbar" style="overflow: hidden;">

                <div class="is-overlay hero-body">
                    <div class="container has-text-centered">
                        <h1 class="title is-1 has-text-light">
                                {"Exeter Cycling Club" }
                        </h1>
                        <h3 class="subtitle is-4 has-text-light">
                            { "The best cycling club in the world!" }
                        </h3>
                        <div class="block">
                            <button
                                class="button is-primary m-2"
                                onclick={ 
                                    go_to_page(navigator.clone(), Route::About)
                                }
                            >
                                { "About Us" }
                            </button>
                            <button
                                class="button m-2"
                                onclick={
                                    go_to_page(navigator.clone(), Route::RidePage)
                                }
                            >
                                { "Upcoming Ride" }
                            </button>
                        </div>
                    </div>
                </div>
            </section>
        </section>
    }
}
