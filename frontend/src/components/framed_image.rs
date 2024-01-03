use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FramedImageProps {
    pub image: &'static str,
}

#[function_component(FramedImage)]
pub fn framed_image(props: &FramedImageProps) -> Html {
    html! {
        <div class="box p-2 has-background-dark" style="display: flex;">
            <img src={props.image} style="object-fit: cover;" />
        </div>
    }
}
