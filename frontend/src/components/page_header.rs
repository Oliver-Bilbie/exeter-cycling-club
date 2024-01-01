use yew::prelude::*;

#[function_component(PageHeader)]
pub fn page_header() -> Html {
    let images = vec![
        "images/header1.jpg",
        "images/header2.jpg",
        "images/header3.jpg",
    ];
    html! {
        <div>
            <figure class="image">
                <img src="images/header1.jpg" />
            </figure>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    pub images: Vec<&'static str>,
}

#[function_component(Carousel)]
pub fn carousel(CarouselProps { images }: &CarouselProps) -> Html {
    let images_html = images
        .iter()
        .map(|image| {
            html! {
                <div class="column">
                    <img src={ *image } />
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class="columns is-desktop">
            { images_html }
        </div>
    }
}
