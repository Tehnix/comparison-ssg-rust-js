use leptos::*;

#[component]
pub fn SubPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"SubPage"</h1>
        <p>"This is a subpage with interactivity: " {count}</p>
        <button on:click=on_click>"Increment"</button>
    }
}
