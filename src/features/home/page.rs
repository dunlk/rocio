use crate::shared::components::button::Button;
use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <h1 class="font-bold text-4xl">"Bienvenid@ a "
            <span class="font-bold text-cyan-500 italic">"Rocio"</span>
        </h1>
        <h2 class="text-xl">"Te ayudamos a que todo este en orden"</h2>
        <div class="flex gap-2 mt-[10px] flex justify-center items-center">
            <Button text="Ver alumnos".to_string() color="bg-cyan-600/80 rounded-3xl p-4 font-bold".to_string()/>
            <Button text="Crear actividad".to_string() color="bg-pink-600/80 rounded-3xl p-4 font-bold".to_string()/>
        </div>
    }
}
