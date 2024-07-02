use crate::anotherpage::AnotherPage; // This line here
use crate::error_template::{AppError, ErrorTemplate};
use crate::subpage::SubPage; // This line here
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-example.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <main>
                <Routes>
                    // This line here
                    <StaticRoute
                        mode=StaticMode::Upfront
                        path=""
                        view=HomePage
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    // This line here
                    <StaticRoute
                        mode=StaticMode::Upfront
                        path="/sub-page"
                        view=SubPage
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                    // This line here
                    <StaticRoute
                        mode=StaticMode::Upfront
                        path="/another-page"
                        view=AnotherPage
                        static_params=move || Box::pin(async move { StaticParamsMap::default() })
                    />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        // This line here
        <A href="/sub-page">"Sub page"</A>
        // This line here
        <A href="/another-page">"Another page"</A>
    }
}
