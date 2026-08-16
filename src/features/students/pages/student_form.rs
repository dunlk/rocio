use leptos::logging;
use leptos::prelude::*;
use leptos::reactive::spawn_local;

use crate::features::students;
use crate::features::students::models::CreateStudent;
use crate::features::students::service::{create_student, get_students};

#[component]
pub fn StudentForm(
    set_active_register_student: WriteSignal<bool>,
    refresh_students: Callback<()>,
) -> impl IntoView {
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());

    let handle_create_student = move |_| {
        let first_name = first_name.get();
        let last_name = last_name.get();

        spawn_local(async move {
            match create_student(first_name, last_name).await {
                Ok(_) => {
                    set_active_register_student.set(false);
                    refresh_students.run(());
                }
                Err(error) => {
                    logging::error!("{error}");
                }
            }
        });
    };

    let handle_cancel = move |_: leptos::ev::MouseEvent| {
        set_active_register_student.set(false);
    };

    view! {
        <h1 class="animate-in fade-in text-3xl font-bold z-10">"Registrar Alumno"</h1>

        <div
            class="animate-in fade-in mt-2 mx-4 rounded-3xl flex flex-col items-center justify-center gap-0 "
            on:click=move |ev| ev.stop_propagation()
        >
            <div class="p-3 rounded-3xl text-md">
                <label class="font-bold">"Nombres:  "</label>
                <input
                    class="outline-none border-1 border-cyan-600 focus:border-pink-600/80 ml-1 focus:border-3  transition-all focus:bg-cyan-400/30 bg-cyan-400/10 rounded-3xl p-3"
                    type="text"
                    value=move || first_name.get()
                    on:input=move |ev| { set_first_name.set(event_target_value(&ev)) }
                />
            </div>
            <div class="p-3 rounded-3xl text-md">
                <label class="font-bold">"Apellidos:  "</label>
                <input
                    class="outline-none border-1 border-cyan-600 ml-1 focus:border-3  transition-all
                      focus:bg-cyan-400/20 focus:border-pink-600/80 bg-cyan-400/10 rounded-3xl p-3"
                    type="text"
                    prop:value=move || last_name.get()
                    on:input=move |ev| {
                        set_last_name.set(event_target_value(&ev));
                    }
                />
            </div>
            <div class="flex gap-2">
                <button
                    class="bg-green-400/40 rounded-3xl px-5 mx-auto py-2 whitespace-nowrap"
                    on:click= handle_create_student
                >
                    "Crear alumno"
                </button>
                <button
                    class="bg-red-400/40 rounded-3xl px-5 mx-auto py-2 whitespace-nowrap"
                    on:click= handle_cancel
                >
                    "Cancelar"
                </button>
            </div>
        </div>
    }
}

#[component]
pub fn StudentFormModal(
    active_register_student: ReadSignal<bool>,
    set_active_register_student: WriteSignal<bool>,
    refresh_students: Callback<()>,
) -> impl IntoView {
    view! {
        <div
            class="absolute grid content-center z-18 w-full animate-in fade-in h-screen bg-black/30 backdrop-blur-lg"
            class=(["block"], move || active_register_student.get())
            class=(["hidden"], move || !active_register_student.get())
            on:click=move |_| { set_active_register_student.set(false) }
        >
            <StudentForm
                set_active_register_student=set_active_register_student
                refresh_students=refresh_students
            />
        </div>
    }
}
