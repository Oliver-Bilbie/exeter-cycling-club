use yew::prelude::*;

use crate::components::framed_image::FramedImage;
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
            <div class="container columns is-desktop">
                // We only want to reverse in the left-right direction, not in the
                // up-down direction which will be used on smaller displays.
                // Since the reactivity is handled by the css, we create two images
                // for reversed sections and allow the css to handle which one to
                // render for a given display size.
                { match props.reverse {
                    true => html! {
                        <div class={format!("column {}", SHOW_LARGE_DISPLAY_ONLY)} style="display: flex;">
                            <FramedImage image={props.image} />
                        </div>
                    },
                    false => html! {},
                }}
                <TextColumn content={props.content.clone()} reverse={props.reverse} />
                { match props.reverse {
                    true => html! {
                        <div class={format!("column {}", SHOW_SMALL_DISPLAY_ONLY)} style="display: flex;">
                            <FramedImage image={props.image} />
                        </div>
                    },
                    false => html! {
                        <div class="column" style="display: flex;">
                            <FramedImage image={props.image} />
                        </div>
                    },
                }}
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct TextColumnProps {
    content: AboutUsSection,
    reverse: bool,
}

#[function_component(TextColumn)]
fn text_column(props: &TextColumnProps) -> Html {
    let text_color = match props.reverse {
        true => "has-text-dark",
        false => "has-text-light",
    };
    html! {
        <div class="column">
            <h1 class={format!("title is-1 has-text-centered {}", text_color)}>
                { props.content.title }
            </h1>
            {
                props.content.body.iter().filter(|p| p.len() > 0).map(|p| html! {
                    <p class={format!("has-text-centered m-2 {}", text_color)}>
                        { *p }
                    </p>
                }).collect::<Html>()
            }
        </div>
    }
}
