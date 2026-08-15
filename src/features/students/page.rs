use super::pages::student_form::StudentForm;
use super::pages::student_list::StudentList;
use leptos::prelude::*;

#[component]
pub fn StudentsPage() -> impl IntoView {
    view! {
        <div class="absolute p-2 px-10 font-semibold shadow-lg shadow-black/20 top-[30px] border-1 border-cyan-600 -translate-x-1/2 left-1/2 bg-cyan-400/30 rounded-4xl z-10">
            <button class="p-2 rounded-4xl">"Crear Alumno"</button>
        </div>

        <StudentForm/>
        // <StudentList/>
    }
}
