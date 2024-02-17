use exeter_cycling_club::App;

fn main() {
    #[cfg(target_arch = "wasm32")]
    yew::Renderer::<App>::new().hydrate();
}
