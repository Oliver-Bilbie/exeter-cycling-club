use bounce::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::helpers::auth_state::AuthState;
use crate::helpers::go_to_page::go_to_page;
use crate::Route;

pub const NAVBAR_HEIGHT: f64 = 52.0;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub is_sticky: bool,
}

#[function_component(NavBar)]
pub fn nav_bar(props: &Props) -> Html {
    let auth_state = use_atom_value::<AuthState>();
    let navigator = use_navigator().unwrap();
    let menu_open = use_state(|| false);

    let nav_styles = match props.is_sticky {
        true => format!(
            "height: {}px; position: -webkit-sticky; position: sticky; top: 0;",
            NAVBAR_HEIGHT
        ),
        false => format!("height: {}px;", NAVBAR_HEIGHT),
    };

    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation" style={nav_styles}>
            <div class="navbar-brand">
                <a
                    // For formatting purposes the logo is hidden on small displays when the menu is open.
                    // This is handled through CSS so that the logo does not disappear when the
                    // menu is opened and then the screen is resized to a larger size.
                    class={ match *menu_open {
                        true => "navbar-item is-hidden-touch pt-0",
                        false => "navbar-item pt-0",
                    }}
                    onclick={ go_to_page(navigator.clone(), Route::Home) }
                >
                    <img
                        class="logo-button"
                        src="/images/logo.png"
                        rel="preload"
                        alt="Home"
                    />
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
                class={
                    match *menu_open {
                        true => "navbar-menu is-active",
                        false => "navbar-menu",
                    }
                }
            >
                <div class="navbar-start">
                    <a class="navbar-item" onclick={ go_to_page(navigator.clone(), Route::About) }>
                        { "About Us" }
                    </a>
                    <a class="navbar-item" onclick={ go_to_page(navigator.clone(), Route::RidePage) }>
                        { "Upcoming Ride" }
                    </a>
                    <a class="navbar-item" onclick={ go_to_page(navigator.clone(), Route::Contact) }>
                        { "Contact Us" }
                    </a>

                    {match auth_state.user_data {
                        Some(ref user_data) => {
                            if user_data.admin {
                                html! {
                                    <>
                                        <a class="navbar-item" onclick={ go_to_page(navigator.clone(), Route::RouteSelect) }>
                                            { "Set route" }
                                        </a>
                                        <a class="navbar-item" onclick={ go_to_page(navigator.clone(), Route::RouteCancel) }>
                                            { "Cancel ride" }
                                        </a>
                                    </>
                                }
                            } else {
                                html! {}
                            }
                        },
                        None => html! {}
                    }}
                </div>

                <div class="navbar-end">
                    <div class="navbar-item is-flex is-flex-direction-row is-align-items-center">
                        {match auth_state.user_data {
                            Some(ref user_data) => html! {
                                <>
                                    <strong class="is-vcentered m-2">{ user_data.name.clone() }</strong>
                                    <a class="button is-light m-2" onclick={ go_to_page(navigator.clone(), Route::SignOut) }>
                                        { "Sign out" }
                                    </a>
                                </>
                            },
                            None => html! {
                                <a class="button is-primary" onclick={ go_to_page(navigator.clone(), Route::SignIn) }>
                                    <strong>{ "Sign in" }</strong>
                                </a>
                            }
                        }}
                    </div>
                </div>
            </div>
        </nav>
    }
}
