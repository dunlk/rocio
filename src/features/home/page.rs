use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="animate-in fade-in z-20 relative">
            <h1 class="font-bold text-4xl">
                "Bienvenid@ a " <span class="font-extrabold text-cyan-500 italic">"Rocio"</span>
            </h1>
            <h2 class="text-xl">"Te ayudamos a que todo este en orden"</h2>
            <div class="flex gap-2 mt-[10px] flex justify-center items-center">
                <div class="bg-cyan-600/70 rounded-3xl p-4 font-bold">
                    <A href="/students">"Ver alumnos"</A>
                </div>
                <div class="bg-pink-600/70 rounded-3xl p-4 font-bold">
                    <A href="/activities">"Ver actividades"</A>
                </div>
            </div>
        </div>
    }
}
