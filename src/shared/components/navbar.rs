use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

#[component]
pub fn Navbar() -> impl IntoView {
    let location = use_location();
    view! {
        <div class="absolute z-10 bottom-[30px] shadow-lg shadow-black/20 -translate-x-1/2  left-1/2 rounded-4xl
                    bg-cyan-400/20 text-white text-md font-semibold border-1 border-cyan-600 p-2 backdrop-blur-md">
            <ul class="flex gap-1 items-center justify-between">
                <li

                    class="p-4 rounded-4xl transition-colors duration-700 ease-in-out"
                    class=(
                        "bg-pink-400/60", move || location.pathname.get() == "/"
                    )
                >
                    <A href="/">"Inicio"</A>
                </li>
                <li
                    class="p-4 rounded-4xl transition-colors duration-700 ease-in-out"

                    class=(
                        "bg-pink-400/60", move || location.pathname.get() == "/students"
                    )
                >
                    <A href="/students">"Alumnos"</A>
                </li>

                <li
                    class="p-4 rounded-4xl transition-colors duration-700 ease-in-out"
                    class=(
                        "bg-pink-400/60", move || location.pathname.get() == "/activities"
                    )
                >
                    <A href="/activities">"Actividades"</A>
                </li>
            </ul>
        </div>
    }
}
