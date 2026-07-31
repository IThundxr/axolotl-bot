use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

use crate::pages::dashboard::Dashboard;
use crate::pages::error_pages::Page404;
use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| Page404>
                <Route path=path!("/") view=Home />
                <Route path=path!("/dashboard") view=Dashboard />
            </Routes>
        </Router>
    }
}
