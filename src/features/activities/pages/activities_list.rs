use leptos::prelude::*;

use super::card_activity::CardActivity;
use crate::features::activities::models::Activity;
use leptos_router::{hooks::use_location, location};

#[component]
pub fn ActivitiesList(
    activities: ReadSignal<Vec<Activity>>,
    refresh_activities: Callback<()>,
    loading: ReadSignal<bool>,
    set_active_register_activity: WriteSignal<bool>,
) -> impl IntoView {
    let location = use_location();
    Effect::new(move |_| {
        refresh_activities.run(());
    });
    view! {
        <div
            class="h-screen duration-500 py-32 overflow-y-auto flex flex-col gap-2 z-10 px-4"
            class=(
                ["animate-in", "fade-in"], move || location.pathname.get() == "/activities"
            )
            class=(["animate-out", "fade-out"], move || location.pathname.get() != "/activities")
        >
            <CardActivity set_active_register_activity=set_active_register_activity/>
            <CardActivity set_active_register_activity=set_active_register_activity/>
            <CardActivity set_active_register_activity=set_active_register_activity/>
            <CardActivity set_active_register_activity=set_active_register_activity/>
        </div>
    }
}
