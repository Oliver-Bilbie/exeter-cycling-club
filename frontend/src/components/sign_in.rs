use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;
use crate::constants::application_endpoints::APPLICATION_URL;
use crate::constants::strava_endpoints::{STRAVA_CLIENT_ID, STRAVA_OAUTH_ENDPOINT};

#[function_component(SignIn)]
pub fn set_status() -> Html {
    let redirect_uri = format!("{}/redirect", APPLICATION_URL);
    const SCOPE: &str = "read";
    const RESPONSE_TYPE: &str = "code";
    const APPROVAL_PROMPT: &str = "auto";

    let strava_authorize_url_with_params = format!(
        "{}?client_id={}&redirect_uri={}&response_type={}&approval_prompt={}&scope={}",
        STRAVA_OAUTH_ENDPOINT,
        STRAVA_CLIENT_ID,
        redirect_uri,
        RESPONSE_TYPE,
        APPROVAL_PROMPT,
        SCOPE
    );

    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={false} />
            <PageHeader title="Exeter Cycling Club" />

            <div class="hero texture-light is-flex-grow-5">
                <div class="container is-flex is-flex-direction-column is-justify-content-center">
                    <div class="my-6 mx-4 is-flex is-flex-direction-column">
                        <h2 class="title is-2 has-text-centered">
                            {"Sign in"}
                        </h2>
                        <a href={strava_authorize_url_with_params}>
                            <img src="/images/btn_strava_connectwith_orange.png" />
                        </a>
                    </div>
                </div>
            </div>

            <Footer />
        </section>
    }
}
