use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::about_section::AboutSection;
use crate::components::footer::Footer;
use crate::components::nav_bar::{NavBar, NAVBAR_HEIGHT};
use crate::helpers::about_us::*;
use crate::helpers::go_to_page::go_to_page;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub header_visible: bool,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let navigator = use_navigator().unwrap();
    let header_visible = props.header_visible;

    let scroll_to_about_us = |_| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let about_us = document.get_element_by_id("about-us").unwrap();

        let about_us_top = about_us.get_bounding_client_rect().top();
        let y_offset = window.scroll_y().unwrap();

        let scroll_options = &web_sys::ScrollToOptions::new();
        scroll_options.set_top(about_us_top + y_offset - NAVBAR_HEIGHT);
        scroll_options.set_behavior(web_sys::ScrollBehavior::Smooth);
        window.scroll_with_scroll_to_options(scroll_options);
    };

    html! {
        <>
            // Speed up image loading by adding it to the html
            <img src="/images/header1.jpg" rel="preload" class="is-hidden" />

            <section class="is-fullheight">
                <NavBar is_sticky={header_visible} />

                if header_visible {
                    <section
                        class="fullheight-bg hero is-fullheight-with-navbar"
                        style="overflow: hidden;"
                    >
                        <div class="is-overlay hero-body">
                            <div class="container has-text-centered">
                                <h1 class="title is-1 has-text-light">
                                        { "Exeter Cycling Club" }
                                </h1>
                                <h3 class="subtitle is-4 has-text-light">
                                    { "Group rides in East Devon and Dartmoor" }
                                </h3>
                                <div class="block">
                                    <button
                                        class="button is-primary m-2"
                                        onclick={ scroll_to_about_us }
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
        </>
    }
}
