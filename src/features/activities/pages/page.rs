use super::activities_form::ActivitiesForm;
use leptos::prelude::*;

#[component]
pub fn ActivitiesPage() -> impl IntoView {
    view! {
        <ActivitiesForm type_form="create".to_string() />
    }
}
