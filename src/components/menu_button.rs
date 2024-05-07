use leptos::*;

use crate::components::icons::Icon;

#[allow(non_snake_case)]
#[component]
pub fn MenuButton(selected_page: RwSignal<String>, page: String, name: String) -> impl IntoView {
    let page_clone = page.clone();
    view! {
        <div class="flex-0">
            <button
                class=move || {
                    if selected_page.get() == page.clone() {
                        "btn btn-sm btn-ghost capitalize font-semibold bg-base-100 rounded-xl border border-success text-white hover:bg-success hover:bg-opacity-30"
                    } else {
                        "btn btn-sm btn-ghost capitalize font-normal bg-base-100 rounded-xl hover:bg-success hover:bg-opacity-30"
                    }
                }

                on:click=move |_| selected_page.set(page_clone.clone())
            >
                {name}
            </button>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn MenuButtonWithIcon(selected_page: RwSignal<String>, page: String, name: String, icon:String) -> impl IntoView {
    let page_clone = page.clone();
    view! {
        <div class = "flex-0">
            <button class = {move || if selected_page.get() == page.clone() {"btn btn-sm btn-ghost capitalize font-semibold bg-base-100 rounded-xl border border-success text-success"} else {"btn btn-sm btn-ghost capitalize font-normal bg-base-100 rounded-xl"}} on:click = move |_| selected_page.set(page_clone.clone())>
            <Icon title ={icon} size = String::from("w-4 h-4")/>
            {name}
            </button>
        </div>
    }
}