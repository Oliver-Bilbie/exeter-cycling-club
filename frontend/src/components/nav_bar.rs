use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::app::Route;
use crate::helpers::go_to_page::go_to_page;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_sticky: bool,
}

#[function_component(NavBar)]
pub fn nav_bar(props: &Props) -> Html {
    let menu_open = use_state(|| false);
    let navigator = use_navigator().unwrap();

    let nav_styles = match props.is_sticky {
        true => "height: 52px; position: -webkit-sticky; position: sticky; top: 0;",
        false => "height: 52px;",
    };

    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation" style={nav_styles}>
            <div class="navbar-brand">
                <a class="navbar-item pt-0" onclick={ go_to_page(navigator.clone(), Route::Home) }>
                    <img class="logo-button" src="images/logo.png" style="max-height: 120px; align-self: start;" />
                </a>

                <a
                    role="button"
                    class={
                        match *menu_open {
                            true => "navbar-burger is-active",
                            false => "navbar-burger",
                        }
                    }
                    aria-label="menu"
                    aria-expanded={
                        match *menu_open {
                            true => "true",
                            false => "false",
                        }
                    }
                    data-target="navbarBasicExample"
                    onclick={
                        let menu_open = menu_open.clone();
                        Callback::from(move |_| {
                            menu_open.set(!*menu_open);
                        })
                    }
                >
                <span aria-hidden="true" />
                <span aria-hidden="true" />
                <span aria-hidden="true" />
                </a>
            </div>

            <div
                id="navbarBasicExample"
                class={
                    match *menu_open {
                        true => "navbar-menu is-active",
                        false => "navbar-menu",
                    }
                }
            >
                <div class="navbar-start">
                    <a class="navbar-item" onclick={ go_to_page(navigator.clone(), Route::Home) }>
                        { "Home" }
                    </a>

                    <a class="navbar-item" onclick={ go_to_page(navigator.clone(), Route::About) }>
                        { "About Us" }
                    </a>

                    <a class="navbar-item" onclick={ go_to_page(navigator.clone(), Route::RidePage) }>
                        { "Upcoming Ride" }
                    </a>

                    <a class="navbar-item" onclick={ go_to_page(navigator.clone(), Route::Contact) }>
                        { "Contact Us" }
                    </a>

                </div>

                <div class="navbar-end">
                    <div class="navbar-item">
                        <div class="buttons">
                            <a class="button is-primary">
                                <strong>{ "Sign up" }</strong>
                            </a>
                            <a class="button is-light">
                                { "Log in" }
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}
