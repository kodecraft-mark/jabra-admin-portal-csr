use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn DefaultNone(text: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="p-5">
            <span class="opacity-50 font-extralight">{ move || text.get()}</span>
        </div>
    }
}