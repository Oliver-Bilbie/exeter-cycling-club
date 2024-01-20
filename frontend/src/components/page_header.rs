use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PageHeaderProps {
    pub title: &'static str,
}

#[function_component(PageHeader)]
pub fn page_header(props: &PageHeaderProps) -> Html {
    html! {
        <section class="hero" style="height: 300px; position: relative;">
            <div class="hero-body fullheight-bg-2 is-vcentered">
                <h1 class="is-hidden-mobile title is-1 has-text-light" style="top: 150px; max-width: 32%; z-index: 10;">
                        { props.title }
                </h1>
                <h1 class="is-hidden-tablet title is-1 has-text-light" style="top: 150px; z-index: 10;">
                        { props.title }
                </h1>
            </div>
            <div class="is-hidden-tablet" style="height: 300px; width: 100%; position: absolute; top: 0; left: 0; background-color: rgba(103, 103, 103, 0.6);" />
            <div class="is-hidden-mobile" style="height: 300px; width: 100%; position: absolute; top: 0; left: 0;">
                <img src="/images/overlay.png" style="height: 300px; width: 100%;" />
            </div>
        </section>
    }
}
