use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

#[component]
pub fn Navbar(
    set_active_register_student: WriteSignal<bool>,
    active_register_student: ReadSignal<bool>,
) -> impl IntoView {
    let location = use_location();
    view! {
        <div
            class="absolute z-50 bottom-[30px] shadow-lg shadow-black/20 -translate-x-1/2  left-1/2 rounded-4xl flex
             bg-cyan-400/20 w-[320px] text-white transition-all duration-300  text-md font-semibold border-1 
             border-cyan-600 p-2 backdrop-blur-md"
            class=("w-[370px]", move || location.pathname.get() == "/students")
        >
            <ul class="flex gap-1 items-center justify-between">
                <li

                    class="p-4 rounded-4xl transition-colors duration-700 ease-in-out"
                    class=(["bg-pink-400/60"], move || location.pathname.get() == "/")
                >
                    <A href="/">"Inicio"</A>
                </li>
                <li
                    class="p-4 rounded-4xl transition-colors duration-700 ease-in-out"

                    class=(
                        ["bg-pink-400/60", "shadow-black/30", "shadow-lg"],
                        move || location.pathname.get() == "/students",
                    )
                    class=(["bg-pink-500/90"], move || active_register_student.get())
                >
                    {move || {
                        if location.pathname.get() == "/students" {
                            view! {
                                <button
                                    on:click=move |_| set_active_register_student.set(true)
                                    class="min-w-[120px] flex justify-center"
                                >
                                    "Crear alumno"
                                </button>
                            }
                                .into_any()
                        } else {
                            view! {
                                <div>
                                    <A href="/students">"Alumnos"</A>
                                </div>
                            }
                                .into_any()
                        }
                    }}
                </li>

                <li
                    class="p-4 rounded-4xl transition-colors duration-700 ease-in-out"
                    class=("bg-pink-400/60", move || location.pathname.get() == "/activities")
                >
                    <A href="/activities">"Actividades"</A>
                </li>
            </ul>
        </div>
    }
}
