use super::activities_form::ActivitiesForm;
use leptos::prelude::*;

#[component]
pub fn ActivitiesPage() -> impl IntoView {
    view! {
        <ActivitiesForm title="Registro de actividad".to_string() />
    }
}
