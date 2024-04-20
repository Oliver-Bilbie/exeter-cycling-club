use yew::prelude::*;

use crate::helpers::about_us::*;

#[derive(Properties, PartialEq)]
pub struct TextColumnProps {
    pub content: AboutUsSection,
    pub reverse: bool,
}

#[function_component(TextColumn)]
pub fn text_column(props: &TextColumnProps) -> Html {
    let text_color = match props.reverse {
        true => "has-text-dark",
        false => "has-text-light",
    };
    html! {
        <div
            class="column"
        >
            <h1 class={format!("title is-1 has-text-centered {}", text_color)}>
                { props.content.title }
            </h1>
            {
                props.content.body.iter().filter(|p| p.len() > 0).map(|p| html! {
                    <p class={format!("has-text-centered m-4 {}", text_color)}>
                        { *p }
                    </p>
                }).collect::<Html>()
            }
        </div>
    }
}
