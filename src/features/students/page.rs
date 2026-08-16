use super::pages::student_form::{StudentForm, StudentFormModal};
use super::pages::student_list::StudentList;
use leptos::prelude::*;

#[component]
pub fn StudentsPage(
    active_register_student: ReadSignal<bool>,
    set_active_register_student: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <StudentFormModal
            active_register_student=active_register_student
            set_active_register_student=set_active_register_student
        />
        // <StudentForm/>
        <StudentList />
    }
}
