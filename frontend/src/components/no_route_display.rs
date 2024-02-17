use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NoRouteDisplayProps {
    pub message: String,
}

#[function_component(NoRouteDisplay)]
pub fn no_route_display(props: &NoRouteDisplayProps) -> Html {
    html! {
        <div class="container is-vcentered mb-6">
            {props.message.split("$NEWLINE").map(|paragraph| html! {
                <h2 class="title is-2 has-text-centered">
                    {paragraph}
                </h2>
            }).collect::<Html>()}
        </div>
    }
}
