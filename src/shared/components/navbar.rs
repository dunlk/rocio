use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="absolute z-10 bottom-[30px] shadow-lg shadow-black/20 -translate-x-1/2  left-1/2 rounded-4xl
                    bg-cyan-400/35 text-white text-md font-semibold border-1 border-cyan-600 p-2 backdrop-blur-md">
            <ul class="flex gap-1 items-center justify-between">
                <li class="bg-pink-500/70 p-4 rounded-4xl">"Inicio"</li>
                <li class="p-4 rounded-4xl">"Alumnos"</li>
                <li class="p-4 rounded-4xl">"Actividades"</li>
            </ul>
        </div>
    }
}
