use crate::shared::layouts::app_layout::AppLayout;
use leptos::prelude::*;
use leptos_router::components::Router;

#[component]
pub fn App() -> impl IntoView {
    view! {
    <div class="relative overflow-hidden">

        <Router>
            <AppLayout/>
            <div class="blur-[300px] z-0 left-[-800px] top-[-800px] absolute aspect-square w-[1000px] rounded-full bg-pink-500"></div>
            <div class="blur-[300px] z-0 right-[-800px] bottom-[-800px] absolute aspect-square w-[1000px] rounded-full bg-cyan-500"></div>
        </Router>

    </div>
        }
}
