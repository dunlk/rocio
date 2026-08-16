use leptos::{logging, prelude::*};

use wasm_bindgen_futures::spawn_local;

use crate::features::students;
use crate::features::students::models::Student;
use crate::features::students::service::get_students;

#[component]
pub fn StudentList(
    refresh_students: Callback<()>,
    loading: ReadSignal<bool>,
    students_list: ReadSignal<Vec<Student>>,
) -> impl IntoView {
    Effect::new(move |_| {
        refresh_students.run(());
    });

    view! {
        <div class="w-full p-3 z-10">
            <h2 class="animate-in fade-in text-3xl">"Alumnos"</h2>

            {move || {
                if loading.get() {
                    view! { <p>"Cargando..."</p> }.into_any()
                } else if students_list.get().is_empty() {

                    view! { <p class="text-xl">"No tienes estudiantes registrados"</p> }
                        .into_any()
                } else {

                    view! {
                        <div class="flex flex-col gap-1 pb-15 w-full h-[750px] overflow-y-auto">
                            <For
                                each=move || students_list.get()
                                key=|student| student.id
                                children=move |student| {
                                    view! {
                                        <div class="p-4 px-6 bg-black/30 rounded-3xl">
                                            <div class="flex justify-between text-xl">
                                                <div>{student.first_name}" "{student.last_name}</div>
                                                <div class="font-bold flex gap-2">
                                                    <button>"X"</button>
                                                    <button>"E"</button>
                                                </div>

                                            </div>
                                            <div class="flex gap-2 text-sm">
                                                <p>"Pagados: 3"</p>
                                                <p>"Debe: 0"</p>

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
        // <For
        // each=move || students.get()
        // key=|student| student.id
        // children=move |student| {
        // view! {
        // <p>
        // {student.first_name}
        // " "
        // {student.last_name}
        // </p>
        // }
        // }
        // />
        </div>
    }
}
