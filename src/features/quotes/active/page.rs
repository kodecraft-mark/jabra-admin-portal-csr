use std::collections::BTreeMap;
use std::collections::HashSet;

use leptos::html::Q;
use leptos::*;
use crate::commons::models::counterparty::CounterParty;
use crate::commons::models::counterparty::GetCounterPartiesResponse;
use crate::commons::models::quote::{
    ApproveTradeQuoteResponse, ModifyQuoteResponse, QuoteOption, QuotesOptionForStatusChange,
    QuotesOptionsForModification,
};
use crate::commons::services::counterparty::get_counter_parties;
use crate::commons::services::quote::{
    approve_reject_quotes_option, edit_quotes_option, get_quotes_option,
    get_quotes_option_under_24_hrs,
};
use crate::components::arrow_down::ArrowDown;
use crate::components::arrow_up::ArrowUp;
use crate::components::component_size::ComponentSize;
use crate::components::component_type::ComponentType;
use crate::components::confirm_all_quotes_modal::ConfirmModalAllQuotes;
use crate::components::confirm_batch_quotes_modal::ConfirmModalBatchQuotes;
use crate::components::data_table_local::*;
use crate::components::error_modal::ErrorModal;
use crate::components::loading_spinners::Spinners;
use crate::components::success_refetch_modal::SuccessModalRefetch;
use crate::components::success_then_refetch_modal::SuccessModalWithRefetch;
use crate::utilities::date_util::{
    calculate_time_difference, convert_utc_to_local, format_date, parse_str_to_utc_datetime_str,
};
use chrono::Utc;
use serde_json::Value;

