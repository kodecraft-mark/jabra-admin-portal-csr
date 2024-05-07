use leptos::*;

use crate::{components::{component_size::ComponentSize, component_type::ComponentType, default_none::DefaultNone, error_modal::ErrorModal, loading_spinners::Spinners, success_refetch_modal::SuccessModalRefetch}, features::trades::termsheets::{models::{ApproveRejectTermSheetRequest, ApproveRejectTermSheetResponse}, services::{download_termsheet, fetch_new_term_sheet_list, post_approve_term_sheet}}, utilities::{date_util::convert_utc_to_local, number_util::format_number_en}};

use super::models::{GetNewTermSheetData, GetNewTermSheetResponse};

#[allow(non_snake_case)]
#[component]
pub fn TermSheets() -> impl IntoView {
    let show_notif_ar = create_rw_signal(false);

    let (approve_reject_response, set_approve_reject_response) =
        create_signal(ApproveRejectTermSheetResponse::default());

    let approve_reject_action: Action<(i64, String), bool> =
        create_action(move |(id, status): &(i64, String)| {
            let request = ApproveRejectTermSheetRequest {
                id: *id,
                status: status.into(),
            };
            let message = if status.to_string() == "Approve" {
                "Approval"
            } else {
                "Rejection"
            };
            async move {
                let result = post_approve_term_sheet(request).await;
                match result {
                    Ok(response) => match response {
                        true => {
                            show_notif_ar.set(true);
                            let message = format!("{} Successful", message);
                            set_approve_reject_response.set(ApproveRejectTermSheetResponse::new(
                                true, message,
                            ));
                            true
                        }
                        false => {
                            show_notif_ar.set(true);
                            let message = format!("{} Failed", message);
                            set_approve_reject_response.set(ApproveRejectTermSheetResponse::new(
                                false, message,
                            ));
                            false
                        }
                    },
                    Err(_e) => {
                        show_notif_ar.set(true);
                        let message = format!("{} Failed", message);
                        set_approve_reject_response.set(ApproveRejectTermSheetResponse::new(
                            false, message,
                        ));
                        false
                    }
                }
            }
        });

    let new_term_sheets_resource = create_local_resource(
        || (),
        move |_| async move { fetch_new_term_sheet_list().await },
    );

    view! {
        <NewTermSheetPage
            nts_resource = new_term_sheets_resource
            approve_reject_action = approve_reject_action
            show_notif_ar = show_notif_ar
            approve_reject_response = approve_reject_response
            set_approve_reject_response = set_approve_reject_response
        />
    }
}

#[allow(non_snake_case)]
#[component]
pub fn NewTermSheetPage(
    nts_resource: Resource<(), Result<GetNewTermSheetResponse, ServerFnError>>,
    approve_reject_action: Action<(i64, String), bool>,
    show_notif_ar: RwSignal<bool>,
    approve_reject_response: ReadSignal<ApproveRejectTermSheetResponse>,
    set_approve_reject_response: WriteSignal<ApproveRejectTermSheetResponse>,
) -> impl IntoView {

    let nts_response = RwSignal::new(GetNewTermSheetResponse::default());
    view! {
        <div class="h-full p-4">
        <div class = "flex-1 pb-5 ml-2 text-xl font-bold text-white">
            <span>New Term Sheets</span>
        </div>
        <Suspense fallback = move || view! {
            <div class = "items-center mt-5">
                <div class = "flex justify-center">
                    <Spinners size = ComponentSize::SMALL _type = ComponentType::SUCCESS />
                </div>
            </div>
        }>
            {
                move || {
                    if let Some(Ok(term_sheets)) = nts_resource.get() {
                        nts_response.set(term_sheets.clone());
                        view! {<div class = "p-2 overflow-auto rounded bg-base-100">
                                <table class = "table table-zebra-zebra table-xs">
                                    <thead>
                                        <tr class="text-sm text-white bg-opacity-50 bg-success">
                                            <th >ID</th>
                                            <th>COUNTERPARTY</th>
                                            <th>CURRENCY PAIR</th>
                                            <th>CURRENCY</th>
                                            <th>OPTION</th>
                                            <th>DEAL DATE</th>
                                            <th>EXPIRATION DATE</th>
                                            <th>ACTION</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {move ||
                                            nts_response.get().data.into_iter().map(|ts| {
                                                let (show, set_show) = create_signal(false);

                                                let show_val = move || {
                                                    if show.get() { "HIDE DETAILS" } else { "SHOW DETAILS" }
                                                };
                                                let (term_sheet, _set_term_sheet) = create_signal(ts.clone());

                                                view! {
                                                    <tr class = "hover hover:text-success">
                                                        <td>{ts.id}</td>
                                                        <td>{ts.counterparty_id.name}</td>
                                                        <td>{ts.pair_id.name}</td>
                                                        <td>{ts.deposit_ccy_id.ticker}</td>
                                                        <td>{ts.instrument_type}</td>
                                                        <td>{convert_utc_to_local(&ts.deal_date)}</td>
                                                        <td>{convert_utc_to_local(&ts.expiry_date)}</td>
                                                        <td><button class = "rounded-lg btn btn-success btn-xs" on:click = move |_| set_show.update(|s| *s = !*s)>{move || show_val()}</button></td>
                                                    </tr>
                                                    <Show when = move || show.get()>
                                                        <tr>
                                                            <td class = "content-center bg-base-100 border-s-4 border-base-300" colspan = "8">
                                                                <TermSheetDetails
                                                                    nts_resource = nts_resource
                                                                    term_sheet_data = term_sheet
                                                                    approve_reject_action = approve_reject_action
                                                                    show_notif_ar = show_notif_ar
                                                                    approve_reject_response = approve_reject_response
                                                                    set_approve_reject_response = set_approve_reject_response
                                                                />
                                                            </td>
                                                        </tr>
                                                    </Show>
                                                }
                                            }).collect_view()
                                        }
                                    </tbody>
                                </table>
                        </div>}
                    }else{
                        view! {
                            <div class="items-center mt-5">
                                <DefaultNone text = RwSignal::new(String::from("No Available New Term Sheets"))/>
                            </div>
                        }
                    }
                }
            }
        </Suspense>
        </div>
        
    }
}

