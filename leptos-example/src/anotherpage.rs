#![allow(non_snake_case)]
use leptos::*;
use leptos_dom::window;

#[component]
pub fn AnotherPage() -> impl IntoView {
    let (window_height, set_window_height) = create_signal::<Option<f64>>(None);
    let window_text = move || {
        window_height()
            .map(|w| format!(": {}", w))
            .unwrap_or("".to_owned())
    };

    // Ensure that the window object is available before calling browser APIs.
    create_effect(move |_| {
        set_window_height(window().inner_height().ok().map(|w| w.as_f64().unwrap()));
    });

    view! {
        <h1>"Another Page"</h1>
        <p>"This is a another page calling browser APIs" {window_text}</p>
    }
}
