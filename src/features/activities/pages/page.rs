use crate::features::activities::models::{Activity, FormMode};
use crate::features::activities::services::get_activities;

use super::activities_form::ActivitiesForm;
use super::activities_list::ActivitiesList;
use leptos::reactive::spawn_local;
use leptos::{logging, prelude::*};

#[component]
pub fn ActivitiesPage(
    active_register_activity: ReadSignal<bool>,
    set_active_register_activity: WriteSignal<bool>,
) -> impl IntoView {
    let (activities, set_activities) = signal(Vec::<Activity>::new());
    let (loading, set_loading) = signal(true);
    let (mode, set_mode) = signal(FormMode::Create);

    let refresh_activities = Callback::new(move |_| {
        spawn_local(async move {
            match get_activities().await {
                Ok(activities) => set_activities.set(activities),

                Err(error) => {
                    logging::error!("Error al traer activities: {error}")
                }
            }

            set_loading.set(false)
        });
    });

    let update_activities = Callback::new(move |new_activity: Activity| {
        set_activities.update(|activities| {
            if let Some(activity) = activities
                .iter_mut()
                .find(|activity| activity.id == new_activity.id)
            {
                *activity = new_activity;
            }
        });
    });

    view! {
        <ActivitiesForm
            mode=mode
            active_register_activity=active_register_activity
            set_active_register_activity=set_active_register_activity
            refresh_activities=refresh_activities
            update_activities=update_activities
        />
        <ActivitiesList
            activities=activities
            refresh_activities=refresh_activities
            loading=loading
            set_active_register_activity=set_active_register_activity
            set_mode=set_mode
        />
    }
}
