use crate::features::activities;
use crate::features::activities::models::Activity;
use crate::features::activities::pages::page::ActivitiesPage;
use crate::features::activities::services::{get_activities, update_activity};
use crate::features::{home::page::HomePage, students::page::StudentsPage};
use crate::shared::components::navbar::Navbar;
use leptos::leptos_dom::logging;
use leptos::prelude::*;
use leptos::reactive::spawn_local;
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
    let (activities, set_activities) = signal(Vec::<Activity>::new());
    // let (activities, set_activities) = signal::<Vec<RwSignal<Activity>>>(Vec::new());
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

    spawn_local(async move {
        match get_activities().await {
            Ok(data) => set_activities.set(data),
            Err(error) => leptos::logging::error!("{error}"),
        }
    });

    let refresh_activities = Callback::new(move |()| {
        spawn_local(async move {
            match get_activities().await {
                Ok(activities) => set_activities.set(activities),

                Err(error) => {
                    leptos::logging::error!("Error al traer activities: {error}")
                }
            }
        });
    });

    let add_activity = Callback::new(move |activity: Activity| {
        set_activities.update(|activities| {
            activities.push(activity);
        })
    });

    // let update_activity = Callback::new(move  |activity:Activity| {
    //     spawn_local(async move {
    //         match update_activity(id, data)
    //     });
    // })
    //

    Effect::new(move |_| {
        if location.pathname.get() != "/students" {
            set_active_register_student.set(false)
        }
    });
    view! {
        <div class="text-center overflow-hidden text-white bg-slate-900 flex items-center justify-center flex-col h-screen">

            <Navbar
                set_active_register_student=set_active_register_student
                active_register_student=active_register_student
                add_activity=add_activity
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
                                    activities_list=activities
                                    refresh_activities=refresh_activities
                                />
                            }
                        }
                    />
                </Routes>
            </main>
        </div>
        <div class=move || {
            format!(
                "blur-[300px] z-0 absolute aspect-square w-[1000px] rounded-full bg-pink-500/80 \
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
