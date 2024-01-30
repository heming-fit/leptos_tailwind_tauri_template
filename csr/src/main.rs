use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! { <h1 class="text-3xl font-bold underline">Hello leptos!</h1> }
}
