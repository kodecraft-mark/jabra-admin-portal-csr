use leptos::*;
/// Component for a success modal.
/// Has a read and write signal.
/// Has a message of type String.
///
/// ## Example
/// ```rust
/// let (show_success_modal, set_show_success_modal) = create_signal(false);
///
/// view! {
///     <SuccessModal
///        read_signal = show_success_modal
///        write_signal = set_show_success_modal
///        message = "Successfully added quote".to_string()
///     />
/// }
/// ```

#[allow(non_snake_case)]
#[component]
pub fn SuccessModal(
    read_signal: ReadSignal<bool>,
    write_signal: WriteSignal<bool>,
    message: String,
) -> impl IntoView {
    let set_show_modal = write_signal;

    set_show_modal.set(true);

    view! {

        <Show when=move || read_signal.get() fallback=|| ()>
            <div class="blur-bg">
                <div class="modal-top-middle">
                    <div class="modal-box bg-success rounded-box">
                        <h3 class="font-bold text-2xl text-black">SUCCESS!</h3>
                        <p class="py-4 text-black">{message.clone()}</p>
                        <div class="modal-action">
                            <button class="btn btn-sm rounded" title="Close" on:click = move |_| set_show_modal.set(false)>Close</button>
                        </div>
                    </div>
                </div>
            </div>
        </Show>

    }
}