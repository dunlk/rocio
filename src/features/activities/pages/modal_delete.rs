use leptos::logging;
use leptos::prelude::*;
use leptos::reactive::spawn_local;

use crate::features::activities::services::delete_activity;
#[component]
pub fn ModalDelete(id: ReadSignal<Option<i64>>, set_id: WriteSignal<Option<i64>>) -> impl IntoView {
    let handle_confirm = move |_: leptos::ev::MouseEvent| {
        // logging::log!("id:{:?}", id)
        spawn_local(async move {
            if let Some(id) = id.get_untracked() {
                match delete_activity(id).await {
                    Ok(_) => set_id.set(None),
                    Err(error) => logging::error!("{error}"),
                }
                // set_id.set(None);
            }
        });
    };

    view! {
        <Show when=move || id.get().is_some()>
            // Overlay
            <div class="
                fixed inset-0 z-50
                flex items-center justify-center
                bg-black/50 backdrop-blur-sm
                px-5
            ">
                // Modal
                <div class="
                    w-full max-w-[340px]
                    rounded-3xl
                    bg-[#0b2239]/95
                    border border-cyan-400/20
                    shadow-2xl shadow-black/30
                    p-6
                    text-white
                    animate-in fade-in zoom-in-95
                    duration-200
                ">

                    // Icono
                    <div class="
                        mx-auto
                        w-14 h-14
                        rounded-full
                        flex items-center justify-center
                        bg-pink-500/15
                        border border-pink-400/30
                        text-pink-400
                        text-2xl
                    ">
                        "!"
                    </div>

                    // Texto
                    <div class="text-center mt-4">
                        <h2 class="text-xl font-bold">
                            "Eliminar actividad"
                        </h2>

                        <p class="text-sm text-white/50 mt-2">
                            "¿Estás seguro de que quieres eliminar este alumno?"
                        </p>

                        <p class="text-xs text-pink-300/70 mt-1">
                            "Esta acción no se puede deshacer."
                        </p>
                    </div>

                    // Botones
                    <div class="flex gap-3 mt-6">

                        <button
                            class="
                                flex-1
                                rounded-2xl
                                py-3
                                bg-white/5
                                border border-white/10
                                text-white/70
                                font-semibold
                                active:scale-95
                                transition-all
                            "
                            on:click=move |_| {
                                set_id.set(None);
                            }
                        >
                            "Cancelar"
                        </button>

                        <button
                            class="
                                flex-1
                                rounded-2xl
                                py-3
                                bg-pink-500/20
                                border border-pink-400/40
                                text-pink-300
                                font-semibold
                                active:scale-95
                                transition-all
                            "
                            on:click=handle_confirm
                        >
                            "Eliminar"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
