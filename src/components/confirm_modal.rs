use leptos::*;

/// Component for a modal that has a confirm and cancel button.
/// Executes a function when the confirm button is clicked.
/// Has a pending state that shows a loading spinner when the action is pending.
///
/// ## Example
/// ```rust
/// let (show_confirm_modal, set_show_confirm_modal) = create_signal(false);
/// let call_action_fn = move || {add_quote_action.dispatch()};
/// let add_quote_action: Action<Vec<Quote>, ()> = create_action(/*action here*/);
///
/// <ConfirmModal
///    when = show_confirm_modal.get()
///    write_signal = set_show_confirm_modal
///    function = call_action_fn
///    action = add_quote_action
/// />
/// ```

#[allow(non_snake_case)]
#[component]
pub fn ConfirmModal<C, T>(
    when: bool,
    write_signal: WriteSignal<bool>,
    mut function: C,
    action: Action<T, ()>,
) -> impl IntoView
where
    C: FnMut() + Clone + 'static,
    T: Clone + 'static,
{
    let on_click = move |_| function();
    let is_pending = action.pending();
    let set_show_modal = write_signal;

    // Checks if an action has a value, then sets the show_modal to false, and resets the action_value to None
    create_effect(move |_| {
        let action_value = action.value();

        if let Some(_action) = action_value.get() {
            set_show_modal.set(false);
            action_value.set(None);
        }
    });

    view! {

        <Show when=move || when fallback=|| ()>
            <div class="blur-bg">
                <div class="modal-top-middle">
                    <div class="modal-box rounded-box">
                        <h3 class="text-2xl font-bold">CONFIRM?</h3>
                        <p class="py-4">Are you sure you want to confirm?</p>
                        <div class="modal-action">
                            <button class="rounded btn btn-error btn-sm" prop:disabled=is_pending title="Cancel" on:click = move |_| set_show_modal.set(false)>Cancel</button>
                            {
                                match is_pending.get() {
                                    true => view! {
                                        <button class="rounded btn btn-success btn-sm" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
                                    }.into_any(),
                                    false => view! {
                                        <button class="rounded btn btn-success btn-sm" title="Confirm" on:click = on_click.clone()>Confirm</button>
                                    }.into_any(),
                                }
                            }

                        </div>
                    </div>
                </div>
            </div>
        </Show>

    }
}