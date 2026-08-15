use super::pages::student_form::StudentForm;
use super::pages::student_list::StudentList;
use leptos::prelude::*;

#[component]
pub fn StudentsPage() -> impl IntoView {
    view! {
        // <h1 class="animate-in fade-in text-2xl">"Bienvenidos a alumnos"</h1>

        <StudentForm/>
        // <StudentList/>
    }
}
