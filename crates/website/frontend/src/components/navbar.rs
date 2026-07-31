use leptos::prelude::*;

#[component]
pub fn Navbar(children: Children) -> impl IntoView {
    view! {
        <nav class="
        w-full
        flex
        items-center
        px-4
        py-3
        border-2
        rounded-lg
        border-lightblue
        ">{children()}</nav>
    }
}
