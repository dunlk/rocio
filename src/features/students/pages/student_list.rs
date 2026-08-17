use leptos::{logging, prelude::*};

use wasm_bindgen_futures::spawn_local;

use crate::features::students::models::Student;

use crate::features::students::service::{delete_student, update_student, UpdateStudent};
#[component]
pub fn StudentList(
    refresh_students: Callback<()>,
    loading: ReadSignal<bool>,
    students_list: ReadSignal<Vec<Student>>,
) -> impl IntoView {
    let (id_delete, set_id_delete) = signal::<Option<i64>>(None);
    let (is_editing, set_is_editing) = signal(false);
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());
    let (editing_id, set_editing_id) = signal::<Option<i64>>(None);

    let handle_delete = move |_: leptos::ev::MouseEvent| {
        spawn_local(async move {
            if let Some(id) = id_delete.get_untracked() {
                match delete_student(id).await {
                    Ok(_) => {
                        set_id_delete.set(None);
                        refresh_students.run(())
                    }
                    Err(error) => {
                        logging::error!("{error}")
                    }
                }
            }
        });
    };

    let clear_names = move || {
        set_last_name.set(String::new());
        set_first_name.set(String::new())
    };

    let handle_edit = move |_: leptos::ev::MouseEvent| {
        let first_name = first_name.get();
        let last_name = last_name.get();

        if first_name.is_empty() || last_name.is_empty() {
            return;
        }

        if is_editing.get() {
            let Some(id) = editing_id.get() else {
                return;
            };

            let data = UpdateStudent {
                first_name,
                last_name,
            };

            spawn_local(async move {
                match update_student(id, data).await {
                    Ok(_) => {}

                    Err(error) => {
                        web_sys::console::error_1(&error.into());
                    }
                }
                refresh_students.run(());
                clear_names();

                set_id_delete.set(None);
            });
        }
    };

    let handle_cancel = move |_: leptos::ev::MouseEvent| {
        set_id_delete.set(None);
        set_is_editing.set(false)
    };

    Effect::new(move |_| {
        refresh_students.run(());
    });

    // let handle_delete_student = move |_| {
    //     spawn_local(asynv move [
    //         de
    // ]);
    // }
    view! {
        <div class="w-full z-10 h-screen grid content-center">
            // <h2 class="animate-in fade-in text-3xl font-bold mb-2">"Alumnos"</h2>
            {move || {
                if loading.get() {
                    view! { <p>"Cargando..."</p> }.into_any()
                } else if students_list.get().is_empty() {

                    view! { <p class="text-xl">"No tienes estudiantes registrados"</p> }
                        .into_any()
                } else {

                    view! {
                        <div class="flex flex-col gap-3 w-full overflow-y-auto py-32 px-4">

                            <For
                                each=move || students_list.get()
                                key=|student| student.id
                                children=move |student| {
                                    let student_id = student.id;
                                    let edit_first_name = student.first_name.clone();
                                    let edit_last_name = student.last_name.clone();

                                    view! {
                                        <div class="
                                        p-4 px-5 bg-cyan-500/10 backdrop-blur-md text-white rounded-3xl
                                        border border-cyan-400/20 shadow-lg shadow-black/10
                                        ">
                                            // Parte superior
                                            <div class="flex items-center justify-between">

                                                // Alumno
                                                <div>
                                                    <p class="text-lg font-bold">
                                                        {student.first_name} " " {student.last_name}
                                                    </p>

                                                    <p class="text-xs text-white/50">"Alumno"</p>
                                                </div>

                                                // Acciones
                                                <div class="flex gap-2">

                                                    // Editar
                                                    <button
                                                        class="
                                                        w-10 h-10 flex items-center justify-center rounded-full bg-cyan-400/15
                                                        text-cyan-300 border border-cyan-400/20 active:scale-90 transition-all
                                                        "
                                                        on:click=move |_| {
                                                            set_first_name.set(edit_first_name.clone());
                                                            set_last_name.set(edit_last_name.clone());
                                                            set_editing_id.set(Some(student_id));
                                                            set_is_editing.set(true);
                                                            set_id_delete.set(Some(student_id));
                                                        }
                                                    >
                                                        "✎"
                                                    </button>

                                                    // Eliminar
                                                    <button
                                                        class="
                                                        w-10 h-10
                                                        flex items-center justify-center
                                                        rounded-full
                                                        bg-red-500/15
                                                        text-red-400
                                                        border border-red-400/20
                                                        active:scale-90
                                                        transition-all
                                                        "
                                                        on:click=move |_| { set_id_delete.set(Some(student_id)) }
                                                    >
                                                        "×"
                                                    </button>

                                                </div>
                                            </div>

                                            // Separador
                                            <div class="my-3 h-px bg-white/10"></div>

                                            // Información
                                            <div class="flex gap-5 text-sm">
                                                <div>
                                                    <span class="text-white/50">"Pagados "</span>
                                                    <span class="font-semibold text-cyan-300">"3"</span>
                                                </div>

                                                <div>
                                                    <span class="text-white/50">"Debe "</span>
                                                    <span class="font-semibold text-pink-400">"S/ 0.00"</span>
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }
                            />
                        </div>
                    }
                        .into_any()
                }
            }}
            <div
                class="bg-black/30 backdrop-blur-md transition-all duration-500 absolute w-full h-screen grid content-center gap-2"
                class=(["block"], move || !id_delete.get().is_none())
                class=(["hidden"], move || id_delete.get().is_none())
                on:click=move |_| { set_id_delete.set(None) }
            >

                {move || {
                    if is_editing.get() {
                        view! {
                            <div
                                class="animate-in fade-in mt-2 mx-4 rounded-3xl flex flex-col items-center justify-center gap-0 "
                                on:click=move |ev| ev.stop_propagation()
                            >
                                <h3 class="text-2xl font-bold">"Editar alumno"</h3>
                                <div class="p-3 rounded-3xl text-md">
                                    <label class="font-bold">"Nombres:  "</label>
                                    <input
                                        class="outline-none border-1 border-cyan-600 focus:border-pink-600/80 ml-1 focus:border-3  transition-all focus:bg-cyan-400/30 bg-cyan-400/10 rounded-3xl p-3"
                                        type="text"
                                        required
                                        prop:value=move || first_name.get()
                                        on:input=move |ev| {
                                            set_first_name.set(event_target_value(&ev))
                                        }
                                    />
                                </div>
                                <div class="p-3 rounded-3xl text-md">
                                    <label class="font-bold">"Apellidos:  "</label>
                                    <input
                                        class="outline-none border-1 border-cyan-600 ml-1 focus:border-3  transition-all
                                        focus:bg-cyan-400/20 focus:border-pink-600/80 bg-cyan-400/10 rounded-3xl p-3"
                                        type="text"
                                        prop:value=move || last_name.get()
                                        required
                                        on:input=move |ev| {
                                            set_last_name.set(event_target_value(&ev));
                                        }
                                    />
                                </div>
                                <div class="flex gap-2 font-bold">
                                    <button
                                        class="bg-green-400/40 rounded-3xl px-5 mx-auto py-3 whitespace-nowrap"
                                        on:click=handle_edit
                                    >
                                        "Guardar cambios"
                                    </button>
                                    <button
                                        class="bg-red-500/40 rounded-3xl px-5 mx-auto py-3 whitespace-nowrap"
                                        on:click=handle_cancel
                                    >
                                        "Cancelar"
                                    </button>
                                </div>
                            </div>
                        }
                            .into_any()
                    } else {
                        view! {
                            <h2>"Esta seguro de eliminar este alumno?"</h2>
                            <div class="font-bold flex gap-2 justify-center">
                                <button
                                    class="rounded-4xl bg-green-400/50 p-3 px-6 backdrop-blur-md"
                                    on:click=handle_delete
                                >
                                    // "Eliminar"
                                    {if is_editing.get() { "Guardar cambios" } else { "Eliminar" }}
                                </button>
                                <button
                                    class="rounded-4xl bg-red-500/50 p-3 px-6 backdrop-blud-md"
                                    on:click=move |_| set_id_delete.set(None)
                                >
                                    "Cancelar"
                                </button>
                            </div>
                        }
                            .into_any()
                    }
                }}
            </div>
        </div>
    }
}
