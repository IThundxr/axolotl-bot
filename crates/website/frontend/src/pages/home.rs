use leptos::prelude::*;
use leptos_router::components::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="min-h-screen relative crt jetbrains-mono font-normal flex flex-col items-center pt-8">
            <div class="pt-8 pl-4 inline-block whitespace-nowrap select-none text-center">
                <div class="text-7xl text-pink">"Axolotl"</div>
                <div class="text-2xl text-lightblue pt-4">
                    "Axolotl is a private discord bot" <br /> "with highly configurable plugins."
                </div>

                <A
                    href="/dashboard"
                    attr:class="
                    mt-10 inline-block
                    border-2 rounded border-lightblue
                    px-6 py-2
                    text-2xl text-lightblue
                    transition-colors duration-150
                    hover:bg-lightblue hover:text-neutral-900
                    focus:outline-none focus:ring-2 focus:ring-lightblue
                    "
                >
                    "> Dashboard"
                </A>
            </div>
        </main>
    }
}
