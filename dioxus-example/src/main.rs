#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::info;

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

// Generate all routes and output them to the docs path
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    let wrapper = DefaultRenderer {
        before_body: r#"<!DOCTYPE html>
<html>
<head>
  <title>dioxus-example</title>
  <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta charset="UTF-8" />
  <link rel="stylesheet" href="/./assets/tailwind.css">

</head>
<body>"#
            .to_string(),
        after_body: r#"  <script type="module">
    import init from "/./assets/assets/dioxus/dioxus-example.js";
    init("/./assets/assets/dioxus/dioxus-example_bg.wasm").then(wasm => {
      if (wasm.__wbindgen_start == undefined) {
        wasm.main();
      }
    });
  </script>

</body>
</html"#
            .to_string(),
    };
    let mut renderer = IncrementalRenderer::builder()
        .static_dir("static")
        .map_path(|route| {
            let mut path = std::env::current_dir().unwrap();
            path.push("static");
            for segment in route.split('/') {
                path.push(segment);
            }
            println!("built: static{}", route);
            path
        })
        .build();
    renderer.renderer_mut().pre_render = true;
    pre_cache_static_routes::<Route, _>(&mut renderer, &wrapper)
        .await
        .unwrap();
}

// Hydrate the page
#[cfg(not(feature = "server"))]
fn main() {
    #[cfg(all(feature = "web", not(feature = "server")))]
    dioxus::web::launch::launch(App, vec![], dioxus::web::Config::default().hydrate(true));
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        Link { to: Route::SubPage {}, "Sub page" } // This line here
        Link { to: Route::AnotherPage {}, "Another page" } // This line here
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
