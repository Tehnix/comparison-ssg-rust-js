use leptos::*;
use leptos_dom::window;
use wasm_bindgen::JsValue;

#[component]
pub fn AnotherPage() -> impl IntoView {
    let (window_height, set_window_height) = create_signal::<Option<JsValue>>(None);
    let window_text = move || {
        if let Some(w) = window_height().map(|w| w.as_f64().unwrap()) {
            format!(": {w}")
        } else {
            "".to_owned()
        }
    };

    // Ensure that the window object is available before calling browser APIs.
    create_effect(move |_| {
        let window = window();
        set_window_height(window.inner_height().ok());
    });

    view! {
        <h1>"Another Page"</h1>
        <p>"This is a another page calling browser APIs" {window_text}</p>
    }
}
