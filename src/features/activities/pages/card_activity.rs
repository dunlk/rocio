use leptos::logging;
use leptos::prelude::*;

use crate::features::activities::models::{Activity, FormMode};

#[component]
pub fn CardActivity(
    set_active_register_activity: WriteSignal<bool>,
    // set_mode: WriteSignal<FormMode>,
    activity: Activity,
    set_id: WriteSignal<Option<i64>>,
    set_activity_edit: WriteSignal<Option<Activity>>,
) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (activity_type, set_activity_type) = signal(String::new());
    let (amount, set_amount) = signal(0.00);
    let (activities_date, set_activities_date) = signal(String::new());
    let (due_date, set_due_date) = signal(String::new());
    let activity_id = activity.id;

    let activity_for_edit = activity.clone();

    let handle_edit = move |_: leptos::ev::MouseEvent| {
        // set_mode.set(FormMode::Edit);
        set_id.set(Some(activity.id));
        set_active_register_activity.set(true);
        set_activity_edit.set(Some(activity_for_edit.clone()));
    };

    let handle_delete = move |_: leptos::ev::MouseEvent| {
        leptos::logging::log!("CLICK FUNCIONA");
        set_id.set(Some(activity.id));
    };

    view! {
            <div class="
            p-4 px-5
            bg-cyan-500/10 backdrop-blur-md
            text-white rounded-3xl
            border border-cyan-400/20
            shadow-lg shadow-black/10
            ">
                // Parte superior
                <div class="flex items-center justify-between">

                    // Actividad
                    <div class="min-w-0">
                        <p class="text-lg font-bold truncate">{activity.name}</p>

                        <p class="text-xs text-white/50">"Actividad"</p>
                    </div>

                    // Acciones
                    <div class="flex gap-2">

                        // Editar
                        <button class="
                        w-10 h-10
                        flex items-center justify-center
                        rounded-full
                        bg-cyan-400/15
                        text-cyan-300
                        border border-cyan-400/20
                        active:scale-90
                        transition-all
                        "
                        on:click=handle_edit
                        >"✎"</button>

                        // Eliminar
                        <button class="
                        w-10 h-10
                        flex items-center justify-center
                        rounded-full
                        bg-red-500/15
                        text-red-400
                        border border-red-400/20
                        active:scale-90
                        transition-all
                        "
                        on:click=handle_delete
                        >"×"</button>

                    </div>
                </div>

                // Descripción
                <p class="mt-3 text-sm text-white/60 line-clamp-2">
                    {activity.description}
                </p>

                // Separador
                <div class="my-3 h-px bg-white/10"></div>

                // Tipo y monto
                <div class="flex items-center justify-between">

                    <div>
                        <p class="text-xs text-white/40">"Tipo"</p>

                        <span class="
                        inline-block mt-1
                        px-3 py-1
                        rounded-full
                        bg-cyan-400/15
                        text-cyan-300
                        text-xs font-semibold
                        border border-cyan-400/20
                        ">{move || match activity.activity_type.as_str() {
                            "monthly" => "Mensual",
                            "occasional" => "Ocasional",
                            _ => "Desconocido"
                        }}</span>
                    </div>

                    <div class="text-right">
                        <p class="text-xs text-white/40">"Monto"</p>

                        <p class="text-xl font-bold text-cyan-300">"S/ "{activity.amount}</p>
                    </div>

                </div>

                // Fechas
                <div class="
                mt-4 p-3
                rounded-2xl
                bg-black/10
                flex justify-between
                text-sm
                ">
                    <div>
                        <p class="text-xs text-white/40">"Fecha"</p>
                        <p class="font-medium">{activity.activities_date}</p>
                    </div>

                    <div class="text-right">
                        <p class="text-xs text-white/40">"Vencimiento"</p>
                        <p class="font-medium text-pink-300">{activity.due_date}</p>
                    </div>
                </div>
            </div>
    }
}
