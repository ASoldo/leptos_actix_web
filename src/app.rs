use crate::pages::{home_page::HomePage, not_found::NotFound};
use crate::store::counter::provide_counter_store;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    provide_counter_store();

    view! {
    <Stylesheet id="leptos" href="/pkg/main.css" />
    <Title text="Welcome to Leptosssaa" />
    <Router>
        <main>
            <Routes fallback=move || view! { <NotFound /> }>
            <Route path=StaticSegment("") view=HomePage />
            </Routes>
        </main>
    </Router>
    }
}
