use crate::features::activities;
use crate::features::activities::models::{Activity, FormMode};
use crate::features::activities::services::get_activities;

use super::activities_form::ActivitiesForm;
use super::activities_list::ActivitiesList;
use leptos::reactive::spawn_local;
use leptos::{logging, prelude::*};

#[component]
pub fn ActivitiesPage(
    activities_list: ReadSignal<Vec<Activity>>,
    refresh_activities: Callback<()>,
) -> impl IntoView {
    view! {
        <ActivitiesList
            activities=activities_list
            refresh_activities=refresh_activities
        />
    }
}
