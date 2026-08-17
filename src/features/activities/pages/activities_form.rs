use crate::features::{
    activities::{
        self,
        models::{Activity, CreateActivity, FormMode, UpdateActivity},
        services::create_activity,
    },
    students::service::UpdateStudent,
};
use leptos::{logging, prelude::*, reactive::spawn_local};
use serde::Serialize;

#[component]
pub fn ActivitiesForm(
    mode: ReadSignal<FormMode>,
    active_register_activity: ReadSignal<bool>,
    set_active_register_activity: WriteSignal<bool>,
    refresh_activities: Callback<()>,
    update_activities: Callback<Activity>,
) -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (description, set_description) = signal(String::new());
    let (activity_type, set_acitivity_type) = signal(String::new());
    let (amount, set_amount) = signal(0.00);
    let (due_date, set_dute_date) = signal(String::new());
    let (activities_date, set_activities_date) = signal(String::new());

    let handle_button_submit = move |_: leptos::ev::MouseEvent| {
        // logging::log!("{}", name.get());
        // logging::log!("{}", description.get());
        // logging::log!("{}", type_activity.get());
        // logging::log!("{}", amount.get());
        // logging::log!("{}", due_date.get());
        // logging::log!("{}", activities_date.get());
        // logging::log!("{mode:?}");

        match mode.get() {
            FormMode::Create => {
                let name = name.get();
                let description = Some(description.get());
                let activity_type = Some(activity_type.get());
                let amount = amount.get();
                let due_date = due_date.get();
                let activities_date = activities_date.get();

                let data = CreateActivity {
                    name,
                    description,
                    activity_type,
                    amount,
                    activities_date,
                    due_date,
                };
                spawn_local(async move {
                    match create_activity(data).await {
                        Ok(activity) => update_activities.run(activity),
                        Err(error) => {
                            logging::error!("{error}")
                        }
                    }
                });
            }

            FormMode::Edit => {
                let name = Some(name.get());
                let description = Some(description.get());
                let activity_type = Some(activity_type.get());
                let amount = Some(amount.get());
                let activities_date = Some(activities_date.get());
                let due_date = Some(due_date.get());

                let data = UpdateActivity {
                    name,
                    description,
                    activities_date,
                    amount,
                    activity_type,
                    due_date,
                };

                spawn_local(async move {
                    // match update_activity(data).await {
                    //     Ok(activity) => update_activities.run(activity),
                    //     Err(error) => {
                    //         logging::error!("{error}")
                    //     }
                    // }
                });
            }
        }
    };

    view! {
        <div
            class="absolute h-screen transition-all animate-in fade-in duration-500 grid w-full content-center bg-black/30 backdrop-blur-lg z-20"
            class=(["block"], move || active_register_activity.get())
            class=(["hidden"], move || !active_register_activity.get())
            on:click=move |_| set_active_register_activity.set(false)
        >
            <h1 class="z-10 font-bold">
            {move || match mode.get() {
                    FormMode::Create => "Crear actividad",
                    FormMode::Edit => "Editar actividad"
                }}
            </h1>
            <div
                class="flex flex-col gap-4 w-full px-4 z-10"
                on:click=move |ev| ev.stop_propagation()
            >

                // Nombre
                <div class="flex flex-col gap-2">
                    <label class="text-sm font-semibold text-white/80">"Nombre"</label>

                    <input
                        type="text"
                        placeholder="Ej. Mensualidad agosto"
                        class="
                        w-full
                        rounded-2xl
                        border border-cyan-500/40
                        bg-cyan-400/10
                        px-4 py-3.5
                        text-white
                        placeholder:text-white/30
                        outline-none
                        transition-all
                        focus:border-pink-500/70
                        focus:bg-cyan-400/15
                        focus:ring-2
                        focus:ring-pink-500/20
                        "
                        prop:value=move || name.get()
                        on:input=move |ev| set_name.set(event_target_value(&ev))
                    />
                </div>

                // Descripción
                <div class="flex flex-col gap-2">
                    <label class="text-sm font-semibold text-white/80">"Descripción"</label>

                    <textarea
                        rows="3"
                        placeholder="Descripción opcional..."
                        class="
                        w-full
                        resize-none
                        rounded-2xl
                        border border-cyan-500/40
                        bg-cyan-400/10
                        px-4 py-3.5
                        text-white
                        placeholder:text-white/30
                        outline-none
                        transition-all
                        focus:border-pink-500/70
                        focus:bg-cyan-400/15
                        focus:ring-2
                        focus:ring-pink-500/20
                        "
                        prop:value=move || description.get()
                        on:input=move |ev| set_description.set(event_target_value(&ev))
                    ></textarea>
                </div>

                // Tipo
                <div class="flex flex-col gap-2">
                    <label class="text-sm font-semibold text-white/80">"Tipo de actividad"</label>

                    <select
                        class="
                        w-full
                        rounded-2xl
                        border border-cyan-500/40
                        bg-cyan-400/10
                        px-4 py-3.5
                        text-white
                        outline-none
                        transition-all
                        focus:border-pink-500/70
                        focus:ring-2
                        focus:ring-pink-500/20
                        "
                        // prop:value=move || type_activity.get()
                        on:change= move |ev| set_acitivity_type.set(event_target_value(&ev))
                    >
                        <option value="monthly">"Mensualidad"</option>
                        <option value="occasional">"Ocasional"</option>
                    </select>
                </div>

                // Monto
                <div class="flex flex-col gap-2">
                    <label class="text-sm font-semibold text-white/80">"Monto"</label>

                    <div class="
                    flex items-center
                    rounded-2xl
                    border border-cyan-500/40
                    bg-cyan-400/10
                    transition-all
                    focus-within:border-pink-500/70
                    focus-within:ring-2
                    focus-within:ring-pink-500/20
                    ">
                        <span class="pl-4 text-white/50 font-semibold">"S/"</span>

                        <input
                            type="number"
                            step="0.01"
                            min="0"
                            placeholder="0.00"
                            class="
                            w-full
                            bg-transparent
                            px-3 py-3.5
                            text-white
                            outline-none
                            placeholder:text-white/30
                            "
                            prop:value=move || amount.get()
                            on:input=move |ev| {
                                let value = event_target_value(&ev);
                                if let Ok(amount) = value.parse::<f64>() {
                                    set_amount.set(amount);
                                }
                            }
                        />
                    </div>
                </div>

                // Fechas
                <div class="grid grid-cols-2 gap-3">

                    <div class="flex flex-col gap-2">
                        <label class="text-sm font-semibold text-white/80">"Fecha"</label>

                        <input
                            type="date"
                            class="
                            w-full
                            rounded-2xl
                            border border-cyan-500/40
                            bg-cyan-400/10
                            px-3 py-3.5
                            text-sm text-white
                            outline-none
                            focus:border-pink-500/70
                            "
                            prop:value=move || activities_date.get()
                            on:change=move |ev| set_activities_date.set(event_target_value(&ev))
                        />
                    </div>

                    <div class="flex flex-col gap-2">
                        <label class="text-sm font-semibold text-white/80">"Vencimiento"</label>

                        <input
                            type="date"
                            class="
                            w-full
                            rounded-2xl
                            border border-cyan-500/40
                            bg-cyan-400/10
                            px-3 py-3.5
                            text-sm text-white
                            outline-none
                            focus:border-pink-500/70
                            "
                            prop:value=move || due_date.get()
                            on:change=move |ev| set_dute_date.set(event_target_value(&ev))
                        />
                    </div>

                </div>

                // Botón
                <button
                    type="submit"
                    class="
                    mt-2
                    w-full
                    rounded-2xl
                    bg-cyan-500
                    py-4
                    font-bold
                    text-black
                    transition-all
                    active:scale-[0.98]
                    "
                    on:click=handle_button_submit
                >
                    {move || match mode.get() {
                        FormMode::Create => "Crear actividad",
                        FormMode::Edit => "Guardar cambios"
                    }}
                </button>

            </div>
        </div>
    }
}
