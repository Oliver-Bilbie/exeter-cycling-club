use yew::prelude::*;

use crate::components::framed_image::FramedImage;
use crate::components::text_column::TextColumn;
use crate::helpers::about_us::*;

const SHOW_SMALL_DISPLAY_ONLY: &str = "is-hidden-desktop";
const SHOW_LARGE_DISPLAY_ONLY: &str = "is-hidden-mobile is-hidden-tablet-only";

#[derive(Properties, PartialEq)]
pub struct AboutSectionProps {
    pub content: AboutUsSection,
    pub image: &'static str,
    pub reverse: bool,
}

#[function_component(AboutSection)]
pub fn about_section(props: &AboutSectionProps) -> Html {
    let texture = match props.reverse {
        true => "texture-light",
        false => "texture-dark",
    };
    let id = props.content.title.replace(" ", "-").to_lowercase();

    html! {
        <section id={id} class={format!("section {}", texture)}>
            <container class="container is-widescreen is-flex">
                <div class="columns is-desktop is-centered m-1">
                    // We only want to reverse in the left-right direction, not in the
                    // up-down direction which will be used on smaller displays.
                    // Since the reactivity is handled by the css, we create two images
                    // for reversed sections and allow the css to handle which one to
                    // render for a given display size.
                    { match props.reverse {
                        true => html! {
                            <div
                                class={format!("column is-flex {}", SHOW_LARGE_DISPLAY_ONLY)}
                            >
                                <FramedImage image={props.image} />
                            </div>
                        },
                        false => html! {},
                    }}
                    <TextColumn content={props.content.clone()} reverse={props.reverse} />
                    { match props.reverse {
                        true => html! {
                            <div
                                class={format!("column is-flex {}", SHOW_SMALL_DISPLAY_ONLY)}
                            >
                                <FramedImage image={props.image} />
                            </div>
                        },
                        false => html! {
                            <div
                                class="column is-flex"
                            >
                                <FramedImage image={props.image} />
                            </div>
                        },
                    }}
                </div>
            </container>
        </section>
    }
}
