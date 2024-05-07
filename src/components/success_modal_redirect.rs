use leptos::*;
use leptos_router::*;

#[allow(non_snake_case)]
#[component]
pub fn SuccessModalWithRedirection(
    read_signal: ReadSignal<bool>,
    write_signal: WriteSignal<bool>,
    message: String,
    url: String,
) -> impl IntoView {
    let set_show_modal = write_signal;

    set_show_modal.set(true);

    view! {

        <Show when=move || read_signal.get() fallback=|| ()>
            <div class="blur-bg">
                <div class="modal-top-middle">
                    <div class="modal-box bg-success rounded-box">
                        <h3 class="text-2xl font-bold text-black">SUCCESS!</h3>
                        <p class="py-4 text-black">{message.clone()}</p>
                        <div class="modal-action">
                        <Form action = {url.clone()} method = "get">
                            <button class="rounded btn btn-sm" title="Close">Close</button>
                        </Form> 
                        </div>
                    </div>
                </div>
            </div>
        </Show>

    }
}