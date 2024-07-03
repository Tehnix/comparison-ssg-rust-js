#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn SubPage() -> Element {
    // Creates a reactive value to update the button
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "SubPage" }
        p { "This is a subpage with interactivity: {count}" }
        button { onclick: move |_| count += 1, "Increment" }
    }
}
