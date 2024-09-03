use web_sys::{window, ScrollBehavior, ScrollToOptions};

pub fn scroll_to_top() {
    let window = window().unwrap();

    let scroll_options = &ScrollToOptions::new();
    scroll_options.set_top(0.0);
    scroll_options.set_behavior(ScrollBehavior::Instant);
    window.scroll_to_with_scroll_to_options(scroll_options);
}
