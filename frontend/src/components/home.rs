use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::about_section::AboutSection;
use crate::components::footer::Footer;
use crate::components::nav_bar::NavBar;
use crate::helpers::about_us::*;
use crate::helpers::go_to_page::go_to_page;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub header_visible: bool,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let header_visible = use_state(|| props.header_visible);
    let navigator = use_navigator().unwrap();

    html! {
        <div>
            <section class="is-fullheight">
                <NavBar is_sticky={true} />

                if *header_visible {
                    <section class="fullheight-bg hero is-fullheight-with-navbar" style="overflow: hidden;">
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
                                            move |_| header_visible.set(false)
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
                }
            </section>

            <AboutSection content={ABOUT_US} image="images/home1.jpg" reverse={true} />
            <AboutSection content={JOIN_US_ON_A_RIDE} image="images/home2.jpg" reverse={false} />
            <AboutSection content={RIDING_GUIDELINES} image="images/home3.jpg" reverse={true} />

            <Footer />
        </div>
    }
}
