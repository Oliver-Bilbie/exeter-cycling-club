use yew::prelude::*;
use yew_router::prelude::*;

use crate::constants::external_links::{FACEBOOK_URL, STRAVA_URL};
use crate::helpers::go_to_page::go_to_page;
use crate::Route;

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
                    <a class="has-text-light" onclick={ go_to_page(navigator.clone(), Route::About) }>
                        <p> { "About us" } </p>
                    </a>
                    <a class="has-text-light" onclick={ go_to_page(navigator.clone(), Route::RidePage) }>
                        <p> { "Upcoming ride" } </p>
                    </a>
                    <a class="has-text-light" onclick={ go_to_page(navigator.clone(), Route::Contact) }>
                        <p> { "Contact us" } </p>
                    </a>
                    <a class="has-text-light" onclick={ go_to_page(navigator.clone(), Route::SignIn) }>
                        <p> { "Sign in" } </p>
                    </a>
                </section>

                <div style="height: 50px;">
                    <span class="icon light-icon is-large m-2">
                        <a href={STRAVA_URL} target="_blank">
                            <img src="/images/strava_logo.svg" loading="lazy" alt="View our Strava group" />
                        </a>
                    </span>
                    <span class="icon light-icon is-large m-2">
                        <a href={FACEBOOK_URL} target="_blank">
                            <img src="/images/facebook_logo.svg" loading="lazy" alt="View our Facebook page" />
                        </a>
                    </span>
                </div>
            </div>
        </footer>
    }
}
