use leptos::prelude::*;

use super::card_activity::CardActivity;
use crate::features::activities::models::Activity;

#[component]
pub fn ActivitiesList(
    activities: ReadSignal<Vec<Activity>>,
    refresh_activities: Callback<()>,
    loading: ReadSignal<bool>,
    set_active_register_activity: WriteSignal<bool>,
) -> impl IntoView {
    Effect::new(move |_| {
        refresh_activities.run(());
    });
    view! {
        <div class="h-screen py-32 overflow-y-auto flex flex-col gap-2 z-10 px-4">
            <CardActivity set_active_register_activity=set_active_register_activity/>
            <CardActivity set_active_register_activity=set_active_register_activity/>
            <CardActivity set_active_register_activity=set_active_register_activity/>
            <CardActivity set_active_register_activity=set_active_register_activity/>
        </div>
    }
}
