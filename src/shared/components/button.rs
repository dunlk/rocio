use leptos::prelude::*;

#[component]
pub fn Button(text: String, color: String) -> impl IntoView {
    view! {
        <button class=color>
            {text}
        </button>
    }
}
