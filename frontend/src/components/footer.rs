use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::app::Route;
use crate::helpers::external_links::{STRAVA_URL, FACEBOOK_URL};
use crate::helpers::go_to_page::go_to_page;

#[function_component(Footer)]
pub fn footer() -> Html {
    let navigator = use_navigator().unwrap();

    html! {
        <footer class="footer texture-dark pt-0 pb-6">
            <div class="content has-text-centered">
                <section class="section">
                    <h5 class="has-text-light">
                        { "Sitemap" }
                    </h5>
                    <a class="has-text-light" onclick={ go_to_page(navigator.clone(), Route::Home) }>
                        <p> { "Home" } </p>
                    </a>
                    <a class="has-text-light" onclick={ go_to_page(navigator.clone(), Route::About) }>
                        <p> { "About" } </p>
                    </a>
                    <a class="has-text-light" onclick={ go_to_page(navigator.clone(), Route::RidePage) }>
                        <p> { "Upcoming ride" } </p>
                    </a>
                    <a class="has-text-light" onclick={ go_to_page(navigator.clone(), Route::Contact) }>
                        <p> { "Contact us" } </p>
                    </a>
                    <a class="has-text-light">
                        <p> { "Sign in" } </p>
                    </a>
                </section>

                <div style="height: 50px;">
                    <span class="icon is-large m-2">
                        <a href={STRAVA_URL}>
                            <img src="images/strava_logo.svg" />
                        </a>
                    </span>
                    <span class="icon is-large m-2" href={FACEBOOK_URL}>
                        <a href={FACEBOOK_URL}>
                            <img src="images/facebook_logo.svg" />
                        </a>
                    </span>
                </div>
                    
            </div>
        </footer>
    }
}
