use leptos::prelude::*;

#[component]
pub fn Page404() -> impl IntoView {
    view! {
        <main class="min-h-screen crt flex items-center justify-center">
            <div class="text-5xl text-lightblue">"404 Not Found!"</div>
        </main>
    }
}
