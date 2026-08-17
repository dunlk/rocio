use crate::features::activities::models::Activity;
use crate::features::students;
use crate::features::students::service::get_students;

use super::models::Student;
use super::pages::student_form::{StudentForm, StudentFormModal};
use super::pages::student_list::StudentList;
use leptos::reactive::spawn_local;
use leptos::{logging, prelude::*};

#[component]
pub fn StudentsPage(
    active_register_student: ReadSignal<bool>,
    set_active_register_student: WriteSignal<bool>,
) -> impl IntoView {
    let (students_list, set_students_list) = signal(Vec::<Student>::new());
    let (loading, set_loading) = signal(true);

    let refresh_students = Callback::new(move |_| {
        spawn_local(async move {
            match get_students().await {
                Ok(students) => set_students_list.set(students),

                Err(error) => {
                    logging::error!("{error}")
                }
            }

            set_loading.set(false)
        });
    });

    view! {
        <StudentFormModal
            active_register_student=active_register_student
            set_active_register_student=set_active_register_student
            refresh_students=refresh_students
        />
        // <StudentForm/>
        <StudentList
            refresh_students=refresh_students
            loading=loading
            students_list=students_list
        />
    }
}
