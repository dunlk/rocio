use leptos::prelude::*;

#[component]
pub fn StudentForm() -> impl IntoView {
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());

    view! {
        <h1 class="animate-in fade-in text-3xl font-bold">Registrar alumno</h1>

        <div class="animate-in fade-in bg-black/30 mt-3 backdrop-blur-md rounded-3xl p-3 shadow-black/20 shadow-lg ">
            <div class="p-3 rounded-3xl text-md">
                <label class="font-bold">"Nombres:  "</label>
                <input
                    class="outline-none border-1 border-cyan-600 focus:border-pink-600/80 ml-1 focus:border-3  transition-all focus:bg-cyan-400/40 bg-cyan-400/10 rounded-3xl p-3"
                    type="text"
                    value=move || first_name.get()
                    on:input=move |ev| {
                        set_first_name.set(event_target_value(&ev))
                    }
                />
            </div>
            <div class="p-3 rounded-3xl text-md">
                <label class="font-bold">"Apellidos:  "</label>
                <input
                    class="outline-none border-1 border-cyan-600 ml-1 focus:border-3  transition-all
                            focus:bg-cyan-400/40 focus:border-pink-600/80 bg-cyan-400/10 rounded-3xl p-3"
                    type="text"
                    prop:value={move || first_name.get()}
                    on:input={move |ev| {
                            set_last_name.set(event_target_value(&ev))
                    }}
                />
            </div>
        </div>
    }
}
