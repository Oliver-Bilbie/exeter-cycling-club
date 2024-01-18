use yew::prelude::*;

use crate::helpers::list_routes::RouteData;


// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// pub struct RouteData {
//     pub id_str: String,
//     pub name: String,
//     pub distance: f32,
//     pub elevation_gain: f32,
//     pub updated_at: String,
//     pub map_urls: MapUrls,
// }

// #[derive(Serialize, Deserialize, PartialEq, Debug)]
// pub struct MapUrls {
//     pub url: String,
// }

#[derive(Properties, PartialEq)]
pub struct RouteCardProps {
    pub route_data: RouteData,
}

#[function_component(RouteCard)]
pub fn route_card(props: &RouteCardProps) -> Html {
    let route_data = &props.route_data;
    html! {
        <div class="card column" style="max-width: 200px;">
            <div class="card-image">
                <figure class="image is-4by3">
                    <img src={ route_data.map_urls.url.clone() } />
                </figure>
            </div>
            <div class="card-content">
                <div class="content">
                    <p class="title is-4">{ &route_data.name }</p>
                </div>

                <div class="media">
                    <div class="media-left">
                        <figure class="image is-48x48">
                        <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                        </figure>
                    </div>
                    <div class="media-content">
                        <p class="title is-4">{ &route_data.distance }</p>
                    </div>
                </div>

                <div class="media">
                    <div class="media-left">
                        <figure class="image is-48x48">
                        <img src="https://bulma.io/images/placeholders/96x96.png" alt="Placeholder image"/>
                        </figure>
                    </div>
                    <div class="media-content">
                        <p class="title is-4">{ &route_data.elevation_gain }</p>
                    </div>
                </div>
                
                <div class="content">
                    <time>{ &route_data.updated_at }</time>
                </div>
            </div>
        </div>
    }
}
