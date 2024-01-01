mod components;
mod helpers;

use crate::components::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
