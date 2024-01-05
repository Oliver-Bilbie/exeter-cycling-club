use regex::Regex;
use yew::prelude::*;

use crate::components::footer::Footer;
use crate::components::nav_bar::NavBar;
use crate::components::page_header::PageHeader;

#[function_component(ContactUs)]
pub fn contact_us() -> Html {
    let email = use_state(|| "".to_string());
    let message = use_state(|| "".to_string());

    html! {
        <section class="hero is-fullheight">
            <NavBar is_sticky={false} />
            <PageHeader title="Contact us" />

            <div class="hero texture-light">
                <div class="container">
                    <form class="my-6 mx-4">
                        <div class="field">
                            <label class="label is-size-5">{ "Your email address" }</label>
                            <div class="control has-icons-left has-icons-right">
                                <input class="input is-medium" type="email" placeholder="Email" />
                                <span class="icon is-small is-left">
                                    <i class="fas fa-envelope" />
                                </span>
                            </div>
                        </div>

                        <div class="field">
                            <label class="label is-size-5">{ "Message" }</label>
                            <div class="control">
                                <textarea class="textarea has-fixed-size is-medium" placeholder="Message" style="height: 300px;" />
                            </div>
                        </div>

                        <div class="has-text-centered">
                            <button class="button is-primary is-medium is-align-self-center" type="submit">
                                { "Send" }
                            </button>
                        </div>
                    </form>
                </div>
            </div>

            <Footer />
        </section>
    }
}

fn validate_email(email: String) -> bool {
    match Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$") {
        Ok(re) => re.is_match(&email),
        Err(_) => false,
    }
}
