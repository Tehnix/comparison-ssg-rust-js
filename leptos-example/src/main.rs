#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{build_static_routes, generate_route_list_with_ssg, LeptosRoutes}; // This line here
    use leptos_example::app::*;
    use leptos_example::fileserv::file_and_error_handler;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;

    // Build our static routes
    let (routes, static_data_map) = generate_route_list_with_ssg(App); // This line here
    build_static_routes(&leptos_options, App, &routes, static_data_map).await; // This line here

    #[cfg(not(feature = "static"))]
    {
        let addr = leptos_options.site_addr;
        // build our application with a route
        let app = Router::new()
            .leptos_routes(&leptos_options, routes.clone(), App)
            .fallback(file_and_error_handler)
            .with_state(leptos_options);

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        logging::log!("listening on http://{}", &addr);
        axum::serve(listener, app.into_make_service())
            .await
            .unwrap();
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
