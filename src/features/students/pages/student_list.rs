use leptos::prelude::*;

use wasm_bindgen_futures::spawn_local;

use crate::features::students::service::get_students;

#[component]
pub fn StudentList() -> impl IntoView {
    let (students, set_students) = signal(Vec::new());
    let (loading, set_loading) = signal(true);

    spawn_local(async move {
        match get_students().await {
            Ok(data) => {
                set_students.set(data);
            }

            Err(error) => {
                web_sys::console::error_1(&format!("Error cargando estudiantes: {error}").into());
            }
        }

        set_loading.set(false);
    });

    view! {
        <div>
            <h2 class="animate-in fade-in text-3xl">"Estudiantes"</h2>

            {move || {
                if loading.get() {
                    view! { <p>"Cargando..."</p> }.into_any()
                } else if students.get().is_empty() {

                    view! { <p class="text-xl">"No tienes estudiantes registrados"</p> }
                        .into_any()
                } else {

                    view! {
                        <For
                            each=move || students.get()
                            key=|student| student.id
                            children=move |student| {
                                view! { <p>{student.first_name} " " {student.last_name}</p> }
                            }
                        />
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
