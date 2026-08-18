use leptos::prelude::*;
use leptos_router::{components::A, hooks::use_location};

use crate::features::activities::models::{Activity, FormMode};
use crate::features::activities::pages::activities_form::ActivitiesForm;

#[component]
pub fn Navbar(
    set_active_register_student: WriteSignal<bool>,
    active_register_student: ReadSignal<bool>,
    add_activity: Callback<Activity>,
) -> impl IntoView {
    let (active_register, set_active_register) = signal(false);
    let location = use_location();
    let (activity, set_activity) = signal::<Option<Activity>>(None);
    let (id, set_id) = signal::<Option<i64>>(None);

    Effect::new(move |_| {
        if location.pathname.get() != "activities" {
            set_active_register.set(false);
        }
    });
    view! {
        <ActivitiesForm
             active_register_activity=active_register
             set_active_register_activity=set_active_register
             mode=FormMode::Create
             id=id
             activity=activity
            add_activity=add_activity
         />
        <div
            class="absolute z-50 bottom-[30px] shadow-lg shadow-black/20 -translate-x-1/2  left-1/2 rounded-4xl flex
            bg-cyan-400/10 w-[320px] text-white transition-all duration-300  text-md font-semibold border-1 
            border-cyan-600 p-2 backdrop-blur-md"
            class=("w-[370px]", move || location.pathname.get() == "/students")
            class=("w-[350px]", move || location.pathname.get() == "/activities")
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
                    class=(
                        ["bg-pink-400/60", "shadow-black/30", "shadow-lg"],
                        move || location.pathname.get() == "/activities",
                    )
                    class=("bg-pink-500/90", move || active_register.get())
                >
                    {move || {
                        if location.pathname.get() == "/activities" {
                            view! {
                                <button
                                    class="min-w-[120px] flex justify-center"
                                    on:click=move |_| set_active_register.set(true)
                                >
                                    "Crear actividad"
                                </button>
                            }
                                .into_any()
                        } else {

                            view! { <A href="/activities">"Actividades"</A> }
                                .into_any()
                        }
                    }}
                </li>
            </ul>
        </div>
    }
}
