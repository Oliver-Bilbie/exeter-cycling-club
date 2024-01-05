use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    pub title: &'static str,
}

#[function_component(PageHeader)]
pub fn page_header(props: &PageHeaderProps) -> Html {
    html! {
        <section class="hero" style="height: 300px;">
            <div class="hero-body fullheight-bg-2 is-vcentered">
                <h1 class="title is-1 has-text-light" style="top: 150px;">
                        { props.title }
                </h1>
            </div>
        </section>
    }
}
