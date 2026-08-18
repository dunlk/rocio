use crate::features::activities::pages::page::ActivitiesPage;
use crate::features::{home::page::HomePage, students::page::StudentsPage};
use crate::shared::components::navbar::Navbar;
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use leptos_router::{
    components::{Route, Routes},
    path,
};

// struct modal {
//     active_register_student: bool,
// }

#[component]
pub fn AppLayout() -> impl IntoView {
    let (active_register_student, set_active_register_student) = signal(false);
    // let (active_register_activity, set_active_register_activity) = signal(false);
    let location = use_location();

    let pink_position = move || match location.pathname.get().as_str() {
        "/" => "-translate-x-[800px] -translate-y-[1800px]",
        "/students" => "-translate-x-[40%] -translate-y-[1800px]",
        "/activities" => "translate-x-[0px] -translate-y-[1700px]",
        _ => "-translate-x-[800px] -translate-y-[1700px]",
    };

    let cyan_position = move || match location.pathname.get().as_str() {
        "/" => "translate-x-[0px]",
        "/students" => "-translate-x-[200px] ",
        "/activities" => "-translate-x-[400px]",
        _ => "translate-x-[0px]",
    };

    Effect::new(move |_| {
        if location.pathname.get() != "/students" {
            set_active_register_student.set(false)
        }

        // if location.pathname.get() != "/activities" {
        //     set_active.set(false)
        // }
    });
    view! {
        <div class="text-center overflow-hidden text-white bg-slate-900 flex items-center justify-center flex-col h-screen">

            <Navbar
                set_active_register_student=set_active_register_student
                active_register_student=active_register_student
                // set_active_register_activity=set_active_register_activity
                // active_register_activity=active_register_activity
            />
            <main class="w-full h-screen flex flex-col justify-center items-center">
                <Routes fallback=|| view! { <p>"Vista no encontrada"</p> }>
                    <Route path=path!("/") view=|| HomePage() />
                    <Route
                        path=path!("/students")
                        view=move || {
                            view! {
                                <StudentsPage
                                    active_register_student=active_register_student
                                    set_active_register_student=set_active_register_student
                                />
                            }
                        }
                    />
                    <Route
                        path=path!("/activities")
                        view=move || {
                            view! {
                                <ActivitiesPage
                                    // active_register_activity=active_register_activity
                                    // set_active_register_activity=set_active_register_activity
                                />
                            }
                        }
                    />
                </Routes>
            </main>
        </div>
        <div class=move || {
            format!(
                "blur-[300px] z-0 absolute aspect-square w-[1000px] rounded-full bg-pink-500 \
                transition-transform duration-700 ease-in-out {}",
                pink_position(),
            )
        }></div>

        <div class=move || {
            format!(
                "blur-[300px] z-0 transition-transform absolute \
                        aspect-square w-[1000px] rounded-full bg-cyan-500 duration-700 {}",
                cyan_position(),
            )
        }></div>
    }
}
