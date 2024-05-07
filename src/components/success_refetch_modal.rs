use leptos::*;

/// Component for a success modal that refetches a resource after closing the modal.
/// Similar to [`SuccessModalWithRefetch`] but has a function instead of a resource to allow for more flexibility.
/// Has only a read signal.
/// Has a message of type String.
/// Has a function.
///
/// ## Example
/// ```rust
/// let (show_success_modal, set_show_success_modal) = create_signal(false);
/// let refetch_resource = move || {
/// // Can do anything here unlike SuccessModalWithRefetch which is limited to refetching one resource.
///     set_show_success_modal.set(false);
///     active_quotes_resource.refetch();
/// };
///
/// view! {
///     <SuccessModalRefetch
///        read_signal = show_success_modal
///        message = "Successfully added quote".to_string()
///        resource = refetch_resource
///     />
/// }
/// ```

#[allow(non_snake_case)]
#[component]
pub fn SuccessModalRefetch<F>(
    read_signal: ReadSignal<bool>,
    message: String,
    mut function: F,
) -> impl IntoView
where
    F: FnMut() + Clone + 'static,
{
    let on_click = move |_| {
        function();
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
                                on:click=on_click.clone()
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
