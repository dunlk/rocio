use super::activities_form::ActivitiesForm;
use super::activities_list::ActivitiesList;
use leptos::prelude::*;

#[component]
pub fn ActivitiesPage(
    active_regiser_activity: ReadSignal<bool>,
    set_active_register_activity: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <ActivitiesForm type_form="create".to_string()  active_register_activity=active_regiser_activity
            set_active_register_activity=set_active_register_activity
        />
        <ActivitiesList/>
        // <ActivitiesList/>
        // <ActivitiesList/>
    }
}
