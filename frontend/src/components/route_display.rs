use yew::prelude::*;

use crate::helpers::get_route::RouteData;

#[derive(Properties, PartialEq)]
pub struct RouteDataProps {
    pub route_data: RouteData,
}

#[function_component(RouteDisplay)]
pub fn route_display(props: &RouteDataProps) -> Html {
    let route_data = props.route_data.clone();
    html! {
        <div class="container columns is-desktop is-vcentered mb-6">
            <div class="column has-text-centered my-4">
                <h1 class="title is-1">{route_data.name}</h1>
                {route_data.message.split("$NEWLINE").map(
                    |paragraph| html! {
                        <p class="is-4 m-3" style="word-wrap: break-word;">
                            {paragraph}
                        </p>
                    })
                    .collect::<Html>()}
            </div>
            <div class="column">
                <iframe
                    class="strava-embed-placeholder"
                    data-embed-type="route"
                    data-embed-id={route_data.id}
                    data-units="metric"
                    data-full-width="true"
                    data-style="standard"
                />
            </div>
            <script async={true} src="https://strava-embeds.com/embed.js" />
        </div>
    }
}
