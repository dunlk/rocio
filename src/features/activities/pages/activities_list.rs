use leptos::prelude::*;

use super::activities_form::ActivitiesForm;
use super::card_activity::CardActivity;
use super::modal_delete::ModalDelete;
use crate::features::activities::models::Activity;
use crate::features::activities::models::FormMode;
use leptos_router::{hooks::use_location, location};

#[component]
pub fn ActivitiesList(
    activities: ReadSignal<Vec<Activity>>,
    refresh_activities: Callback<()>,
    #[prop(optional)] add_activity: Option<Callback<Activity>>,
) -> impl IntoView {
    let (id, set_id) = signal::<Option<i64>>(None);
    let (active_edit, set_active_edit) = signal(false);
    let (activity_edit, set_activity_edit) = signal::<Option<Activity>>(None);
    let (is_activa_modal, set_is_active_modal) = signal(false);

    let location = use_location();

    Effect::new(move |_| {
        refresh_activities.run(());
    });

    view! {
        <ModalDelete
            id=id
            set_id=set_id
            is_active_modal=is_activa_modal
            set_is_active_modal=set_is_active_modal
            refresh_activities=refresh_activities
        />
        <ActivitiesForm
            mode=FormMode::Edit
            active_register_activity=active_edit
            set_active_register_activity=set_active_edit
            activity=activity_edit
            id=id
            refresh_activities=refresh_activities
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
            key=|activity| (
                activity.id,
                activity.name.clone(),
                activity.amount.to_bits(),
                activity.description.clone(),
                activity.activity_type.clone(),
                activity.due_date.clone(),
                activity.activities_date.clone()

            )
            children=move |activity| {
                let activity = RwSignal::new(activity);
                view! {
                    <CardActivity
                        set_active_register_activity=set_active_edit
                        activity=activity
                        set_activity_edit=set_activity_edit
                        set_id=set_id
                        set_is_active_modal=set_is_active_modal
                    />
                }
            }
        />
        </div>
    }
}
