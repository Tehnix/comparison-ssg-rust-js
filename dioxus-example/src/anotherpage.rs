#![allow(non_snake_case)]
use dioxus::prelude::*;
use web_sys::window;

#[component]
pub fn AnotherPage() -> Element {
    let mut window_height = use_signal::<Option<f64>>(|| None);
    let window_text = use_memo(move || {
        window_height()
            .map(|w| format!(": {}", w))
            .unwrap_or("".to_owned())
    });

    // Ensure that the window object is available before calling browser APIs.
    use_effect(move || {
        let window = window();
        if let Some(w) = window {
            window_height.set(w.inner_height().ok().map(|w| w.as_f64().unwrap()));
        }
    });
    rsx! {
        h1 { "Another Page" }
        p { "This is a another page calling browser APIs{window_text}" }
    }
}
