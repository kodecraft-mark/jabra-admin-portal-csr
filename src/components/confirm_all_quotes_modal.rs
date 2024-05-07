use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn ConfirmModalAllQuotes<C>(
    signal: RwSignal<bool>,
    mut function: C,
    params: String,
    pending_signal: ReadSignal<bool>,
    title: String,
) -> impl IntoView
where
    C: FnMut(String) + Clone + 'static,
{
    let on_click = move |_| function(params.clone());

    view! {
        <Show when=move || signal.get() fallback=|| ()>
            <div class="blur-bg">
                <div class="modal-top-middle">
                    <div class="modal-box rounded-box">
                        <h3 class="font-bold text-2xl">{title.clone()} ?</h3>
                        <p class="py-4">Are you sure you want to {title.clone()} ?</p>
                        <div class="modal-action">
                            <button
                                class="btn btn-error btn-sm rounded"
                                prop:disabled=pending_signal
                                title="Cancel"
                                on:click=move |_| signal.set(false)
                            >
                                Cancel
                            </button>

                            {match pending_signal() {
                                true => {
                                    view! {
                                        <button
                                            class="btn btn-success btn-sm rounded"
                                            title="Confirm"
                                        >
                                            <span class="loading loading-spinner loading-sm"></span>
                                        </button>
                                    }
                                        .into_any()
                                }
                                false => {
                                    view! {
                                        <button
                                            class="btn btn-success btn-sm rounded"
                                            title="Confirm"
                                            on:click=on_click.clone()
                                        >
                                            Confirm
                                        </button>
                                    }
                                        .into_any()
                                }
                            }}

                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
