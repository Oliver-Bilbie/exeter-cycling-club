// TODO: Retrieve route data on page load

use yew::prelude::*;

use crate::components::email_sign_up::EmailSignUp;
use crate::components::footer::Footer;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;

#[function_component(RidePage)]
pub fn ride_page() -> Html {
    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={false} />
            <PageHeader title="Upcoming ride" />

            <section class="section texture-light pt-8 is-flex-grow-5">
                <div class="container columns is-desktop is-vcentered mb-6">
                    <div class="column has-text-centered my-4">
                        <h1 class="title is-1">{ "Culmstock, Dunkeswell" }</h1>
                        <p>{ "Some text can go here to describe the ride." }</p>
                        <p>{ "Something about meeting at Sainsbury's at 8 I guess." }</p>
                    </div>
                    <div class="column">
                        <iframe
                            class="strava-embed-placeholder"
                            data-embed-type="route"
                            data-embed-id="3177284926916691964"
                            data-units="metric"
                            data-full-width="true"
                            data-style="standard"
                        />
                    </div>
                </div>

                <EmailSignUp />
            </section>

            <Footer />
            <script async={true} src="https://strava-embeds.com/embed.js" />
        </section>
    }
}