/// Component for the Term Sheet Details.
/// Has the view for the term sheet details.
/// Shows when the user clicks the `SHOW DETAILS` button.

#[allow(non_snake_case)]
#[component]
pub fn TermSheetDetails(
    nts_resource: Resource<(), Result<GetNewTermSheetResponse, ServerFnError>>,
    term_sheet_data: ReadSignal<GetNewTermSheetData>,
    approve_reject_action: Action<(i64, String), bool>,
    show_notif_ar: RwSignal<bool>,
    approve_reject_response: ReadSignal<ApproveRejectTermSheetResponse>,
    set_approve_reject_response: WriteSignal<ApproveRejectTermSheetResponse>,
) -> impl IntoView {
    let approve_reject_dispatch = move |status: String| {
        let id = term_sheet_data.get().id;
        approve_reject_action.dispatch((id, status));
    };
    let (file_id, _set_file_id) = create_signal(term_sheet_data.get().term_sheet);
    let download_url_resource = create_local_resource(move || file_id.get(), move |_| {
        // let id = file_id().unwrap();
        let id = match file_id.get() {
            Some(id) => id,
            None => "".to_string(),
        };
        async move { download_termsheet(id).await }
    });

    let (show_confirm_modal_approve, set_show_confirm_modal_approve) = create_signal(false);
    let (show_confirm_modal_reject, set_show_confirm_modal_reject) = create_signal(false);

    let (show_success_modal, set_show_success_modal) = create_signal(false);
    let (show_error_modal, set_show_error_modal) = create_signal(false);

    let is_pending = approve_reject_action.pending();
    // let pseudo_pending = create_rw_signal(false);

    // Checks if an action has a value, then sets the show_modal to false, and resets the action_value to None
    create_effect(move |_| {
        let action_value = approve_reject_action.value();

        if let Some(_action) = action_value.get() {
            set_show_confirm_modal_approve.set(false);
            set_show_confirm_modal_reject.set(false);
            action_value.set(None);
        }
    });

    // AFTER CLOSING SUCCESS MODAL, CALL THIS FUNCTION TO REFETCH THE RESOURCE AND SET EVERYTHING TO DEFAULT
    let refetch_resource = move || {
        set_approve_reject_response.set(ApproveRejectTermSheetResponse::default());
        show_notif_ar.set(false);
        set_show_success_modal.set(false);
        set_show_error_modal.set(false);
        nts_resource.refetch();
    };

    view! {
        <div class = "p-2 bg-base-100">
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Counterparty Name</p></div>
            <div class = "flex-initial w-2/3"><p>{term_sheet_data.get().counterparty_id.name}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Trade Date</p></div>
            <div class = "flex-initial w-2/3"><p>{convert_utc_to_local(&term_sheet_data.get().deal_date)}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Term 2 Settlement Date</p></div>
            <div class = "flex-initial w-2/3"><p>{convert_utc_to_local(&term_sheet_data.get().expiry_date)}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Interest Price</p></div>
            <div class = "flex-initial w-2/32/3"><p>{term_sheet_data.get().px_in_quote_ccy}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Base Currency</p></div>
            <div class = "flex-initial w-2/3"><p>{term_sheet_data.get().base_ccy_id.ticker}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Trade Date Reference Exchange Rate</p></div>
            <div class = "flex-initial w-2/3"><p>{term_sheet_data.get().spot_t1}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Term Currency</p></div>
            <div class = "flex-initial w-2/3"><p>{term_sheet_data.get().term_ccy_id.ticker}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>{move || format!("Trade Date {} Notional Amount (deposit)", term_sheet_data.get().deposit_ccy_id.ticker)}</p></div>
            <div class = "flex-initial w-2/3"><p>{term_sheet_data.get().deposit_amount}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Term 2 Settlement Time</p></div>
            <div class = "flex-initial w-2/3"><p>{convert_utc_to_local(&term_sheet_data.get().expiry_date)}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Conditional Lost Limit Event</p></div>
            <div class = "flex-initial w-2/3"><p>{term_sheet_data.get().conditional_loss_limit_event.unwrap_or("N/A".to_string())}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Collateral Setting Methodology</p></div>
            <div class = "flex-initial w-2/3"><p>{term_sheet_data.get().collateral_setting_method}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>Collateral Exchange Settlement</p></div>
            <div class = "flex-initial w-2/3"><p>{term_sheet_data.get().collateral_exchange_settlement}</p></div>
        </div>
        <div class = "flex">
            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>CCY1/CCY2 exchange rate determining agent on T2</p></div>
            <div class = "flex-initial w-2/3"><p>{term_sheet_data.get().exchange_rate_determining_agent}</p></div>
        </div>
        {
                term_sheet_data.get().dcl_settlement_details.into_iter().map(|opt| {
                    view! {
                        <div class = "flex">
                            <div class = "flex-initial w-1/3 p-2 font-semibold"><p>{opt.settlement_condition}</p></div>
                            <div class = "flex-initial w-2/3"><p>{format_number_en(opt.settlement_value, 2)}</p></div>
                        </div>
                    }
                }).collect_view()
        }
        <div class = "flex">
            <div class = "flex-initial w-1/2 my-3">
                <button class = "mr-6 rounded-lg btn btn-primary btn-xs" on:click = move |_| set_show_confirm_modal_approve.set(true)>APPROVE</button>
                <button class = "mr-6 rounded-lg btn btn-error btn-xs" on:click = move |_| set_show_confirm_modal_reject.set(true)>REJECT</button>
                <Transition fallback = move || view! {<span class="inline-block loading loading-bars loading-xs"></span>} >
                    {
                        move || {
                            download_url_resource.get().map(|url|{
                                let uri = url.unwrap();
                                view! {
                                    <a class = "rounded-lg btn btn-warning btn-xs" href = uri download target = "_blank">DOWNLOAD</a>
                                }
                            }).collect_view()
                        }
                    }
                </Transition>
            </div>
            <div class = "flex-initial w-1/2"></div>
        </div>
        </div>

        {
            move || {

                view! {
                    <Show when=move || show_confirm_modal_approve.get() fallback=|| ()>
                        <div class="blur-bg">
                            <div class="modal-box modal-center">
                                <h3 class="text-2xl font-bold">APPROVE?</h3>
                                <p class="py-4">Are you sure you want to approve?</p>
                                <div class="modal-action">
                                    <button class="rounded btn btn-error btn-sm" prop:disabled=is_pending title="Cancel" on:click = move |_| set_show_confirm_modal_approve.set(false)>CANCEL</button>
                                    {
                                        match is_pending.get() {
                                            true => view! {
                                                <button class="rounded btn btn-success btn-sm" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
                                            }.into_any(),
                                            false => view! {
                                                <button class="rounded btn btn-success btn-sm" title="Confirm" on:click = move |_| approve_reject_dispatch(String::from("Approve"))>CONFIRM</button>
                                            }.into_any(),
                                        }
                                    }

                                </div>
                            </div>
                        </div>
                    </Show>
                }
            }
        }

        {
            move || {

                view! {
                    <Show when=move || show_confirm_modal_reject.get() fallback=|| ()>
                        <div class="blur-bg">
                            <div class="modal-box modal-center">
                                <h3 class="text-2xl font-bold">REJECT?</h3>
                                <p class="py-4">Are you sure you want to reject?</p>
                                <div class="modal-action">
                                    <button class="rounded btn btn-error btn-sm" prop:disabled=is_pending title="Cancel" on:click = move |_| set_show_confirm_modal_reject.set(false)>CANCEL</button>
                                    {
                                        match is_pending.get() {
                                            true => view! {
                                                <button class="rounded btn btn-success btn-sm" title="Confirm"><span class="loading loading-spinner loading-sm"></span></button>
                                            }.into_any(),
                                            false => view! {
                                                <button class="rounded btn btn-success btn-sm" title="Confirm" on:click = move |_| approve_reject_dispatch(String::from("Reject"))>CONFIRM</button>
                                            }.into_any(),
                                        }
                                    }

                                </div>
                            </div>
                        </div>
                    </Show>
                }
            }
        }

        {
            move || match show_notif_ar.get() {
                true => if !approve_reject_response.get().success  {
                    set_show_error_modal.set(true);
                    view! {
                        <ErrorModal
                            read_signal = show_error_modal
                            write_signal = set_show_error_modal
                            message = approve_reject_response.get().message
                        />
                        }
                } else {
                    set_show_success_modal.set(true);
                    view! {
                        <SuccessModalRefetch
                            read_signal = show_success_modal
                            message = approve_reject_response.get().message
                            function = refetch_resource
                        />
                    }
                }.into_view(),
                false => view! {<div></div>}.into_view(),
            }
        }
    }
}