use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    pub title: &'static str,
}

#[function_component(PageHeader)]
pub fn page_header(props: &PageHeaderProps) -> Html {
    html! {
        <section class="hero is-medium">
            <div class="hero-body fullheight-bg-2 is-vcentered">
                <h1 class="title is-1 has-text-light">
                        { props.title }
                </h1>
            </div>
        </section>
    }
}
