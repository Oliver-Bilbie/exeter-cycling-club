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
    let route_name = props.route_data.name.clone();
    let map_url = props.route_data.map_urls.url.clone();
    let display_distance = format!("{:.0} km", props.route_data.distance / 1000.0);
    let display_elevation_gain = format!("{:.0} m", props.route_data.elevation_gain);
    let date_without_time = props
        .route_data
        .updated_at
        .split("T")
        .collect::<Vec<&str>>()[0];
    let display_date = format!("Updated: {}", date_without_time);

    html! {
        <div class="card" style="max-width: 350px; display: inline-grid;">
            <div class="card-image">
                <figure class="image is-350x146">
                    <img src={ map_url } />
                </figure>
            </div>
            <header class="card-header">
                <h4 class="card-header-title">{ route_name }</h4>
            </header>
            <div class="card-content m-0 p-0" style="align-self: end;">
                <div class="media is-align-items-center m-2">
                    <div class="media-left">
                        <figure class="image is-32x32">
                            <img src="/images/road_icon.svg" alt="Placeholder image"/>
                        </figure>
                    </div>
                    <div class="media-content">
                        <strong class="is-5">{ display_distance }</strong>
                    </div>
                    <div class="m-3" />
                    <div class="media-left">
                        <figure class="image is-32x32">
                            <img src="/images/mountain_icon.svg" alt="Placeholder image"/>
                        </figure>
                    </div>
                    <div class="media-content">
                        <strong class="is-5">{ display_elevation_gain }</strong>
                    </div>
                </div>
                <div class="media m-2 pt-2">
                    <time>{ display_date }</time>
                </div>
            </div>
        </div>
    }
}
