use crate::shared::layouts::app_layout::AppLayout;
use leptos::prelude::*;
use leptos_router::components::Router;

#[component]
pub fn App() -> impl IntoView {
    view! {
    <div class="relative overflow-hidden bg-slate-900">
        <Router>
            <AppLayout/>
        </Router>
    </div>
        }
}
