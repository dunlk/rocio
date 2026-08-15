use crate::features::activities::pages::page::ActivitiesPage;
use crate::features::{home::page::HomePage, students::page::StudentsPage};
use crate::shared::components::navbar::Navbar;
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_router::{
    components::{Route, Routes},
    path,
};

#[component]
pub fn AppLayout() -> impl IntoView {
    let location = use_location();

    let pink_position = move || match location.pathname.get().as_str() {
        "/" => "-translate-x-[800px] -translate-y-[1800px]",
        "/students" => "-translate-x-[400px] -translate-y-[1800px]",
        "/activities" => "translate-x-[0px] -translate-y-[1700px]",
        _ => "-translate-x-[800px] -translate-y-[1700px]",
    };

    let cyan_position = move || match location.pathname.get().as_str() {
        "/" => "translate-x-[0px]",
        "/students" => "-translate-x-[200px] ",
        "/activities" => "-translate-x-[400px]",
        _ => "translate-x-[0px]",
    };
    view! {
        <div class="text-center overflow-hidden text-white bg-slate-900 flex items-center justify-center flex-col container h-screen pt-[20px]">

            <Navbar/>
            <main>
                <Routes fallback=|| view! { <p>"Vista no encontrada"</p> }>
                    <Route path=path!("/") view=|| HomePage()/>
                    <Route path=path!("/students") view=|| StudentsPage()/>
                    <Route path=path!("/activities") view=|| ActivitiesPage()/>
                </Routes>
            </main>
        </div>
        <div
            class=move || format!(
                "blur-[300px] z-0 absolute aspect-square w-[1000px] rounded-full bg-pink-500 \
                transition-transform duration-700 ease-in-out {}",
                pink_position()
            )
        ></div>

        <div
            class=move || format!(
                        "blur-[300px] z-0 transition-transform absolute \
                        aspect-square w-[1000px] rounded-full bg-cyan-500 duration-700 {}",
                        cyan_position()
            )
        ></div>
    }
}
