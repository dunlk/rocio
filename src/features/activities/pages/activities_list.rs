use leptos::prelude::*;

use super::activities_form::ActivitiesForm;
use super::card_activity::CardActivity;
use crate::features::activities::models::Activity;
use crate::features::activities::models::FormMode;
use leptos_router::{hooks::use_location, location};

#[component]
pub fn ActivitiesList(
    activities: ReadSignal<Vec<Activity>>,
    refresh_activities: Callback<()>,
) -> impl IntoView {
    let (id, set_id) = signal::<Option<i64>>(None);
    let (active_edit, set_active_edit) = signal(false);
    let (activity_edit, set_activity_edit) = signal::<Option<Activity>>(None);

    let location = use_location();

    Effect::new(move |_| {
        refresh_activities.run(());
    });

    view! {
        <ActivitiesForm
            mode=FormMode::Edit
            active_register_activity=active_edit
            set_active_register_activity=set_active_edit
            activity=activity_edit
            id=id
        />
        <div
            class="h-screen duration-500 py-32 overflow-y-auto flex flex-col gap-2 z-10 px-4 w-full"
            class=(
                ["animate-in", "fade-in"], move || location.pathname.get() == "/activities"
            )
            class=(["animate-out", "fade-out"], move || location.pathname.get() != "/activities")
        >
            // <CardActivity set_active_register_activity=set_active_register_activity set_mode=set_mode/>
            <For
                each=move || activities.get()
                key=|activity| activity.id
                children=move |activity| {
                    view! {
                        <CardActivity
                            set_active_register_activity=set_active_edit
                            // set_mode=set_mode
                            activity=activity
                            set_activity_edit=set_activity_edit
                            set_id=set_id
                        />
                    }
                }
            />
        </div>
    }
}
