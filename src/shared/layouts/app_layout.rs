use crate::shared::components::navbar::Navbar;
use crate::{features::home::page::HomePage, shared::components::button::Button};
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Routes},
    path,
};

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div class="relative text-center overflow-hidden text-white bg-slate-900 flex items-center justify-center flex-col container h-screen pt-[20px]">

            <Navbar/>
            <main>
                <Routes fallback=|| view! { <p>"Vista no encontrada"</p> }>
                    <Route path=path!("/") view=|| HomePage()/>
                </Routes>
            </main>
        </div>
    }
}
