use web_sys::{window, ScrollBehavior, ScrollToOptions};

pub fn scroll_to_top() {
    let window = window().unwrap();
    window.scroll_to_with_scroll_to_options(
        ScrollToOptions::new()
            .top(0.0)
            .behavior(ScrollBehavior::Instant),
    );
}
