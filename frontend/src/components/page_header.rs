use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    pub title: &'static str,
}

#[function_component(PageHeader)]
pub fn page_header(props: &PageHeaderProps) -> Html {
    html! {
        <>
            // Speed up image loading by adding it to the html
            <img src="/images/header2.webp" rel="preload" class="is-hidden" />

            <section class="hero" style="height: 20vh; max-height: 300px; min-height: 200px; position: relative;">
                <div class="hero-body fullheight-bg-2 is-vcentered" style="height: 20vh; max-height: 300px; min-height: 200px;">
                    <h1 class="title is-1 has-text-light" style="top: 200px; z-index: 10;">
                            { props.title }
                    </h1>
                </div>
                <div class="is-hidden-tablet" style="height: 20vh; max-height: 300px; min-height: 200px; width: 100%; position: absolute; top: 0; left: 0; background-color: rgba(103, 103, 103, 0.6);" />
                <div class="is-hidden-mobile" style="height: 20vh; max-height: 300px; min-height: 200px; width: 100%; position: absolute; top: 0; left: 0; background: linear-gradient(90deg, rgba(103,103,103,0.6) 0%, rgba(103,103,103,0.6) 30%, rgba(103,103,103,0) 70%);" />
            </section>
        </>
    }
}
