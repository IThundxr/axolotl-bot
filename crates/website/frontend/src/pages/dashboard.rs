use gloo_net::http::Request;
use leptos::prelude::*;
use leptos::web_sys;
use leptos_router::hooks::use_navigate;
use crate::components::editor::CodeMirror;
use crate::components::navbar::Navbar;

#[component]
pub fn Dashboard() -> impl IntoView {
    let navigate = use_navigate();

    let logout = move |_| {
        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            let _ = Request::post("/api/auth/discord/logout").credentials(web_sys::RequestCredentials::Include).send()
                .await;

            navigate("/", Default::default());
        });
    };

    view! {
        <main class="min-h-screen crt px-3 pt-3">
            <Navbar>
                <div class="flex items-center justify-between w-full">
                    <div class="text-xl text-pink">"Axolotl"</div>

                    <button
                        class="
                        rounded
                        px-3 py-1
                        text-base text-lightblue
                        transition-colors duration-150
                        hover:bg-lightblue hover:text-neutral-900
                        focus:outline-none focus:ring-2 focus:ring-lightblue
                        "
                        on:click=logout
                    >
                        "> Log Out"
                    </button>
                </div>
            </Navbar>
        </main>
    }
}
