use leptos::prelude::*;

#[component]
pub fn StudentForm(set_active_register_student: WriteSignal<bool>) -> impl IntoView {
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());

    view! {
        <h1 class="animate-in fade-in text-3xl font-bold z-10">"Registrar Alumno"</h1>

        <form
            class="animate-in fade-in mt-2 mx-4 rounded-3xl p-3 "
            on:click=move |ev| ev.stop_propagation()
        >
            <div class="p-3 rounded-3xl text-md">
                <label class="font-bold">"Nombres:  "</label>
                <input
                    class="outline-none border-1 border-cyan-600 focus:border-pink-600/80 ml-1 focus:border-3  transition-all focus:bg-cyan-400/30 bg-cyan-400/10 rounded-3xl p-3"
                    type="text"
                    value=move || first_name.get()
                    on:input=move |ev| { set_first_name.set(event_target_value(&ev)) }
                />
            </div>
            <div class="p-3 rounded-3xl text-md">
                <label class="font-bold">"Apellidos:  "</label>
                <input
                    class="outline-none border-1 border-cyan-600 ml-1 focus:border-3  transition-all
                      focus:bg-cyan-400/20 focus:border-pink-600/80 bg-cyan-400/10 rounded-3xl p-3"
                    type="text"
                    prop:value=move || last_name.get()
                    on:input=move |ev| { set_last_name.set(event_target_value(&ev)) }
                />
            </div>
        </form>
    }
}

#[component]
pub fn StudentFormModal(
    active_register_student: ReadSignal<bool>,
    set_active_register_student: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div
            class="absolute grid content-center z-20 w-full animate-in fade-in h-screen bg-black/30 backdrop-blur-lg"
            class=(["block"], move || active_register_student.get())
            class=(["hidden"], move || !active_register_student.get())
            on:click=move |_| { set_active_register_student.set(false) }
        >
            <StudentForm set_active_register_student=set_active_register_student />
        </div>
    }
}
