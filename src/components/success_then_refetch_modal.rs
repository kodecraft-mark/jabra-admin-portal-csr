use leptos::*;
/// Component for a success modal that refetches a resource after closing the modal.
/// Has a read and write signal.
/// Has a message of type String.
/// Has a resource with two generic type parameters.
///
/// ## Example
/// ```rust
/// let (show_success_modal, set_show_success_modal) = create_signal(false);
///
/// view! {
///     <SuccessModalWithRefetch
///        read_signal = show_success_modal
///        write_signal = set_show_success_modal
///        message = "Successfully added quote".to_string()
///        resource = active_quotes_resource
///     />
/// }
/// ```

#[allow(non_snake_case)]
#[component]
pub fn SuccessModalWithRefetch<S, T>(
    read_signal: ReadSignal<bool>,
    write_signal: WriteSignal<bool>,
    message: String,
    resource: Resource<S, T>,
) -> impl IntoView
where
    S: Clone + 'static,
    T: 'static,
{
    let set_show_modal = write_signal;

    set_show_modal.set(true);
    let close_and_refetch = move || {
        set_show_modal.set(false);
        resource.refetch();
    };
    view! {
        <Show when=move || read_signal.get() fallback=|| ()>
            <div class="blur-bg">
                <div class="modal-top-middle">
                    <div class="modal-box bg-success rounded-box">
                        <h3 class="font-bold text-2xl text-black">SUCCESS!</h3>
                        <p class="py-4 text-black">{message.clone()}</p>
                        <div class="modal-action">
                            <button
                                class="btn btn-sm rounded"
                                title="Close"
                                on:click=move |_| close_and_refetch()
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
