#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list_with_ssg, build_static_routes, LeptosRoutes}; // This line here
    use leptos_example::app::*;
    use leptos_example::fileserv::file_and_error_handler;

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    let (routes, static_data_map) = generate_route_list_with_ssg(App); // This line here
    build_static_routes(&leptos_options, App, &routes, static_data_map).await; // This line here
    // Move each file from <route>.html --> <route>/index.html
    routes.iter().for_each(|route| {
        if route.path() == "/" { return; }
        print!("Moving route: {:?}\n", &route.path());
        std::fs::create_dir_all(format!("target/site{}", route.path())).unwrap();
        std::fs::copy(format!("target/site{}.html", route.path()), format!("target/site{}/index.html", route.path())).unwrap();
    });

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

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