#[allow(non_snake_case)]
#[component]
pub fn ActiveQuotes() -> impl IntoView {
    let counterparties_resource: Resource<(), Result<GetCounterPartiesResponse, ServerFnError>> =
        create_local_resource(|| (), move |_| get_counter_parties());
    let counterparties = RwSignal::new(Vec::<CounterParty>::default());
    view! {
        <Suspense>

            {move || {
                counterparties_resource
                    .and_then(|cpr| {
                        counterparties.set(cpr.data.clone());
                    })
            }}

        </Suspense>
        <ActiveQuotesPage counterparties=counterparties/>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn ActiveQuotesPage(counterparties: RwSignal<Vec<CounterParty>>) -> impl IntoView {
    let active_quotes_resource = create_local_resource(|| (), move |_| get_quotes_option(String::from("active")));
    let approved_quotes_resource =
        create_local_resource(|| (), move |_| get_quotes_option_under_24_hrs(String::from("approved")));
    let rejected_quotes_resource =
        create_local_resource(|| (), move |_| get_quotes_option_under_24_hrs(String::from("rejected")));

    let show_approve_quote_alert = create_rw_signal(false);
    let approve_quote_response = create_rw_signal(ApproveTradeQuoteResponse::default());

    let selected_header_rejected = RwSignal::new(String::from(""));
    let sort_asc_rejected = RwSignal::new(true);
    let has_rejected = RwSignal::new(false);

    let selected_header_approved = RwSignal::new(String::from(""));
    let sort_asc_approved = RwSignal::new(true);
    let has_approved = RwSignal::new(false);

    let selected_header_pending = RwSignal::new(String::from(""));
    let sort_asc_pending = RwSignal::new(true);
    let has_pending = RwSignal::new(false);

    let approve_reject_quotes_option_action: Action<
        (String, Vec<QuotesOptionForStatusChange>),
        (),
    > = create_action(
        move |(status, req): &(String, Vec<QuotesOptionForStatusChange>)| {
            let stat = if status.clone() == "approved" {
                "approval".to_string()
            } else {
                "rejection".to_string()
            };
            // let request = ApproveTradeQuoteRequest::new(group_ids.clone(), status.clone());
            let request = req.clone();
            async move {
                // approve_trade_quote(request).await
                let result = approve_reject_quotes_option(request).await;
                match result {
                    Ok(res) => {
                        if res {
                            show_approve_quote_alert.set(true);
                            approve_quote_response.update(|v| {
                                v.success = true;
                                v.message = format!("Trade quote {} is successful", stat)
                            });
                        } else {
                            show_approve_quote_alert.set(true);
                            approve_quote_response.update(|v| {
                                v.success = false;
                                v.message = "Failed request, Please try again!.".to_string()
                            });
                        }
                    }
                    Err(_e) => {
                        show_approve_quote_alert.set(true);
                        approve_quote_response.update(|v| {
                            v.success = false;
                            v.message =
                                "Your session has ended. Please relog your account.".to_string()
                        });
                    }
                }
            }
        },
    );

    let headers = RwSignal::new(
        vec![
            String::from("DATE CREATED"),
            String::from("INSTRUMENT"),
            String::from("AMOUNT"),
            String::from("SIDE"),
            String::from("SPOT"),
            String::from("STRIKE"),
            String::from("SPOT %"),
            String::from("IV"),
            String::from("BASE QUOTE"),
            String::from("QUOTE COST"),
            String::from("EXPIRES IN"),
            String::from("DELTA"),
            String::from("GAMMA"),
            String::from("THETA"),
            String::from("SELECT"),
            String::from("ACTION"),
        ],
    );
    let header_key_params = RwSignal::new(
        vec![
            String::from("date_created"),
            String::from("instrument_name"),
            String::from("amount"),
            String::from("side"),
            String::from("spot"),
            String::from("strike"),
            String::from("offstrike_percentage"),
            String::from("iv"),
            String::from("px_in_base_ccy"),
            String::from("px_in_quote_ccy"),
            String::from("quote_expiry"),
            String::from("gamma"),
            String::from("theta"),
            String::from(""),
        ],
    );

    let confirm_modal_approve_all = create_rw_signal(false);
    let confirm_modal_reject_all = create_rw_signal(false);

    let is_pending = approve_reject_quotes_option_action.pending();

    view! {
        <div class="p-4">
            <ActiveQuotesTable
                has_pending
                confirm_modal_reject_all
                confirm_modal_approve_all
                headers
                selected_header_pending
                header_key_params
                sort_asc_pending
                active_quotes_resource
                approved_quotes_resource
                rejected_quotes_resource
                approve_reject_quotes_option_action
                is_pending
                show_approve_quote_alert
                approve_quote_response
                counterparties
            />
            <ApprovedQuotesTable
                has_approved
                sort_asc_approved
                selected_header_approved
                approved_quotes_resource
            />
            <RejectedQuotesTable
                has_rejected
                sort_asc_rejected
                selected_header_rejected
                rejected_quotes_resource
            />
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn ActiveQuotesTable(
    has_pending: RwSignal<bool>,
    confirm_modal_reject_all: RwSignal<bool>,
    confirm_modal_approve_all: RwSignal<bool>,
    headers: RwSignal<Vec<String>>,
    selected_header_pending: RwSignal<String>,
    header_key_params: RwSignal<Vec<String>>,
    sort_asc_pending: RwSignal<bool>,
    active_quotes_resource: Resource<(), Result<BTreeMap<String, Vec<QuoteOption>>, ServerFnError>>,
    approved_quotes_resource: Resource<(), Result<BTreeMap<String, Vec<QuoteOption>>, ServerFnError>>,
    rejected_quotes_resource: Resource<(), Result<BTreeMap<String, Vec<QuoteOption>>, ServerFnError>>,
    approve_reject_quotes_option_action: Action<(String, Vec<QuotesOptionForStatusChange>), ()>,
    is_pending: ReadSignal<bool>,
    show_approve_quote_alert: RwSignal<bool>,
    approve_quote_response: RwSignal<ApproveTradeQuoteResponse>,
    counterparties: RwSignal<Vec<CounterParty>>
) -> impl IntoView {

    // Signal for modals
    let (show_success_modal, set_show_success_modal) = create_signal(false);
    let (show_error_modal, set_show_error_modal) = create_signal(false);

    // AFTER CLOSING SUCCESS MODAL, CALL THIS FUNCTION TO REFETCH THE RESOURCE AND SET EVERYTHING TO DEFAULT
    let refetch_resource = move || {
        show_approve_quote_alert.set(false);
        set_show_success_modal.set(false);
        set_show_error_modal.set(false);
        active_quotes_resource.refetch();
        approved_quotes_resource.refetch();
        rejected_quotes_resource.refetch();
    };

    view! {
        <div class="text-xl text-white font-semibold ml-2 mb-3">
            <span>Active Quotes</span>
        </div>
        <div class="py-3 mb-3">
            <div class="p-3 mb-3 overflow-auto rounded-md shadow-sm border-opacity-30 shadow-success bg-base-100">
                <table class="table overflow-x-visible table-zebra table-xs">
                    <Show when=move || has_pending.get()>
                        <thead class="text-md bg-opacity-30 text-white">
                            <th colspan={format!("{}", headers.get().len())}>
                                <div class="hidden md:flex justify-end gap-2">
                                    <button
                                        class="btn btn-xs btn-warning"
                                        on:click=move |_| confirm_modal_reject_all.set(true)
                                    >
                                        REJECT ALL
                                    </button>
                                    <button
                                        class="btn btn-xs btn-success"
                                        on:click=move |_| confirm_modal_approve_all.set(true)
                                    >
                                        APPROVE ALL
                                    </button>
                                </div>
                            </th>
                        </thead>
                        <thead class="text-md bg-success bg-opacity-30 text-white">
                            <tr class="hidden px924:contents">

                                {
                                    headers
                                        .get()
                                        .into_iter()
                                        .enumerate()
                                        .map(|(index, key_name)| {
                                            view! {
                                                // header_keys.into_iter().map(|k| {
                                                // view! {
                                                // <GenericDataTableHeader header_selector = selected_header_rejected sorter = k.clone() sort_direction = sort_asc_rejected name = k.clone() />
                                                // }
                                                // }).collect_view()
                                                // let selected_header_param = keys.get()[index];
                                                <th
                                                    class="cursor-pointer"
                                                    on:click=move |_| {
                                                        selected_header_pending
                                                            .set(header_key_params.get()[index].clone());
                                                        sort_asc_pending.update(|s| *s = !*s)
                                                    }
                                                >

                                                    <div class="flex justify-between">
                                                        <span class="flex-0">{key_name}</span>
                                                        <span class="flex-0">
                                                            <svg
                                                                xmlns="http://www.w3.org/2000/svg"
                                                                viewBox="0 0 20 20"
                                                                fill="currentColor"
                                                                class="w-5 h-5"
                                                            >
                                                                <path
                                                                    fill-rule="evenodd"
                                                                    d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z"
                                                                    clip-rule="evenodd"
                                                                ></path>
                                                            </svg>
                                                        </span>
                                                    </div>
                                                </th>
                                            }
                                        })
                                        .collect_view()
                                }

                            </tr>
                        </thead>
                    </Show>
                    <Transition fallback=move || {
                        view! {
                            <div class="flex justify-center ">
                                <Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS />
                            </div>
                        }
                    }>

                    {
                        move || {
                            active_quotes_resource
                                .and_then(|e| {
                                    if e.is_empty() {
                                        has_pending.set(false);
                                        return view! {
                                            <div class="flex justify-center border-b border-b-gray-700">
                                                <p>No available active quotes</p>
                                            </div>
                                        }.into_view();
                                    }

                                    let all_group_ids = create_rw_signal(vec!["".to_string()]);
                                    let mut _all_group_ids: Vec<String> = Vec::new();
                                    let hashmap = create_rw_signal(e.clone());

                                    for quote_options in hashmap.get().values() {
                                        // Iterate over the Vec<QuoteOption>
                                        for quote_option in quote_options {
                                            if !_all_group_ids.contains(&quote_option.group_id) {
                                                _all_group_ids.push(quote_option.group_id.clone());
                                            }
                                        }
                                    }

                                    all_group_ids.set(_all_group_ids.clone());

                                    e.into_iter()
                                        .map(|(counter_party, trade_quotes)| {
                                            has_pending.set(true);
                                            let counterparty_name = counter_party
                                                .split("~")
                                                .collect::<Vec<&str>>()[0]
                                                .to_string();
                                            let counterparty_id = counter_party
                                                .split("~")
                                                .collect::<Vec<&str>>()[1]
                                                .to_string();
                                            let counterparty_id_as_int = counterparty_id
                                                .parse::<u16>()
                                                .unwrap();
                                            let download_uri_resource = create_resource(
                                                || (),
                                                move |_| {
                                                    let c = counterparty_id.clone();
                                                    async move {
                                                        get_download_url(c, "active".to_string()).await
                                                    }
                                                },
                                            );
                                            let (group_ids, set_group_ids) = create_signal(
                                                Vec::<String>::new(),
                                            );
                                            let (trade_quotes_list, set_trade_quotes_list) = create_signal(
                                                trade_quotes.clone(),
                                            );
                                            let json_value_pending = Signal::derive(move || {
                                                trade_quotes_list
                                                    .get()
                                                    .into_iter()
                                                    .map(serde_json::to_value)
                                                    .collect::<Result<Vec<serde_json::Value>, _>>()
                                                    .expect("Failed to serialize to JSON")
                                            });
                                            let header_keys_number_pending = RwSignal::new(
                                                vec![
                                                    String::from("amount"),
                                                    String::from("side"),
                                                    String::from("spot"),
                                                    String::from("strike"),
                                                    String::from("offstrike_percentage"),
                                                    String::from("iv"),
                                                    String::from("px_in_base_ccy"),
                                                    String::from("px_in_quote_ccy"),
                                                    String::from("gamma"),
                                                    String::from("theta"),
                                                ],
                                            );
                                            let tr_pending = Signal::derive(move || {
                                                sort_table(
                                                    json_value_pending.get(),
                                                    sort_asc_pending.get(),
                                                    selected_header_pending.get(),
                                                    header_keys_number_pending.get(),
                                                )
                                            });
                                            let dispatch_accept_trade_quote = move |
                                                status: String,
                                                gi: Vec<String>|
                                            {
                                                let mut quotes_option_for_status_change: Vec<
                                                    QuotesOptionForStatusChange,
                                                > = Vec::<QuotesOptionForStatusChange>::default();
                                                for quote_option in trade_quotes_list.get().into_iter() {
                                                    if gi.contains(&quote_option.group_id) {
                                                        quotes_option_for_status_change
                                                            .push(
                                                                QuotesOptionForStatusChange::new(
                                                                    quote_option.id,
                                                                    status.clone(),
                                                                    format_date(Utc::now()),
                                                                ),
                                                            );
                                                    }
                                                }
                                                approve_reject_quotes_option_action
                                                    .dispatch((
                                                        status.clone(),
                                                        quotes_option_for_status_change,
                                                    ));
                                                set_trade_quotes_list
                                                    .update(|v| {
                                                        v.retain(|x| !gi.contains(&x.group_id));
                                                    });
                                            };
                                            let hide_active_table = move || {
                                                trade_quotes_list.get().len() < 1
                                            };


                                            let approve_reject_all = move |status: String| {
                                                let mut quotes_option_for_status_change = Vec::<QuotesOptionForStatusChange>::default();

                                                // let _status = status.clone();
                                                for quote_options in hashmap.get().values() {
                                                    for quote_option in quote_options {
                                                        if all_group_ids.get().contains(&quote_option.group_id) {
                                                            quotes_option_for_status_change.push(QuotesOptionForStatusChange::new(quote_option.id, status.clone(), format_date(Utc::now())));
                                                        }
                                                    }
                                                }
        
                                                approve_reject_quotes_option_action
                                                    .dispatch((
                                                        status.clone(),
                                                        quotes_option_for_status_change.clone(),
                                                    ));
                                                // set_trade_quotes_list
                                                //     .update(|v| {
                                                //         v.retain(|x| !_all_group_ids.contains(&x.group_id));
                                                //     });
                                            };
                                            let confirm_modal_approve = create_rw_signal(false);
                                            let confirm_modal_reject = create_rw_signal(false);
                                            create_effect(move |_| {
                                                let value = approve_reject_quotes_option_action.value();
                                                if let Some(_data) = value.get() {
                                                    confirm_modal_approve.set(false);
                                                    confirm_modal_reject.set(false);
                                                    confirm_modal_approve_all.set(false);
                                                    confirm_modal_reject_all.set(false);
                                                    value.set(None)
                                                }
                                            });
                                            view! {
                                                // Use this to store the group_id of the trade quotes that have been selected

                                                // Clone all trade quotes of a certain counter party here to enable reactivity

                                                // TO CHECK IF THE ACTION IS PENDING

                                                // Signals for confirm modals

                                                {
                                                    match tr_pending.get().is_empty() {
                                                        true => {
                                                            view! {
                                                                {
                                                                    has_pending.set(false);
                                                                }

                                                                <div class="flex justify-center border-b border-b-gray-700">
                                                                    <p>No available active quotes</p>
                                                                </div>
                                                            }
                                                                .into_view()
                                                        }
                                                        false => {
                                                            view! {
                                                                <tbody>
                                                                    <tr class="text-base bg-base-200 ">
                                                                        <th class="text-success" colspan="16">
                                                                            {counterparty_name.clone()}
                                                                        </th>
                                                                    </tr>

                                                                    {move || {
                                                                        has_pending.set(true);
                                                                        tr_pending
                                                                            .get()
                                                                            .into_iter()
                                                                            .map(|tq| {
                                                                                let show_edit_modal = RwSignal::new(false);
                                                                                let mod_counterparty = RwSignal::new(
                                                                                    counterparty_id_as_int,
                                                                                );
                                                                                let counter_party = match &tq["counterparty_id"]["name"] {
                                                                                    Value::String(s) => s,
                                                                                    _ => &String::from(""),
                                                                                };
                                                                                let date_created = match &tq["date_created"] {
                                                                                    Value::String(s) => s,
                                                                                    _ => &String::from(""),
                                                                                };
                                                                                let amount = match &tq["amount"] {
                                                                                    Value::Number(s) => {
                                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                                    }
                                                                                    _ => 0.0,
                                                                                };
                                                                                let instrument_name = match &tq["instrument_name"] {
                                                                                    Value::String(s) => s,
                                                                                    _ => &String::from(""),
                                                                                };
                                                                                let side = match &tq["side"] {
                                                                                    Value::String(s) => s,
                                                                                    _ => &String::from(""),
                                                                                };
                                                                                let px_in_base_ccy = match &tq["px_in_base_ccy"] {
                                                                                    Value::Number(s) => {
                                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                                    }
                                                                                    _ => 0.0,
                                                                                };
                                                                                let px_in_quote_ccy = match &tq["px_in_quote_ccy"] {
                                                                                    Value::Number(s) => {
                                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                                    }
                                                                                    _ => 0.0,
                                                                                };
                                                                                let offstrike_percentage = match &tq["offstrike_percentage"]
                                                                                {
                                                                                    Value::Number(s) => {
                                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                                    }
                                                                                    _ => 0.0,
                                                                                };
                                                                                let spot = match &tq["spot"] {
                                                                                    Value::Number(s) => {
                                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                                    }
                                                                                    _ => 0.0,
                                                                                };
                                                                                let strike = match &tq["strike"] {
                                                                                    Value::Number(s) => {
                                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                                    }
                                                                                    _ => 0.0,
                                                                                };
                                                                                let iv = match &tq["iv"] {
                                                                                    Value::Number(s) => {
                                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                                    }
                                                                                    _ => 0.0,
                                                                                };
                                                                                let quote_expiry = match &tq["quote_expiry"] {
                                                                                    Value::String(s) => s,
                                                                                    _ => &String::from(""),
                                                                                };
                                                                                let gtc = match &tq["gtc"] {
                                                                                    Value::Bool(s) => s,
                                                                                    _ => &true,
                                                                                };
                                                                                let delta = match &tq["delta"] {
                                                                                    Value::Number(s) => {
                                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                                    }
                                                                                    _ => 0.0,
                                                                                };
                                                                                let gamma = match &tq["gamma"] {
                                                                                    Value::Number(s) => {
                                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                                    }
                                                                                    _ => 0.0,
                                                                                };
                                                                                let theta = match &tq["theta"] {
                                                                                    Value::Number(s) => {
                                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                                    }
                                                                                    _ => 0.0,
                                                                                };
                                                                                let group_id = match &tq["group_id"] {
                                                                                    Value::String(s) => s,
                                                                                    _ => &String::from(""),
                                                                                };
                                                                                let party_b_ = match &tq["party_b"] {
                                                                                    serde_json::Value::Object(party_b) => {
                                                                                        match party_b.get("name") {
                                                                                            Some(name) => {
                                                                                                match name.as_str() {
                                                                                                    Some(name_str) => name_str.to_string(),
                                                                                                    None => String::from(""),
                                                                                                }
                                                                                            }
                                                                                            None => String::from(""),
                                                                                        }
                                                                                    }
                                                                                    _ => String::from(""),
                                                                                };
                                                                                let filtered_data: QuoteOption = trade_quotes_list
                                                                                    .get()
                                                                                    .into_iter()
                                                                                    .find(|item| {
                                                                                        item.group_id == *group_id
                                                                                            && item
                                                                                                .party_b
                                                                                                .as_ref()
                                                                                                .map_or(false, |party_b| party_b.name == *party_b_)
                                                                                    })
                                                                                    .expect("REASON");
                                                                                let mod_quote_option = RwSignal::new(filtered_data);
                                                                                let hide_non_jabra = counter_party != "JABRA";
                                                                                let is_pos_class_s = if side == "Sell" {
                                                                                    "text-error"
                                                                                } else {
                                                                                    "text-success"
                                                                                };
                                                                                let is_pos_class_a = if amount >= 0.0 {
                                                                                    "text-success"
                                                                                } else {
                                                                                    "text-error"
                                                                                };
                                                                                let is_pos_value_a = if amount >= 0.0 {
                                                                                    format!("+{}", amount)
                                                                                } else {
                                                                                    amount.to_string()
                                                                                };
                                                                                let is_pos_class_b = if px_in_base_ccy >= 0.0 {
                                                                                    "text-success"
                                                                                } else {
                                                                                    "text-error"
                                                                                };
                                                                                let is_pos_value_b = if px_in_base_ccy >= 0.0 {
                                                                                    format!("+{}", px_in_base_ccy)
                                                                                } else {
                                                                                    px_in_base_ccy.to_string()
                                                                                };
                                                                                let is_pos_class_q = if px_in_quote_ccy >= 0.0 {
                                                                                    "text-success"
                                                                                } else {
                                                                                    "text-error"
                                                                                };
                                                                                let is_pos_value_q = if px_in_quote_ccy >= 0.0 {
                                                                                    format!("+{}", px_in_quote_ccy)
                                                                                } else {
                                                                                    px_in_quote_ccy.to_string()
                                                                                };
                                                                                let modify_quote_response = RwSignal::new(
                                                                                    ModifyQuoteResponse::default(),
                                                                                );
                                                                                let show_modify_alert_modal = RwSignal::new(false);
                                                                                let (
                                                                                    show_modify_success_modal,
                                                                                    set_show_modify_success_modal,
                                                                                ) = create_signal(true);
                                                                                let (
                                                                                    show_modify_error_modal,
                                                                                    set_show_modify_error_modal,
                                                                                ) = create_signal(true);
                                                                                let modify_quote_action: Action<
                                                                                    Vec<QuotesOptionsForModification>,
                                                                                    (),
                                                                                > = create_action(move |
                                                                                    req: &Vec<QuotesOptionsForModification>|
                                                                                {
                                                                                    let request = req.clone();
                                                                                    async move {
                                                                                        let result = edit_quotes_option(request).await;
                                                                                        match result {
                                                                                            Ok(res) => {
                                                                                                if res {
                                                                                                    show_modify_alert_modal.set(true);
                                                                                                    modify_quote_response
                                                                                                        .update(|v| {
                                                                                                            v.success = true;
                                                                                                            v.message = String::from("Quote Option update successful.");
                                                                                                        });
                                                                                                } else {
                                                                                                    show_modify_alert_modal.set(true);
                                                                                                    modify_quote_response
                                                                                                        .update(|v| {
                                                                                                            v.success = false;
                                                                                                            v
                                                                                                                .message = String::from(
                                                                                                                "Failed request, Please try again!.",
                                                                                                            );
                                                                                                        });
                                                                                                }
                                                                                            }
                                                                                            Err(_e) => {
                                                                                                show_modify_alert_modal.set(true);
                                                                                                modify_quote_response
                                                                                                    .update(|v| {
                                                                                                        v.success = false;
                                                                                                        v
                                                                                                            .message = String::from(
                                                                                                            "Your session has ended. Please relog your account.",
                                                                                                        );
                                                                                                    });
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                });
                                                                                let hide_per_quote = RwSignal::new(true);
                                                                                let offstrike = format!(
                                                                                    "{:.2}%",
                                                                                    offstrike_percentage * 100.0,
                                                                                );
                                                                                view! {
                                                                                    // For QuotesOptionsForModification

                                                                                    // Default value if parsing fails
                                                                                    // Default value if parsing fails
                                                                                    // Default value if parsing fails
                                                                                    // Default value if parsing fails
                                                                                    // Default value if parsing fails
                                                                                    // Default value if parsing fails
                                                                                    // Default value if parsing fails
                                                                                    // Default value if parsing fails
                                                                                    // Default value if parsing fails
                                                                                    // Default value if parsing fails
                                                                                    // Or any default value you want to use

                                                                                    // Or any default value you want to use

                                                                                    // Or any default value you want to use
                                                                                    // Hide traders quote
                                                                                    // let modified_date = tq.modified_date.clone();

                                                                                    // let stat = if status.clone() == "approved" { "approval".to_string() } else { "rejection".to_string() };
                                                                                    // let request = ApproveTradeQuoteRequest::new(group_ids.clone(), status.clone());
                                                                                    // approve_trade_quote(request).await
                                                                                    // active_quotes_resource.refetch();

                                                                                    <tr class="px924:hidden">
                                                                                        <td prop:hidden=hide_non_jabra colspan="17">

                                                                                            {view! {
                                                                                                <button
                                                                                                    class="flex justify-start w-full gap-2 border-l-2 rounded-none btn btn-ghost btn-md bg-base-100 border-l-success"
                                                                                                    on:click=move |_| hide_per_quote.update(|c| *c = !*c)
                                                                                                >
                                                                                                    <div class="text-xs">{instrument_name.clone()}</div>
                                                                                                    <div class="flex justify-end flex-1">
                                                                                                        <Show
                                                                                                            when=move || { hide_per_quote.get() || hide_non_jabra }
                                                                                                            fallback=move || view! { <ArrowDown/> }
                                                                                                        >
                                                                                                            <ArrowUp/>
                                                                                                        </Show>
                                                                                                    </div>
                                                                                                </button>
                                                                                            }
                                                                                                .into_view()}

                                                                                        </td>
                                                                                    </tr>

                                                                                    // MOBILE VIEW
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "DATE CREATED: "
                                                                                            </span>
                                                                                            {convert_utc_to_local(date_created.as_str())}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td
                                                                                            colspan="17"
                                                                                            class=format!("px924:hidden {}", is_pos_class_a)
                                                                                        >
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "AMOUNT: "
                                                                                            </span>
                                                                                            {is_pos_value_a.clone()}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td
                                                                                            colspan="17"
                                                                                            class=format!("px924:hidden {}", is_pos_class_s)
                                                                                        >
                                                                                            <span class="text-sm text-success px924:hidden">"SIDE: "</span>
                                                                                            {side.clone()}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <span class="text-sm text-success px924:hidden">"SPOT: "</span>
                                                                                            {spot.clone()}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "STRIKE: "
                                                                                            </span>
                                                                                            {strike}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "SPOT(%): "
                                                                                            </span>
                                                                                            {offstrike.clone()}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <span class="text-sm text-success px924:hidden">"IV: "</span>
                                                                                            {iv}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td
                                                                                            colspan="17"
                                                                                            class=format!("px924:hidden {}", is_pos_class_b)
                                                                                        >
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "BASE COST: "
                                                                                            </span>
                                                                                            {is_pos_value_b.clone()}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td
                                                                                            colspan="17"
                                                                                            class=format!("px924:hidden {}", is_pos_class_q)
                                                                                        >
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "QUOTE COST: "
                                                                                            </span>
                                                                                            {is_pos_value_q.clone()}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "EXPIRES IN: "
                                                                                            </span>
                                                                                            {calculate_time_difference(
                                                                                                None,
                                                                                                quote_expiry.clone(),
                                                                                                *gtc,
                                                                                            )}

                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "DELTA: "
                                                                                            </span>
                                                                                            {delta}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "GAMMA: "
                                                                                            </span>
                                                                                            {gamma}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "THETA: "
                                                                                            </span>
                                                                                            {theta}
                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <span class="text-sm text-success px924:hidden">
                                                                                                "SELECT: "
                                                                                            </span>
                                                                                            <input
                                                                                                type="checkbox"
                                                                                                class="checkbox-xs checkbox-success"
                                                                                                prop:value=group_id.clone()
                                                                                                on:change=move |event| {
                                                                                                    let checked = event_target_checked(&event);
                                                                                                    let val = event_target_value(&event);
                                                                                                    if checked {
                                                                                                        set_group_ids.update(|v| v.push(val));
                                                                                                    } else {
                                                                                                        set_group_ids.update(|v| v.retain(|x| x.ne(&val)));
                                                                                                    }
                                                                                                }
                                                                                            />

                                                                                        </td>
                                                                                    </tr>
                                                                                    <tr prop:hidden=move || {
                                                                                        hide_per_quote.get() || hide_non_jabra
                                                                                    }>
                                                                                        <td colspan="17" class="px924:hidden">
                                                                                            <button
                                                                                                class="mr-2 btn btn-xs btn-warning"
                                                                                                on:click=move |_| show_edit_modal.set(true)
                                                                                            >
                                                                                                EDIT
                                                                                            </button>
                                                                                        </td>
                                                                                    </tr>

                                                                                    // DESKTOP VIEW

                                                                                    <div class="hidden px924:contents">
                                                                                        <tr prop:hidden=hide_non_jabra>
                                                                                            <td>{convert_utc_to_local(date_created.as_str())}</td>
                                                                                            <td>{instrument_name.clone()}</td>
                                                                                            <td class=is_pos_class_a>{is_pos_value_a}</td>
                                                                                            <td class=is_pos_class_s>{side.clone()}</td>
                                                                                            <td>{spot.clone()}</td>
                                                                                            <td>{strike}</td>
                                                                                            <td>{offstrike.clone()}</td>
                                                                                            <td>{iv}</td>
                                                                                            <td class=is_pos_class_b>{is_pos_value_b}</td>
                                                                                            <td class=is_pos_class_q>{is_pos_value_q}</td>
                                                                                            <td>
                                                                                                {calculate_time_difference(
                                                                                                    None,
                                                                                                    quote_expiry.clone(),
                                                                                                    *gtc,
                                                                                                )}

                                                                                            </td>
                                                                                            <td>{delta}</td>
                                                                                            <td>{gamma}</td>
                                                                                            <td>{theta}</td>
                                                                                            <td>
                                                                                                <input
                                                                                                    type="checkbox"
                                                                                                    class="checkbox-xs checkbox-success"
                                                                                                    prop:value=group_id.clone()
                                                                                                    on:change=move |event| {
                                                                                                        let checked = event_target_checked(&event);
                                                                                                        let val = event_target_value(&event);
                                                                                                        if checked {
                                                                                                            set_group_ids.update(|v| v.push(val));
                                                                                                        } else {
                                                                                                            set_group_ids.update(|v| v.retain(|x| x.ne(&val)));
                                                                                                        }
                                                                                                    }
                                                                                                />

                                                                                            </td>
                                                                                            <td>
                                                                                                <button
                                                                                                    class="mr-2 btn btn-xs btn-warning"
                                                                                                    on:click=move |_| show_edit_modal.set(true)
                                                                                                >
                                                                                                    EDIT
                                                                                                </button>
                                                                                            </td>
                                                                                        </tr>
                                                                                    </div>

                                                                                    {move || {
                                                                                        view! {
                                                                                            <EditQuotesOptionModal
                                                                                                show=show_edit_modal
                                                                                                quote_option=mod_quote_option
                                                                                                counterparty_id=mod_counterparty
                                                                                                counterparties=counterparties
                                                                                                quotes_option_list=trade_quotes_list
                                                                                                action=modify_quote_action
                                                                                            />
                                                                                        }
                                                                                    }}

                                                                                    {move || match show_modify_alert_modal.get() {
                                                                                        true => {
                                                                                            if !modify_quote_response.get().success {
                                                                                                view! {
                                                                                                    <ErrorModal
                                                                                                        read_signal=show_modify_error_modal
                                                                                                        write_signal=set_show_modify_error_modal
                                                                                                        message=modify_quote_response.get().message
                                                                                                    />
                                                                                                }
                                                                                            } else {
                                                                                                view! {
                                                                                                    <SuccessModalWithRefetch
                                                                                                        read_signal=show_modify_success_modal
                                                                                                        write_signal=set_show_modify_success_modal
                                                                                                        message=modify_quote_response.get().message
                                                                                                        resource=active_quotes_resource
                                                                                                    />
                                                                                                }
                                                                                            }
                                                                                                .into_view()
                                                                                        }
                                                                                        false => view! { <div></div> }.into_view(),
                                                                                    }}
                                                                                }
                                                                            })
                                                                            .collect_view()
                                                                    }}

                                                                    <tr style="background-color:transparent !important">
                                                                        <td colspan="15">
                                                                            <div class="flex justify-between text-xs font-semibold tracking-tight text-gray-600 bg-transparent f whitespace-nowrap dark:text-gray-400">
                                                                                <div>
                                                                                    <Transition fallback=move || {
                                                                                        view! {
                                                                                            <Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS />
                                                                                        }
                                                                                    }>

                                                                                        {move || {
                                                                                            download_uri_resource
                                                                                                .and_then(|url| {
                                                                                                    let uri = url.clone();
                                                                                                    view! {
                                                                                                        <a
                                                                                                            class="btn btn-ghost btn-xs mr-7"
                                                                                                            href=uri
                                                                                                            download
                                                                                                            target="_blank"
                                                                                                        >
                                                                                                            DOWNLOAD PDF
                                                                                                        </a>
                                                                                                    }
                                                                                                })
                                                                                                .collect_view()
                                                                                        }}

                                                                                    </Transition>
                                                                                </div>
                                                                                <div>
                                                                                    <button
                                                                                        class="mr-2 btn btn-xs btn-warning"
                                                                                        prop:disabled=move || group_ids.get().is_empty()
                                                                                        on:click=move |_| confirm_modal_reject.set(true)
                                                                                    >
                                                                                        REJECT
                                                                                    </button>
                                                                                    <button
                                                                                        class="btn btn-xs btn-success"
                                                                                        prop:disabled=move || group_ids.get().is_empty()
                                                                                        on:click=move |_| confirm_modal_approve.set(true)
                                                                                    >
                                                                                        APPROVE
                                                                                    </button>
                                                                                </div>

                                                                            </div>
                                                                        </td>
                                                                    </tr>
                                                                </tbody>

                                                                {move || {
                                                                    view! {
                                                                        <ConfirmModalBatchQuotes
                                                                            signal=confirm_modal_approve
                                                                            function=dispatch_accept_trade_quote
                                                                            params=("approved".to_string(), group_ids.get())
                                                                            pending_signal=is_pending
                                                                            title="APPROVE".to_string()
                                                                        />
                                                                    }
                                                                }}

                                                                {move || {
                                                                    view! {
                                                                        <ConfirmModalBatchQuotes
                                                                            signal=confirm_modal_reject
                                                                            function=dispatch_accept_trade_quote
                                                                            params=("rejected".to_string(), group_ids.get())
                                                                            pending_signal=is_pending
                                                                            title="REJECT".to_string()
                                                                        />
                                                                    }
                                                                }}

                                                                {
                                                                    view! {
                                                                        <ConfirmModalAllQuotes
                                                                            signal=confirm_modal_reject_all
                                                                            function=approve_reject_all
                                                                            params="rejected".to_string()
                                                                            pending_signal=is_pending
                                                                            title="REJECT ALL".to_string()
                                                                        />
                                                                    }
                                                                }

                                                                {
                                                                    view! {
                                                                        <ConfirmModalAllQuotes
                                                                            signal=confirm_modal_approve_all
                                                                            function=approve_reject_all
                                                                            params="approved".to_string()
                                                                            pending_signal=is_pending
                                                                            title="APPROVE ALL".to_string()
                                                                        />
                                                                    }
                                                                }
                                                            }.into_view()
                                                        }
                                                    }
                                                }
                                            }
                                        })
                                        .collect_view()
                                }).into_view()
                        }}

                    </Transition>
                </table>
            </div>

            {move || match show_approve_quote_alert.get() {
                true => {
                    if !approve_quote_response.get().success {
                        set_show_error_modal.set(true);
                        view! {
                            <ErrorModal
                                read_signal=show_error_modal
                                write_signal=set_show_error_modal
                                message=approve_quote_response.get().message
                            />
                        }
                    } else {
                        set_show_success_modal.set(true);
                        view! {
                            <SuccessModalRefetch
                                read_signal=show_success_modal
                                message=approve_quote_response.get().message
                                function=refetch_resource
                            />
                        }
                    }
                        .into_view()
                }
                false => view! { <div></div> }.into_view(),
            }}

        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn ApprovedQuotesTable(
    has_approved: RwSignal<bool>,
    sort_asc_approved: RwSignal<bool>,
    selected_header_approved: RwSignal<String>,
    approved_quotes_resource: Resource<(), Result<BTreeMap<String, Vec<QuoteOption>>, ServerFnError>>
) -> impl IntoView {
    view! {
        <div class="mb-3 ml-2 text-xl font-semibold text-white">
            <span>Recent Accepted Quotes (24 HRS)</span>
        </div>
        <div class="py-3 mb-3">
            <div class="p-3 mb-3 overflow-auto rounded-md shadow-sm border-opacity-30 shadow-success">
                <table class="table overflow-x-visible table-zebra table-xs">
                    <Show when=move || has_approved.get()>
                        <thead class="text-md bg-success bg-opacity-30 text-white">
                            // <tr>
                            // <th colspan = "7" >{counterparty_name}</th>
                            // </tr>
                            <tr class="hidden px924:contents">
                                // <th>DATETIME</th>
                                // <th>INSTRUMENT</th>
                                // <th>AMOUNT</th>
                                // <th>SIDE</th>
                                // <th>STATUS</th>
                                // <th>BASE QUOTE</th>
                                // <th>QUOTE COST</th>

                                {
                                    let headers = RwSignal::new(
                                        vec![
                                            String::from("DATETIME"),
                                            String::from("INSTRUMENT"),
                                            String::from("AMOUNT"),
                                            String::from("SIDE"),
                                            String::from("STATUS"),
                                            String::from("BASE QUOTE"),
                                            String::from("QUOTE COST"),
                                        ],
                                    );
                                    let header_key_params = RwSignal::new(
                                        vec![
                                            String::from("modified_date"),
                                            String::from("instrument_name"),
                                            String::from("amount"),
                                            String::from("side"),
                                            String::from("quote_status"),
                                            String::from("px_in_base_ccy"),
                                            String::from("px_in_quote_ccy"),
                                        ],
                                    );
                                    headers
                                        .get()
                                        .into_iter()
                                        .enumerate()
                                        .map(|(index, key_name)| {
                                            view! {
                                                // header_keys.into_iter().map(|k| {
                                                // view! {
                                                // <GenericDataTableHeader header_selector = selected_header_rejected sorter = k.clone() sort_direction = sort_asc_rejected name = k.clone() />
                                                // }
                                                // }).collect_view()
                                                // let selected_header_param = keys.get()[index];
                                                <th
                                                    class="cursor-pointer"
                                                    on:click=move |_| {
                                                        selected_header_approved
                                                            .set(header_key_params.get()[index].clone());
                                                        sort_asc_approved.update(|s| *s = !*s)
                                                    }
                                                >

                                                    <div class="flex justify-between">
                                                        <span class="flex-0">{key_name}</span>
                                                        <span class="flex-0">
                                                            <svg
                                                                xmlns="http://www.w3.org/2000/svg"
                                                                viewBox="0 0 20 20"
                                                                fill="currentColor"
                                                                class="w-5 h-5"
                                                            >
                                                                <path
                                                                    fill-rule="evenodd"
                                                                    d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z"
                                                                    clip-rule="evenodd"
                                                                ></path>
                                                            </svg>
                                                        </span>
                                                    </div>
                                                </th>
                                            }
                                        })
                                        .collect_view()
                                }

                            </tr>
                        </thead>
                    </Show>
                    <Transition fallback=move || {
                        view! {
                            <div class="flex justify-center ">
                                <Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS />
                            </div>
                        }
                    }>

                        {move || {
                            approved_quotes_resource
                                .and_then(|e| {
                                    if e.is_empty() {
                                        has_approved.set(false);
                                        return view! {
                                            <div class="flex justify-center border-b border-b-gray-700">
                                                <p>No available recent accepted quotes</p>
                                            </div>
                                        }
                                            .into_view();
                                    }
                                    e.into_iter()
                                        .map(|(counterparty, trade_quotes)| {
                                            has_approved.set(true);
                                            let counterparty_name = counterparty
                                                .split("~")
                                                .collect::<Vec<&str>>()[0]
                                                .to_string();
                                            let counterparty_id = counterparty
                                                .split("~")
                                                .collect::<Vec<&str>>()[1]
                                                .to_string();
                                            let (
                                                accepted_trade_quotes_list,
                                                _set_accepted_trade_quotes_list,
                                            ) = create_signal(trade_quotes.clone());
                                            let json_value_approved = Signal::derive(move || {
                                                accepted_trade_quotes_list
                                                    .get()
                                                    .into_iter()
                                                    .map(serde_json::to_value)
                                                    .collect::<Result<Vec<serde_json::Value>, _>>()
                                                    .expect("Failed to serialize to JSON")
                                            });
                                            let header_keys_number_approved = vec![
                                                String::from("amount"),
                                                String::from("px_in_base_ccy"),
                                                String::from("px_in_quote_ccy"),
                                            ];
                                            let tr_approved = Signal::derive(move || {
                                                sort_table(
                                                    json_value_approved.get(),
                                                    sort_asc_approved.get(),
                                                    selected_header_approved.get(),
                                                    header_keys_number_approved.clone(),
                                                )
                                            });
                                            let download_uri_resource = create_resource(
                                                || (),
                                                move |_| {
                                                    let c = counterparty_id.clone();
                                                    async move {
                                                        get_download_url(c, "approved".to_string()).await
                                                    }
                                                },
                                            );
                                            view! {
                                                // let (hidden_approved, set_hidden_approved) = create_signal(false);

                                                // <div class = "p-3 mb-3 border border-gray-700 rounded-md shadow bg-base-100">
                                                // <table class = "table table-zebra-zebra table-sm">
                                                // <thead class = "text-base text-success bg-base-200">
                                                // <div class = "p-3 mb-3 overflow-auto border border-gray-700 rounded-md border-opacity-30 bg-base-100">
                                                // <table class = "table overflow-x-visible table-zebra table-xs">
                                                // <thead class = "text-base bg-base-200 text-success">
                                                // <tr>
                                                // <th colspan = "7" >{counterparty_name}</th>
                                                // </tr>
                                                // <tr class = "hidden px924:contents">
                                                // <th>DATETIME</th>
                                                // <th>INSTRUMENT</th>
                                                // <th>AMOUNT</th>
                                                // <th>SIDE</th>
                                                // <th>STATUS</th>
                                                // <th>BASE QUOTE</th>
                                                // <th>QUOTE COST</th>
                                                // </tr>
                                                // </thead>
                                                <tbody>
                                                    <tr>
                                                        <td class="bg-base-200 text-success font-semibold" colspan="7">
                                                            {counterparty_name}
                                                        </td>
                                                    </tr>

                                                    {move || {
                                                        tr_approved
                                                            .get()
                                                            .into_iter()
                                                            .map(|tq| {
                                                                let counter_party = match &tq["counterparty_id"]["name"] {
                                                                    Value::String(s) => s,
                                                                    _ => &String::from(""),
                                                                };
                                                                let modified_date = match &tq["modified_date"] {
                                                                    Value::String(s) => s,
                                                                    _ => &String::from(""),
                                                                };
                                                                let amount = match &tq["amount"] {
                                                                    Value::Number(s) => {
                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                    }
                                                                    _ => 0.0,
                                                                };
                                                                let instrument_name = match &tq["instrument_name"] {
                                                                    Value::String(s) => s,
                                                                    _ => &String::from(""),
                                                                };
                                                                let side = match &tq["side"] {
                                                                    Value::String(s) => s,
                                                                    _ => &String::from(""),
                                                                };
                                                                let quote_status = match &tq["quote_status"] {
                                                                    Value::String(s) => s,
                                                                    _ => &String::from(""),
                                                                };
                                                                let px_in_base_ccy = match &tq["px_in_base_ccy"] {
                                                                    Value::Number(s) => {
                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                    }
                                                                    _ => 0.0,
                                                                };
                                                                let px_in_quote_ccy = match &tq["px_in_quote_ccy"] {
                                                                    Value::Number(s) => {
                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                    }
                                                                    _ => 0.0,
                                                                };
                                                                let hide_non_jabra = counter_party != "JABRA";
                                                                let is_pos_class_s = if side == "Sell" {
                                                                    "text-error"
                                                                } else {
                                                                    "text-success"
                                                                };
                                                                let hide_per_approved_quote = RwSignal::new(true);
                                                                view! {
                                                                    // Hide traders quote
                                                                    // Default value if parsing fails
                                                                    // Default value if parsing fails
                                                                    // Default value if parsing fails
                                                                    // Hide traders quote
                                                                    // let modified_date = tq.modified_date.clone();

                                                                    <tr class="px924:hidden">
                                                                        <td prop:hidden=hide_non_jabra colspan="16">

                                                                            {view! {
                                                                                <button
                                                                                    class="flex justify-start w-full gap-2 border-l-2 rounded-none btn btn-ghost btn-md bg-base-100 border-l-success"
                                                                                    on:click=move |_| {
                                                                                        hide_per_approved_quote.update(|c| *c = !*c)
                                                                                    }
                                                                                >

                                                                                    <div class="text-xs">{instrument_name}</div>
                                                                                    <div class="flex justify-end flex-1">
                                                                                        <Show
                                                                                            when=move || {
                                                                                                hide_per_approved_quote.get() || hide_non_jabra
                                                                                            }

                                                                                            fallback=move || view! { <ArrowDown/> }
                                                                                        >
                                                                                            <ArrowUp/>
                                                                                        </Show>
                                                                                    </div>
                                                                                </button>
                                                                            }
                                                                                .into_view()}

                                                                        </td>
                                                                    </tr>

                                                                    // MOBILE VIEW
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_approved_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td colspan="16" class="px924:hidden">
                                                                            <span class="text-sm text-success px924:hidden">
                                                                                "DATETIME: "
                                                                            </span>
                                                                            {convert_utc_to_local(modified_date.as_str())}
                                                                        </td>
                                                                    </tr>
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_approved_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td colspan="16" class="px924:hidden">
                                                                            <span class="text-sm text-success px924:hidden">
                                                                                "AMOUNT: "
                                                                            </span>
                                                                            {amount}
                                                                        </td>
                                                                    </tr>
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_approved_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td
                                                                            colspan="16"
                                                                            class=format!("px924:hidden {}", is_pos_class_s)
                                                                        >
                                                                            <span class="text-sm text-success px924:hidden">"SIDE: "</span>
                                                                            {side.clone()}
                                                                        </td>
                                                                    </tr>
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_approved_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td colspan="16" class="px924:hidden">
                                                                            <span class="text-sm text-success px924:hidden">
                                                                                "QUOTE STATUS: "
                                                                            </span>
                                                                            {quote_status.clone()}
                                                                        </td>
                                                                    </tr>
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_approved_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td colspan="16" class="px924:hidden">
                                                                            <span class="text-sm text-success px924:hidden">
                                                                                "BASE COST: "
                                                                            </span>
                                                                            {px_in_base_ccy}
                                                                        </td>
                                                                    </tr>
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_approved_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td colspan="16" class="px924:hidden">
                                                                            <span class="text-sm text-success px924:hidden">
                                                                                "QUOTE COST: "
                                                                            </span>
                                                                            {px_in_quote_ccy}
                                                                        </td>
                                                                    </tr>

                                                                    // DESKTOP VIEW
                                                                    <div class="hidden px924:contents">
                                                                        <tr class="hover" prop:hidden=hide_non_jabra>
                                                                            <td>{convert_utc_to_local(modified_date.as_str())}</td>
                                                                            <td>{instrument_name}</td>
                                                                            <td>{amount}</td>
                                                                            <td class=is_pos_class_s>{side}</td>
                                                                            <td>{quote_status}</td>
                                                                            <td>{px_in_base_ccy}</td>
                                                                            <td>{px_in_quote_ccy}</td>
                                                                        </tr>
                                                                    </div>
                                                                }
                                                            })
                                                            .collect_view()
                                                    }}

                                                    <tr style="background-color:transparent !important">
                                                        <td colspan="7">
                                                            <div class="flex justify-start text-xs font-semibold tracking-tight text-gray-600 bg-transparent whitespace-nowrap dark:text-gray-400">

                                                                <Transition fallback=move || {
                                                                    view! {
                                                                        <Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS />
                                                                    }
                                                                }>

                                                                    {move || {
                                                                        download_uri_resource
                                                                            .and_then(|url| {
                                                                                let uri = url.clone();
                                                                                view! {
                                                                                    <a
                                                                                        class="btn btn-ghost btn-xs mr-7"
                                                                                        href=uri
                                                                                        download
                                                                                        target="_blank"
                                                                                    >
                                                                                        DOWNLOAD PDF
                                                                                    </a>
                                                                                }
                                                                            })
                                                                            .collect_view()
                                                                    }}

                                                                </Transition>

                                                            </div>
                                                        </td>
                                                    </tr>
                                                </tbody>
                                            }
                                        })
                                        .collect_view()
                                })
                        }}

                    </Transition>
                </table>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn RejectedQuotesTable(
    has_rejected: RwSignal<bool>,
    sort_asc_rejected: RwSignal<bool>,
    selected_header_rejected: RwSignal<String>,
    rejected_quotes_resource: Resource<(), Result<BTreeMap<String, Vec<QuoteOption>>, ServerFnError>>
) -> impl IntoView {
    view! {
        <div class="mb-3 ml-2 text-xl font-semibold text-white">
            <span>Recent Rejected Quotes (24 HRS)</span>
        </div>
        <div class="py-3 mb-3">
            <div class="p-3 mb-3 overflow-auto rounded-md border-opacity-30 shadow-sm shadow-success">
                <table
                    class="table overflow-x-visible table-zebra table-xs"
                >
                    <Show when=move || has_rejected.get()>
                        <thead class="text-md bg-success bg-opacity-30 text-white">
                            // <tr>
                            // <th colspan = "7" >{counterparty_name}</th>
                            // </tr>
                            <tr class="hidden px924:contents">
                                // <th>DATETIME</th>
                                // <th>INSTRUMENT</th>
                                // <th>AMOUNT</th>
                                // <th>SIDE</th>
                                // <th>STATUS</th>
                                // <th>BASE QUOTE</th>
                                // <th>QUOTE COST</th>

                                {
                                    let headers = RwSignal::new(
                                        vec![
                                            String::from("DATETIME"),
                                            String::from("INSTRUMENT"),
                                            String::from("AMOUNT"),
                                            String::from("SIDE"),
                                            String::from("STATUS"),
                                            String::from("BASE QUOTE"),
                                            String::from("QUOTE COST"),
                                        ],
                                    );
                                    let header_key_params = RwSignal::new(
                                        vec![
                                            String::from("modified_date"),
                                            String::from("instrument_name"),
                                            String::from("amount"),
                                            String::from("side"),
                                            String::from("quote_status"),
                                            String::from("px_in_base_ccy"),
                                            String::from("px_in_quote_ccy"),
                                        ],
                                    );
                                    headers
                                        .get()
                                        .into_iter()
                                        .enumerate()
                                        .map(|(index, key_name)| {
                                            view! {
                                                // header_keys.into_iter().map(|k| {
                                                // view! {
                                                // <GenericDataTableHeader header_selector = selected_header_rejected sorter = k.clone() sort_direction = sort_asc_rejected name = k.clone() />
                                                // }
                                                // }).collect_view()
                                                // let selected_header_param = keys.get()[index];
                                                <th
                                                    class="cursor-pointer"
                                                    on:click=move |_| {
                                                        selected_header_rejected
                                                            .set(header_key_params.get()[index].clone());
                                                        sort_asc_rejected.update(|s| *s = !*s)
                                                    }
                                                >

                                                    <div class="flex justify-between">
                                                        <span class="flex-0">{key_name}</span>
                                                        <span class="flex-0">
                                                            <svg
                                                                xmlns="http://www.w3.org/2000/svg"
                                                                viewBox="0 0 20 20"
                                                                fill="currentColor"
                                                                class="w-5 h-5"
                                                            >
                                                                <path
                                                                    fill-rule="evenodd"
                                                                    d="M10 3a.75.75 0 01.55.24l3.25 3.5a.75.75 0 11-1.1 1.02L10 4.852 7.3 7.76a.75.75 0 01-1.1-1.02l3.25-3.5A.75.75 0 0110 3zm-3.76 9.2a.75.75 0 011.06.04l2.7 2.908 2.7-2.908a.75.75 0 111.1 1.02l-3.25 3.5a.75.75 0 01-1.1 0l-3.25-3.5a.75.75 0 01.04-1.06z"
                                                                    clip-rule="evenodd"
                                                                ></path>
                                                            </svg>
                                                        </span>
                                                    </div>
                                                </th>
                                            }
                                        })
                                        .collect_view()
                                }

                            </tr>
                        </thead>
                    </Show>
                    <Transition fallback=move || {
                        view! {
                            <div class="flex justify-center ">
                                <Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS />
                            </div>
                        }
                    }>

                        {move || {
                            rejected_quotes_resource
                                .and_then(|e| {
                                    if e.is_empty() {
                                        has_rejected.set(false);
                                        return view! {
                                            <div class="flex justify-center border-b border-b-gray-700">
                                                <p>No available recent rejected quotes</p>
                                            </div>
                                        }
                                            .into_view();
                                    }
                                    e.into_iter()
                                        .map(|(counterparty, trade_quotes)| {
                                            has_rejected.set(true);
                                            let (
                                                rejected_trade_quotes_list,
                                                _set_rejected_trade_quotes_list,
                                            ) = create_signal(trade_quotes.clone());
                                            let json_value_rejected = Signal::derive(move || {
                                                rejected_trade_quotes_list
                                                    .get()
                                                    .into_iter()
                                                    .map(serde_json::to_value)
                                                    .collect::<Result<Vec<serde_json::Value>, _>>()
                                                    .expect("Failed to serialize to JSON")
                                            });
                                            let header_keys_number_rejected = vec![
                                                String::from("amount"),
                                                String::from("px_in_base_ccy"),
                                                String::from("px_in_quote_ccy"),
                                            ];
                                            let tr_rejected = Signal::derive(move || {
                                                sort_table(
                                                    json_value_rejected.get(),
                                                    sort_asc_rejected.get(),
                                                    selected_header_rejected.get(),
                                                    header_keys_number_rejected.clone(),
                                                )
                                            });
                                            let counterparty_name = counterparty
                                                .split("~")
                                                .collect::<Vec<&str>>()[0]
                                                .to_string();
                                            let counterparty_id = counterparty
                                                .split("~")
                                                .collect::<Vec<&str>>()[1]
                                                .to_string();
                                            let download_uri_resource = create_resource(
                                                || (),
                                                move |_| {
                                                    let c = counterparty_id.clone();
                                                    async move {
                                                        get_download_url(c, "rejected".to_string()).await
                                                    }
                                                },
                                            );
                                            view! {
                                                // rejected_trade_quotes_list.set(trade_quotes.clone());
                                                // let rejected_trade_quotes_list = RwSignal::new(Vec::<QuoteOption>::default());

                                                // let (hidden_rejected, set_hidden_rejected) = create_signal(false);

                                                // <div class = "p-3 mb-3 border border-gray-700 rounded-md shadow bg-base-100">
                                                // <table class = "table table-zebra-zebra table-sm">
                                                // <thead class = "text-base text-success bg-base-200" >

                                                <tbody>
                                                    <tr>
                                                        <td class="bg-base-200 text-success font-semibold" colspan="7">
                                                            {counterparty_name}
                                                        </td>
                                                    </tr>

                                                    {move || {
                                                        tr_rejected()
                                                            .into_iter()
                                                            .map(|tq| {
                                                                let counter_party = match &tq["counterparty_id"]["name"] {
                                                                    Value::String(s) => s,
                                                                    _ => &String::from(""),
                                                                };
                                                                let modified_date = match &tq["modified_date"] {
                                                                    Value::String(s) => s,
                                                                    _ => &String::from(""),
                                                                };
                                                                let amount = match &tq["amount"] {
                                                                    Value::Number(s) => {
                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                    }
                                                                    _ => 0.0,
                                                                };
                                                                let instrument_name = match &tq["instrument_name"] {
                                                                    Value::String(s) => s,
                                                                    _ => &String::from(""),
                                                                };
                                                                let side = match &tq["side"] {
                                                                    Value::String(s) => s,
                                                                    _ => &String::from(""),
                                                                };
                                                                let quote_status = match &tq["quote_status"] {
                                                                    Value::String(s) => s,
                                                                    _ => &String::from(""),
                                                                };
                                                                let px_in_base_ccy = match &tq["px_in_base_ccy"] {
                                                                    Value::Number(s) => {
                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                    }
                                                                    _ => 0.0,
                                                                };
                                                                let px_in_quote_ccy = match &tq["px_in_quote_ccy"] {
                                                                    Value::Number(s) => {
                                                                        if let Some(parsed) = s.as_f64() { parsed } else { 0.0 }
                                                                    }
                                                                    _ => 0.0,
                                                                };
                                                                let hide_non_jabra = counter_party != "JABRA";
                                                                let is_pos_class_s = if side == "Sell" {
                                                                    "text-error"
                                                                } else {
                                                                    "text-success"
                                                                };
                                                                let hide_per_rejected_quote = RwSignal::new(true);
                                                                view! {
                                                                    // Default value if parsing fails
                                                                    // Default value if parsing fails
                                                                    // Default value if parsing fails
                                                                    // Hide traders quote
                                                                    // let modified_date = tq.modified_date.clone();

                                                                    <tr class="px924:hidden">
                                                                        <td prop:hidden=hide_non_jabra colspan="16">

                                                                            {view! {
                                                                                <button
                                                                                    class="flex justify-start w-full gap-2 border-l-2 rounded-none btn btn-ghost btn-md bg-base-100 border-l-success"
                                                                                    on:click=move |_| {
                                                                                        hide_per_rejected_quote.update(|c| *c = !*c)
                                                                                    }
                                                                                >

                                                                                    <div class="text-xs">{instrument_name}</div>
                                                                                    <div class="flex justify-end flex-1">
                                                                                        <Show
                                                                                            when=move || {
                                                                                                hide_per_rejected_quote.get() || hide_non_jabra
                                                                                            }

                                                                                            fallback=move || view! { <ArrowDown/> }
                                                                                        >
                                                                                            <ArrowUp/>
                                                                                        </Show>
                                                                                    </div>
                                                                                </button>
                                                                            }
                                                                                .into_view()}

                                                                        </td>
                                                                    </tr>

                                                                    // MOBILE VIEW
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_rejected_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td colspan="16" class="px924:hidden">
                                                                            <span class="text-sm text-success px924:hidden">
                                                                                "DATETIME: "
                                                                            </span>
                                                                            {convert_utc_to_local(modified_date.as_str())}
                                                                        </td>
                                                                    </tr>
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_rejected_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td colspan="16" class="px924:hidden">
                                                                            <span class="text-sm text-success px924:hidden">
                                                                                "AMOUNT: "
                                                                            </span>
                                                                            {amount}
                                                                        </td>
                                                                    </tr>
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_rejected_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td
                                                                            colspan="16"
                                                                            class=format!("px924:hidden {}", is_pos_class_s)
                                                                        >
                                                                            <span class="text-sm text-success px924:hidden">"SIDE: "</span>
                                                                            {side}
                                                                        </td>
                                                                    </tr>
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_rejected_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td colspan="16" class="px924:hidden">
                                                                            <span class="text-sm text-success px924:hidden">
                                                                                "QUOTE STATUS: "
                                                                            </span>
                                                                            {quote_status}
                                                                        </td>
                                                                    </tr>
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_rejected_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td colspan="16" class="px924:hidden">
                                                                            <span class="text-sm text-success px924:hidden">
                                                                                "BASE COST: "
                                                                            </span>
                                                                            {px_in_base_ccy}
                                                                        </td>
                                                                    </tr>
                                                                    <tr prop:hidden=move || {
                                                                        hide_per_rejected_quote.get() || hide_non_jabra
                                                                    }>
                                                                        <td colspan="16" class="px924:hidden">
                                                                            <span class="text-sm text-success px924:hidden">
                                                                                "QUOTE COST: "
                                                                            </span>
                                                                            {px_in_quote_ccy}
                                                                        </td>
                                                                    </tr>

                                                                    // DESKTOP VIEW

                                                                    <div class="hidden px924:contents">
                                                                        <tr class="hover" prop:hidden=hide_non_jabra>
                                                                            <td>{convert_utc_to_local(modified_date.as_str())}</td>
                                                                            <td>{instrument_name}</td>
                                                                            <td>{amount}</td>
                                                                            <td class=is_pos_class_s>{side}</td>
                                                                            <td>{quote_status}</td>
                                                                            <td>{px_in_base_ccy}</td>
                                                                            <td>{px_in_quote_ccy}</td>
                                                                        </tr>
                                                                    </div>
                                                                }
                                                            })
                                                            .collect_view()
                                                    }}

                                                    <tr style="background-color:transparent !important">
                                                        <td colspan="7">
                                                            <div class="flex justify-start text-xs font-semibold tracking-tight text-gray-600 bg-transparent whitespace-nowrap dark:text-gray-400">
                                                                <Transition fallback=move || {
                                                                    view! {
                                                                        <Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS />
                                                                    }
                                                                }>

                                                                    {
                                                                        log::info!("Download URI test");
                                                                        move || {
                                                                            download_uri_resource
                                                                                .and_then(|url| {
                                                                                    let uri = url.clone();
                                                                                    view! {
                                                                                        <a
                                                                                            class="btn btn-ghost btn-xs mr-7"
                                                                                            href=uri
                                                                                            download
                                                                                            target="_blank"
                                                                                        >
                                                                                            DOWNLOAD PDF
                                                                                        </a>
                                                                                    }
                                                                                })
                                                                                .collect_view()
                                                                        }
                                                                    }

                                                                </Transition>
                                                            </div>
                                                        </td>
                                                    </tr>
                                                </tbody>
                                            }
                                        })
                                        .collect_view()
                                })
                        }}

                    </Transition>
                </table>

            </div>
        </div>
    }
}

async fn get_download_url(id: String, status: String) -> Result<String, ServerFnError> {
    let url = std::env::var("JABRAAPIGATEWAYPUB").unwrap_or_default();
    let path = format!("{}/rfq/generate_pdf/{}/{}", url, id, status);
    Ok(path)
}

// /// Component for the Edit Quotes Option Modal.

#[allow(non_snake_case)]
#[component]
pub fn EditQuotesOptionModal(
    /// The RwSignal of type boolean to show the modal.
    show: RwSignal<bool>,
    /// The RwSignal of type `QuoteOption` containing the quote option to be edited.
    quote_option: RwSignal<QuoteOption>,
    /// The RwSignal of type `u16` containing the counterparty id.
    counterparty_id: RwSignal<u16>,
    /// The RwSignal of type `Vec<CounterParty>` containing the list of counterparties.
    /// Used to get the non jabra counterparties.
    counterparties: RwSignal<Vec<CounterParty>>,
    /// The ReadSignal of type `Vec<QuoteOption>` containing the list of quotes options.
    quotes_option_list: ReadSignal<Vec<QuoteOption>>,
    /// The Action that contains a Vector of `QuotesOptionsForModification`.
    action: Action<Vec<QuotesOptionsForModification>, ()>,
) -> impl IntoView {
    // let modal_ref = create_node_ref::<Div>();
    let ccy1 = move || {
        quote_option
            .get()
            .pair_id
            .name
            .split("/")
            .next()
            .map(|s| s.to_string())
            .unwrap()
    };
    let ccy2 = move || {
        quote_option
            .get()
            .pair_id
            .name
            .split("/")
            .nth(1)
            .map(|s| s.to_string())
            .unwrap()
    };
    let non_jabra_counterparties = move || {
        counterparties
            .get()
            .into_iter()
            .filter(|cp| cp.name != "JABRA")
            .collect::<Vec<CounterParty>>()
    };
    // let quote_option_id: RwSignal<u16>  = RwSignal::new(0);
    let quote_option_amount: RwSignal<f64> = RwSignal::new(quote_option.get_untracked().amount);
    let quote_option_counterparty_id: RwSignal<u16> =
        RwSignal::new(counterparty_id.get_untracked());
    let quote_option_px_in_base_ccy: RwSignal<f64> =
        RwSignal::new(quote_option.get_untracked().px_in_base_ccy);
    let quote_option_px_in_quote_ccy: RwSignal<f64> =
        RwSignal::new(quote_option.get_untracked().px_in_quote_ccy);
    let quote_option_quote_expiry: RwSignal<String> = RwSignal::new(convert_utc_to_local(
        quote_option.get_untracked().quote_expiry.as_str(),
    ));
    let quote_option_payout_ccy: RwSignal<Option<String>> =
        RwSignal::new(quote_option.get_untracked().payout_ccy);
    let quote_option_gtc: RwSignal<bool> = RwSignal::new(quote_option.get_untracked().gtc);

    // Checks if an action has a value, then sets the show_modal to false, and resets the action_value to None
    create_effect(move |_| {
        let action_value = action.value();

        if let Some(_action) = action_value.get() {
            show.set(false);
            action_value.set(None);
        }
    });

    let dispatch = move || {
        let quote_options_to_edit = quotes_option_list
            .get()
            .into_iter()
            .filter(|q| q.group_id == quote_option.get().group_id)
            .collect::<Vec<QuoteOption>>();
        let jabra_quote = quote_options_to_edit
            .iter()
            .find(|q| q.counterparty_id.name == "JABRA")
            .unwrap();
        let non_jabra_quote = quote_options_to_edit
            .iter()
            .find(|q| q.counterparty_id.name != "JABRA")
            .unwrap();
        let modified_jabra_quote = QuotesOptionsForModification::new(
            jabra_quote.id.clone(),
            quote_option_amount.get(),
            jabra_quote.counterparty_id.id,
            quote_option_px_in_base_ccy.get(),
            quote_option_px_in_quote_ccy.get(),
            parse_str_to_utc_datetime_str(quote_option_quote_expiry.get().as_str()),
            quote_option_payout_ccy.get(),
            jabra_quote.counterparty_id.id,
            quote_option_counterparty_id.get(),
            quote_option_gtc.get(),
        );

        let modified_non_jabra_quote = QuotesOptionsForModification::new(
            non_jabra_quote.id.clone(),
            -quote_option_amount.get(),
            quote_option_counterparty_id.get(),
            -quote_option_px_in_base_ccy.get(),
            -quote_option_px_in_quote_ccy.get(),
            parse_str_to_utc_datetime_str(quote_option_quote_expiry.get().as_str()),
            quote_option_payout_ccy.get(),
            quote_option_counterparty_id.get(),
            jabra_quote.counterparty_id.id,
            quote_option_gtc.get(),
        );

        let mut request: Vec<QuotesOptionsForModification> =
            Vec::<QuotesOptionsForModification>::default();
        request.push(modified_jabra_quote);
        request.push(modified_non_jabra_quote);
        action.dispatch(request);
    };
    let on_cancel = move || {
        show.set(false);
        quote_option_amount.set(quote_option.get_untracked().amount);
        quote_option_counterparty_id.set(counterparty_id.get_untracked());
        quote_option_px_in_base_ccy.set(quote_option.get_untracked().px_in_base_ccy);
        quote_option_px_in_quote_ccy.set(quote_option.get_untracked().px_in_quote_ccy);
        quote_option_quote_expiry.set(convert_utc_to_local(
            quote_option.get_untracked().quote_expiry.as_str(),
        ));
        quote_option_payout_ccy.set(quote_option.get_untracked().payout_ccy);
        quote_option_gtc.set(quote_option.get_untracked().gtc);
        // show.set(false);
    };
    let is_pending = action.pending();

    view! {
        <Show when=move || show.get()>
            <div class="blur-bg">
                <div class="modal-center-edit">
                    <div class="flex flex-col gap-4 m-2 modal-box">
                        <div class="items-center content-center text-center text-success">
                            <p>{quote_option.get().instrument_name}</p>
                        </div>
                        <div class="grid grid-cols-2 gap-3">
                            <div colspan="1">
                                <label class="block text-sm font-light">Spot</label>
                                <input
                                    type="number"
                                    class="block w-full text-xs border-gray-800 rounded shadow-md input-sm input-disabled bg-base-100"
                                    name="spot"
                                    prop:disabled=true
                                    prop:value=quote_option.get().spot
                                />
                            </div>
                            <div colspan="1">
                                <label class="block text-sm font-light">Side</label>
                                <input
                                    type="text"
                                    class="w-full text-xs border-gray-800 rounded shadow-md input-sm input-disabled bg-base-100"
                                    name="side"
                                    prop:disabled=true
                                    prop:value=quote_option.get().side
                                />
                            </div>
                        </div>
                        <div class="grid grid-cols-2 gap-3">
                            <div colspan="1">
                                <label class="block text-sm font-light">Counter Party</label>
                                <select
                                    type="text"
                                    class="block w-full text-xs border-gray-800 rounded shadow-md input-sm text-success hover:shadow-sm hover:shadow-success bg-base-100"
                                    name="counterparty"
                                    on:change=move |event| {
                                        let val = event_target_value(&event);
                                        quote_option_counterparty_id
                                            .set(val.parse::<u16>().unwrap());
                                    }
                                >

                                    {move || {
                                        non_jabra_counterparties()
                                            .into_iter()
                                            .map(|cp| {
                                                view! {
                                                    <option
                                                        prop:selected=counterparty_id.get() == cp.id.clone()
                                                        value=cp.id.clone().to_string()
                                                    >
                                                        {cp.name}
                                                    </option>
                                                }
                                            })
                                            .collect_view()
                                    }}

                                </select>
                            </div>
                            <div colspan="1">
                                <label class="block text-sm font-light">Amount</label>
                                <input
                                    type="number"
                                    class="block w-full text-xs border-gray-800 rounded shadow-md input-sm text-success hover:shadow-sm hover:shadow-success bg-base-100"
                                    name="amount"
                                    prop:value=quote_option_amount
                                    on:change=move |event| {
                                        let val = event_target_value(&event);
                                        quote_option_amount.set(val.parse::<f64>().unwrap());
                                    }
                                />

                            </div>
                        </div>
                        <div class="grid grid-cols-2 gap-3">
                            <div colspan="1">
                                <label class="block text-sm font-light">Base Currency Cost</label>
                                <input
                                    type="number"
                                    class="block w-full text-xs border-gray-800 rounded shadow-md input-sm text-success hover:shadow-sm hover:shadow-success bg-base-100"
                                    name="base_cost"
                                    prop:value=quote_option_px_in_base_ccy
                                    on:change=move |event| {
                                        let val = event_target_value(&event);
                                        quote_option_px_in_base_ccy
                                            .set(val.parse::<f64>().unwrap());
                                    }
                                />

                            </div>
                            <div colspan="1">
                                <label class="block text-sm font-light">Quote Currency Cost</label>
                                <input
                                    type="number"
                                    class="block w-full text-xs border-gray-800 rounded shadow-md input-sm text-success hover:shadow-sm hover:shadow-success bg-base-100"
                                    name="quote_cost"
                                    prop:value=quote_option_px_in_quote_ccy
                                    on:change=move |event| {
                                        let val = event_target_value(&event);
                                        quote_option_px_in_quote_ccy
                                            .set(val.parse::<f64>().unwrap());
                                    }
                                />

                            </div>
                        </div>
                        <div class="grid grid-cols-2 gap-3">
                            <div colspan="1">
                                <label class="block text-sm font-light">Select Payout</label>
                                <select
                                    class="block w-full text-xs border-gray-800 rounded shadow-md select-sm text-success hover:shadow-sm hover:shadow-success bg-base-100"
                                    name="payout_ccy"
                                    on:change=move |event| {
                                        let val = event_target_value(&event);
                                        quote_option_payout_ccy.set(Some(val));
                                    }
                                >

                                    {match quote_option_payout_ccy.get() {
                                        Some(p) => {
                                            if p == "base" {
                                                view! {
                                                    <option value="base" selected>
                                                        {ccy1()}
                                                    </option>
                                                    <option value="quote">{ccy2()}</option>
                                                }
                                            } else {
                                                view! {
                                                    <option value="quote" selected>
                                                        {ccy2()}
                                                    </option>
                                                    <option value="base">{ccy1()}</option>
                                                }
                                            }
                                        }
                                        None => {
                                            view! {
                                                <option value="" selected>
                                                    Not Assigned
                                                </option>
                                                <option value="base">{ccy1()}</option>
                                                <option value="quote">{ccy2()}</option>
                                            }
                                        }
                                    }}

                                </select>
                            </div>
                            <div colspan="1"></div>
                        </div>
                        <div class="text-center">
                            <label class="block text-sm font-light">Expiry</label>
                        </div>
                        <div class="grid grid-cols-2 gap-3">
                            <div colspan="1">
                                <input
                                    type="datetime-local"
                                    class="block w-full text-xs border-gray-800 rounded shadow-md input-sm text-success hover:shadow-sm hover:shadow-success bg-base-100"
                                    name="expiry"
                                    prop:value=quote_option_quote_expiry
                                    prop:disabled=quote_option_gtc
                                    on:change=move |event| {
                                        let val = event_target_value(&event);
                                        quote_option_quote_expiry.set(val);
                                    }
                                />

                            </div>
                            <div colspan="1" class="m-auto">
                                <input
                                    type="checkbox"
                                    class="text-xs toggle toggle-success toggle-sm"
                                    prop:checked=quote_option_gtc
                                    on:change=move |event| {
                                        let checked = event_target_checked(&event);
                                        quote_option_gtc.set(checked);
                                    }
                                />

                                <label class="ml-4 text-sm font-light">Good Till Cancelled</label>
                            </div>
                        </div>
                        <div class="grid grid-cols-3 gap-3">
                            <div colspan="1"></div>
                            <div colspan="1">

                                {match is_pending.get() {
                                    true => {
                                        view! {
                                            <button class="w-full mt-2 btn btn-sm btn-success">
                                                <span class="loading loading-spinner loading-sm"></span>
                                            </button>
                                        }
                                            .into_any()
                                    }
                                    false => {
                                        view! {
                                            <button
                                                class="w-full mt-2 btn btn-sm btn-success"
                                                on:click=move |_| dispatch()
                                            >
                                                SUBMIT
                                            </button>
                                        }
                                            .into_any()
                                    }
                                }}

                            </div>
                            <div colspan="1">
                                <button
                                    class="w-full mt-2 btn btn-sm btn-error"
                                    on:click=move |_| on_cancel()
                                >
                                    CANCEL
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}