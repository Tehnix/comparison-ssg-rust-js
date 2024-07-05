#![allow(non_snake_case)]

use dioxus::prelude::*;

mod subpage; // This line here
use subpage::SubPage; // This line here
mod anotherpage; // This line here
use anotherpage::AnotherPage; // This line here

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/sub-page")] // This line here
    SubPage {}, // This line here
    #[route("/another-page")] // This line here
    AnotherPage {}, // This line here
}

// Generate all routes and output them to the static path
fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link { to: Route::SubPage {}, "Sub page" } // This line here
        Link { to: Route::AnotherPage {}, "Another page" } // This line here
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}
