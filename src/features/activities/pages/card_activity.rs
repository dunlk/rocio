use leptos::prelude::*;

use crate::features::activities::models::Activity;

#[component]
pub fn CardActivity(
    set_active_register_activity: WriteSignal<bool>,
    activity: RwSignal<Activity>,
    set_id: WriteSignal<Option<i64>>,
    set_activity_edit: WriteSignal<Option<Activity>>,
    set_is_active_modal: WriteSignal<bool>,
) -> impl IntoView {
    let handle_edit = move |_: leptos::ev::MouseEvent| {
        let activity_value = activity.get_untracked();

        set_id.set(Some(activity_value.id));
        set_activity_edit.set(Some(activity_value));
        set_active_register_activity.set(true);
    };

    let handle_delete = move |_: leptos::ev::MouseEvent| {
        let activity_id = activity.get_untracked().id;

        set_id.set(Some(activity_id));
        set_is_active_modal.set(true);
    };

    view! {
        <div class="
            p-4 px-5 animate-in fade-in duration-700
            bg-cyan-400/10
            backdrop-blur-md
            text-white rounded-3xl
            border border-cyan-500/30
            shadow-lg shadow-black/10
        ">
            // Parte superior
            <div class="flex items-center justify-between">

                // Actividad
                <div class="min-w-0">
                    <p class="text-lg font-bold truncate">
                        {move || activity.with(|a| a.name.clone())}
                    </p>

                    <p class="text-xs text-white/50">
                        "Actividad"
                    </p>
                </div>

                // Acciones
                <div class="flex gap-2">

                    // Editar
                    <button
                        class="
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
                    >
                        "✎"
                    </button>

                    // Eliminar
                    <button
                        class="
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
                    >
                        "×"
                    </button>

                </div>
            </div>

            // Descripción
            <p class="mt-3 text-sm text-white/60 line-clamp-2">
                <p class="mt-3 text-sm text-white/60 line-clamp-2">
                    {move || {
                        activity.with(|a| a.description.clone())
                    }}
                </p>
            </p>

            // Separador
            <div class="my-3 h-px bg-white/10"></div>

            // Tipo y monto
            <div class="flex items-center justify-between">

                <div>
                    <p class="text-xs text-white/40">
                        "Tipo"
                    </p>

                    <span class="
                        inline-block mt-1
                        px-3 py-1
                        rounded-full
                        bg-cyan-400/15
                        text-cyan-300
                        text-xs font-semibold
                        border border-cyan-400/20
                    ">
                        {move || {
                            activity.with(|a| {
                                match a.activity_type.as_str() {
                                    "monthly" => "Mensual",
                                    "occasional" => "Ocasional",
                                    _ => "Desconocido",
                                }
                            })
                        }}
                    </span>
                </div>

                <div class="text-right">
                    <p class="text-xs text-white/40">
                        "Monto"
                    </p>

                    <p class="text-xl font-bold text-cyan-300">
                        "S/ "
                        {move || activity.with(|a| a.amount)}
                    </p>
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
                    <p class="text-xs text-white/40">
                        "Fecha"
                    </p>

                    <p class="font-medium">
                        {move || {
                            activity.with(|a| {
                                a.activities_date
                                    .clone()
                            })
                        }}
                    </p>
                </div>

                <div class="text-right">
                    <p class="text-xs text-white/40">
                        "Vencimiento"
                    </p>

                    <p class="font-medium text-pink-300">
                        {move || {
                            activity.with(|a| {
                                a.due_date
                                    .clone()
                            })
                        }}
                    </p>
                </div>
            </div>
        </div>
    }
}
