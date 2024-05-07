use leptos::*;

#[allow(non_snake_case)]
#[component]
pub fn ErrorModal(
    read_signal: ReadSignal<bool>,
    write_signal: WriteSignal<bool>,
    message: String,
) -> impl IntoView {
    let set_show_modal = write_signal;

    set_show_modal.set(true);

    view! {
        <Show when=move || read_signal.get() fallback=|| ()>
            <div class="blur-bg">
                <div class="modal-center">
                    <div class="modal-box bg-error rounded-box">
                        <h3 class="font-bold text-2xl text-black">ERROR!</h3>
                        <p class="py-4 text-black">{message.clone()}</p>
                        <div class="modal-action">
                            <button
                                class="btn btn-sm rounded"
                                title="Close"
                                on:click=move |_| set_show_modal.set(false)
                            >
                                Close
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}
