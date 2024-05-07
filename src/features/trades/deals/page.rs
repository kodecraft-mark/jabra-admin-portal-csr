use base64::engine::general_purpose;
use base64::Engine;
use chrono::FixedOffset;
use chrono::Utc;
use leptos::*;
use leptos_router::create_query_signal;
use leptos_router::Form;

use crate::components::confirm_modal::ConfirmModal;
use crate::components::error_modal::ErrorModal;
use crate::components::success_modal_redirect::SuccessModalWithRedirection;
use crate::utilities::date_util::{convert_utc_to_edt,get_expiration_datetime_utc,get_time_in_local_time,update_edt_time,get_datetime_in_local_time,get_date_in_local_time};
use crate::utilities::local::get_conditional_loss;
use crate::utilities::number_util::format_number_en;

use super::services::*;
use super::models::*;

#[allow(non_snake_case)]
#[component]
pub fn Deals() -> impl IntoView {
    let settlement_option_response: RwSignal<SettlementOptionResponse> =
        create_rw_signal(SettlementOptionResponse::default());
    let preview_enabled = RwSignal::new(false);
    let submit_new_term_sheet_response: RwSignal<SubmitNewTermSheetResponse> =
        create_rw_signal(SubmitNewTermSheetResponse::default());
    let submit_new_term_sheet_response_show: RwSignal<bool> = create_rw_signal(false);
    let settlement_options_action: Action<SettlementOptionRequest, ()> =
        create_action(move |settlement_option_request: &SettlementOptionRequest| {
            let req = settlement_option_request.clone();
            async move {
                let result = post_settlement_option(req).await;
                match result {
                    Ok(response) => {
                        settlement_option_response.set(response);
                        preview_enabled.set(true);
                    }
                    Err(_e) => {
                        settlement_option_response.set(SettlementOptionResponse::default());
                    }
                }
            }
        });

    let new_term_sheet_action_with_id: Action<SubmitNewTermSheetRequestWithGroupId, ()> =
        create_action(
            move |settlement_option_request: &SubmitNewTermSheetRequestWithGroupId| {
                let req = settlement_option_request.clone();
                async move {
                    let result = post_submit_new_term_sheet_with_id(req).await;
                    match result {
                        Ok(response) => {
                            submit_new_term_sheet_response.set(response);
                            submit_new_term_sheet_response_show.set(true);
                        }
                        Err(_e) => {
                            submit_new_term_sheet_response.set(SubmitNewTermSheetResponse {
                                status: -1,
                                message: "The transaction is failed".to_string(),
                                refid: "N/A".to_string(),
                            });
                            submit_new_term_sheet_response_show.set(true);
                        }
                    }
                }
            },
        );

    view! {
        <DealsPage
        settlement_options_action= settlement_options_action
        settlement_option_response = settlement_option_response
        preview_enabled= preview_enabled
        new_term_sheet_action_with_id = new_term_sheet_action_with_id
        submit_new_term_sheet_response = submit_new_term_sheet_response
        submit_new_term_sheet_response_show = submit_new_term_sheet_response_show/>
    }
}
#[allow(non_snake_case)]
#[component]
pub fn DealsPage(
    settlement_options_action: Action<SettlementOptionRequest, ()>,
    settlement_option_response: RwSignal<SettlementOptionResponse>,
    preview_enabled: RwSignal<bool>,
    new_term_sheet_action_with_id: Action<SubmitNewTermSheetRequestWithGroupId, ()>,
    submit_new_term_sheet_response: RwSignal<SubmitNewTermSheetResponse>,
    submit_new_term_sheet_response_show: RwSignal<bool>,
) -> impl IntoView {
    const AGENT: &str = "JABRA TRADING LLC";

    let (show_confirm_modal, set_show_confirm_modal) = create_signal(false);
    let (show_success_modal, set_show_success_modal) = create_signal(true);
    let (show_error_modal, set_show_error_modal) = create_signal(true);

    let (query_signal, _set_query_signal) = create_query_signal::<String>("query");
    let (cp_signal, _set_cp_signal) = create_query_signal::<String>("cp_name");
    let (group_id_signal, _set_group_id_signal) = create_query_signal::<String>("group_id");
    let (date_created_query, _set_date_created_query) =
        create_query_signal::<String>("date_created");
    let (expiry_date, set_expiry_date) = create_signal(Utc::now());
    let (expiry_edt, set_expiry_edt) =
        create_signal(Utc::now().with_timezone(&FixedOffset::east_opt(-4 * 3600).unwrap()));
    let (default_time, set_default_time) = create_signal("00:00:00".to_string());
    let (deals_data, set_deals_data) = create_signal(Deals::default());

    let (counterparty_name, set_counterparty_name) = create_signal(String::default());
    let (collateral_exchange_settlement, set_collateral_exchange_settlement) =
        create_signal(String::from("Deliverable"));
    let (base_currency, set_base_currency) = create_signal("".to_string());
    let (qoute_currency, set_qoute_currency) = create_signal("".to_string());
    let (currency, set_currency) = create_signal("".to_string());
    let (_ccy1_amount, set_ccy1_amount) = create_signal(0.0);
    let (ccy2_amount, set_ccy2_amount) = create_signal(0.0);
    let (group_id, set_group_id) = create_signal("".to_string());
    let (strike_amount, set_strike_amount) = create_signal(0.0);
    let (notional_amount, set_notional_amount) = create_signal(0.0);
    let (conditional_loss_level, set_conditional_loss_level) = create_signal(0.0);
    let (spot, set_spot) = create_signal(0.0);
    let (jabra_side, set_jabra_side) = create_signal("buy".to_string());
    let (amount, set_amount) = create_signal(0.0);
    let (r1, set_r1) = create_signal(0.0);
    let (r2, set_r2) = create_signal(0.0);
    let (option_kind, set_option_kind) = create_signal("".to_string());
    let (iv_t1, set_iv_t1) = create_signal(0.0);
    let (px_in_base_ccy, set_px_in_base_ccy) = create_signal(0.0);
    let (px_in_quote_ccy, set_px_in_quote_ccy) = create_signal(0.0);
    let (can_submit, set_can_submit) = create_signal(false);
    let (settlement_options, set_settlement_options) =
        create_signal(SettlementOptionResponse::default());
    let settlement_options_memo = move || settlement_option_response.clone();
    let (date_created, set_date_created) = create_signal("".to_string());
    let (formatted_date_created, set_formatted_date_created) = create_signal("".to_string());
    let settlement_ccy = RwSignal::new("N/A".to_string());
    let show_settlement_ccy = move || collateral_exchange_settlement.get() == "Cash";
    let expiry_timestamp = RwSignal::new(String::from(""));

    create_effect(move |_| {
        set_date_created.set(
            date_created_query
                .get()
                .unwrap_or_default()
        );
        set_formatted_date_created.set(
            date_created_query
                .get()
                .unwrap_or_default()
                .split("T")
                .next()
                .unwrap_or_default()
                .to_string(),
        );

        let query = query_signal.get().unwrap_or_default();
        let decoded = general_purpose::URL_SAFE_NO_PAD.decode(&query).unwrap();

        let deals = serde_json::from_str::<Deals>(String::from_utf8(decoded).unwrap().as_str());
        set_deals_data.set(deals.unwrap());
        expiry_timestamp.set(deals_data.get().expiry_timestamp);
        set_counterparty_name.set(cp_signal.get().unwrap_or_default());
        set_base_currency.set(deals_data.get().base_currency);
        set_qoute_currency.set(deals_data.get().qoute_currency);
        set_ccy1_amount.set(deals_data.get().ccy1_amount);
        set_ccy2_amount.set(deals_data.get().ccy2_amount.abs()); // convert to absolute value
        set_strike_amount.set(deals_data.get().strike);
        set_currency.set(deals_data.get().currency);
        set_spot.set(deals_data.get().spot);
        set_jabra_side.set(deals_data.get().jabra_side);
        set_amount.set(deals_data.get().amount.abs()); //convert to absolute value
        set_r1.set(deals_data.get().r1);
        set_r2.set(deals_data.get().r2);
        set_iv_t1.set(deals_data.get().iv_t1);
        set_px_in_base_ccy.set(deals_data.get().px_in_base_ccy);
        set_px_in_quote_ccy.set(deals_data.get().px_in_quote_ccy);
        set_option_kind.set(deals_data.get().option_kind);
        set_default_time.set(get_time_in_local_time(expiry_timestamp.get()));
        set_group_id.set(group_id_signal.get().unwrap_or_default());
        settlement_ccy.set(deals_data.get().base_currency);
        match deals_data.get().currency == deals_data.get().base_currency {
            true => set_notional_amount.set(deals_data.get().amount.abs()),
            false => set_notional_amount.set(deals_data.get().amount.abs() / deals_data.get().spot),
        };

        //Term dates
        let now = Utc::now();
        let expiry = get_expiration_datetime_utc(Some(now), deals_data.get().expiry_in_days);
        set_expiry_date.set(expiry);
        set_option_kind.set(deals_data.get().option_kind);
        deals_data
    });

    let expiry_date_memo = create_memo(move |_| expiry_date.get());
    create_effect(move |_| {
        set_expiry_edt.set(convert_utc_to_edt(expiry_date_memo.get()));
    });

    let expiry_time_memos = create_memo(move |_| default_time.get());
    create_effect(move |_| {
        let new_edt_expiry = update_edt_time(expiry_edt.get(), &expiry_time_memos.get());
        set_expiry_edt.set(new_edt_expiry);
    });

    let conditional_loss_memo = move || conditional_loss_level.get();
    let (conditional_loss_event, set_conditional_loss_event) = create_signal("N/A".to_string());
    create_effect(move |_| {
        let clm = conditional_loss_memo();
        if clm > 0.0 {
            let cle = get_conditional_loss(
                option_kind.get(),
                conditional_loss_level.get(),
                notional_amount.get(),
                ccy2_amount.get(),
                strike_amount.get(),
                counterparty_name.get(),
                base_currency.get(),
            );
            set_conditional_loss_event.set(cle);
        } else {
            set_conditional_loss_event.set("N/A".to_string());
        }
        conditional_loss_memo
    });

    create_effect(move |_| {
        let s = settlement_options_memo();
        set_settlement_options.set(s.get());
        settlement_options_memo
    });

    let dispatch_settlement_option_action = move || {
        set_can_submit.set(true);
        settlement_options_action.dispatch(SettlementOptionRequest {
            spot_t1: spot.get(),
            strike: strike_amount.get(),
            deposit: amount.get(),
            ccy2_premium: ccy2_amount.get(),
            counterparty_name: counterparty_name.get(),
            pair_name: format!("{}/{}", base_currency.get(), qoute_currency.get()),
            base_ccy: base_currency.get(),
            term_ccy: qoute_currency.get(),
            deposit_ccy: if currency.get() == base_currency.get() {
                "ccy1".to_string()
            } else {
                "ccy2".to_string()
            },
            call_or_put: option_kind.get().to_lowercase(),
            collateral_exchange_settlement: collateral_exchange_settlement.get().to_lowercase(),
            jabra_side: jabra_side.get(),
        })
    };

    let dispatch_new_term_sheet_with_group_id = move || {
        set_can_submit.set(false);
        new_term_sheet_action_with_id.dispatch(SubmitNewTermSheetRequestWithGroupId {
            counterparty_name: counterparty_name.get(),
            pair_name: format!("{}/{}", base_currency.get(), qoute_currency.get()),
            base_ccy: base_currency.get(),
            term_ccy: qoute_currency.get(),
            instrument_type: option_kind.get(),
            deal_date: date_created.get(),
            // expiry_date: format_date_offset_for_trades(expiry_edt()),
            expiry_date: expiry_timestamp.get(),
            deposit_amount: amount.get(),
            deposit_ccy: currency.get(),
            spot_t1: spot.get(),
            //ADD SIDE TO MODEL
            // side: side(),
            strike: strike_amount.get(),
            r2: r2.get(),
            r1: r1.get(),
            iv_t1: iv_t1.get(),
            term_sheet: "".to_string(),
            dcl_payment: {
                if settlement_options.get().to_dcl_payment_list().len() < 1 {
                    vec![DclPaymentList {
                        id: 20,
                        settlement_condition: "".to_string(),
                        settlement_value: "".to_string(),
                    }]
                } else {
                    settlement_options.get().to_dcl_payment_list()
                }
            },
            collateral_setting_method: AGENT.to_string(),
            collateral_exchange_settlement: collateral_exchange_settlement.get().to_string(),
            exchange_rate_determining_agent: AGENT.to_string(),
            stop_loss_level: conditional_loss_level.get(),
            px_in_base_ccy: px_in_base_ccy.get(),
            px_in_quote_ccy: px_in_quote_ccy.get(),
            conditional_loss_limit_event: conditional_loss_event.get(),
            group_id: group_id.get(),
            settlement_ccy: if collateral_exchange_settlement.get() == "Cash" {
                settlement_ccy.get()
            } else {
                String::from("")
            },
        });
    };

    view! {
        <div class = "p-4">
            <div class="pb-5 ml-2 text-xl font-bold text-white">
                <span>Deal Preview</span>
            </div>
            <div class = "flex flex-wrap gap-3">
                <div class = "flex-auto lg:flex-initial w-full lg:w-1/5 bg-base-300 bg-opacity-50 rounded p-2">
                    <form class = "flex flex-col gap-1" on:submit=|ev| ev.prevent_default()>
                        <div class="text-lg font-bold text-white">
                            <span>Deal Input</span>
                        </div>
                        // <div class = "mx-6">
                            <label class = "text-xs block font-light" for="counter_party">Counterparty</label>
                            <input class = "input input-xs w-full text-sm rounded border border-success border-opacity-70 pointer-events-none opacity-70" type="text" id="counter_party" name="counter_party" readonly prop:value = counterparty_name/>
                        // </div>
                        // <div class = "m-6" class = "mx-6">
                            <label class = "text-xs block font-light" for="trade_date">Trade Date</label>
                            // <Transition fallback = move || view! {<span class="loading loading-bars loading-sm"></span>}>
                                <input class = "input input-xs w-full text-sm rounded border border-success border-opacity-70 pointer-events-none opacity-70" type="date" id="trade_date" name="trade_date" readonly prop:value = formatted_date_created prop:min = formatted_date_created  />
                            // </Transition>
                        // </div>
                        // <div class = "mx-6">
                            <label class = "text-xs block font-light" for="expiry_date">Expiry Date</label>
                            <input class = "input input-xs w-full text-sm rounded border border-success border-opacity-70 pointer-events-none opacity-70" type="date" id="expiry_date" name="expiry_date" prop:value = move || get_date_in_local_time(expiry_timestamp.get()) readonly />
                        // </div>
                        // <div class = "mx-6">
                            // <label class = "block" for="expiry_time">Expiry Time (US/Eastern)</label>
                            <label class = "text-xs block font-light" for="expiry_time">Expiry Time</label>
                            <input class = "input input-xs w-full text-sm rounded border border-success border-opacity-70 pointer-events-none opacity-70" type="time" id="expiry_time" name="expiry_time" step = "1" value = default_time readonly />
                        // </div>
                        // <div class = "mx-6">
                            <label class = "text-xs block font-light" for="final_option_price">Enter Final Option Price (USD)</label>
                            <input class = "input input-xs w-full text-sm rounded border border-success border-opacity-70 pointer-events-none opacity-70" type="number" id="final_option_price" readonly name="final_option_price" placeholder = "0.00" value = move || ccy2_amount.get()
                                on:change = move |event| {
                                    let val: f64 = event_target_value(&event).parse().unwrap();
                                    set_ccy2_amount.set(val);
                                }
                            />
                        // </div>
                        // <div class = "mx-6">
                            <label class = "text-xs block font-light" for="collateral_exchange_settlement">Collateral Exchange Settlement</label>
                            <select class = "select w-full text-sm rounded shadow-md select-sm hover:shadow-md" name="collateral_exchange_settlement" id="collateral_exchange_settlement" on:change = move |event| {
                                let val: String = event_target_value(&event);
                                set_collateral_exchange_settlement.set(val);
                                }
                            >
                                <option value="Deliverable">Deliverable</option>
                                <option value="Cash">Cash</option>
                            </select>
                        // </div>
                        <Show when = move || show_settlement_ccy()>
                            // <div class = "mx-6">
                                <label class = "text-xs block font-light" for="collateral_exchange_settlement">Settlement CCY</label>
                                <select class = "select select-xs w-full text-sm rounded shadow-sm shadow-success hover:shadow-md text-white" name="settlement_ccy" on:change = move |event| {
                                    let val: String = event_target_value(&event);
                                    settlement_ccy.set(val);
                                    }
                                >
                                    <option value=base_currency.get()>{base_currency.get()}</option>
                                    <option value=qoute_currency.get()>{qoute_currency.get()}</option>
                                </select>
                            // </div>
                        </Show>
                        // <div class = "mx-6">
                            <label class = "text-xs block font-light" for="stop_loss_level">Stop Loss Level (USD)</label>
                            <input class = "input input-xs w-full text-sm rounded border border-success border-opacity-70 opacity-70" type="number" id="stop_loss_level" name="stop_loss_level" placeholder = "0" step = "1" min = "0"
                            on:change = move |event| {
                                let val: f64 = event_target_value(&event).parse().unwrap();
                                set_conditional_loss_level.set(val);
                            }
                            />
                        // </div>
                        // <div class = "mx-6 my-3">
                            <button class="w-full rounded btn btn-success btn-sm" on:click = move |_| dispatch_settlement_option_action() >PREVIEW TERMSHEET</button>
                        // </div>
                    </form>
                </div>

                
                <div class = "flex-auto bg-base-300 bg-opacity-50 rounded p-2">
                    // <div class = "m-5">
                        <div class="text-lg font-bold text-white">
                            <span>Deal Summary</span>
                        </div>
                        <Show
                                when = move || preview_enabled.get()
                                fallback = || view! {<span></span>} >
                        <div class="text-sm border-t rounded-sm shadow border-x border-x-gray-700 border-t-gray-700">
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Counterparty Name</span></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{counterparty_name}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Trade Date</span></div>
                                // <Transition fallback = move || view! {<span class="loading loading-bars loading-sm"></span>}>
                                    <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{formatted_date_created.get()}</span></div>
                                // </Transition>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Term 2 Settlement Date</span></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{move || get_date_in_local_time(expiry_timestamp.get())}</span></div>
                                // <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><p>{expiry_timestamp.get()}</p></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Interest Price</span></div>
                                // <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><p>{move || format!("{:.4}",ccy2_amount())}</p></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{move || ccy2_amount.get()}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Base Currency</span></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{base_currency}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Trade Date Reference Exchange Rate</span></div>
                                // <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><p>{move || format!("{:.4}",spot())}</p></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{spot.get()}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Term Currency</span></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{qoute_currency}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">{move || format!("Trade Date {} Notional Amount (deposit)", currency.get())}</span></div>
                                // <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><p>{move || format!("{:.2} {}", amount(), currency())}</p></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{move || format!("{} {}", amount.get(), currency.get())}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Term 2 Settlement Time</span></div>
                                // <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><p>{settlement_time}</p></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{get_datetime_in_local_time(expiry_timestamp.get())}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Conditional Lost Limit Event</span></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{conditional_loss_event}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Collateral Setting Methodology</span></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{AGENT}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Collateral Exchange Settlement</span></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{collateral_exchange_settlement}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">Settlement CCY</span></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{move || if collateral_exchange_settlement.get() == "Cash" {settlement_ccy.get()} else {String::from("N/A")}}</span></div>
                            </div>
                            <div class = "flex">
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">CCY1/CCY2 exchange rate determining agent on T2</span></div>
                                <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{AGENT}</span></div>
                            </div>

                            // <Transition fallback = move || view!{<div><span class="loading loading-bars loading-sm"></span></div>}>
                                    {
                                        move || {
                                            settlement_option_response.get().data.into_iter().map(|opt| {
                                                let val = opt.settlement_value.clone();
                                                view! {
                                                    <div class = "flex">
                                                        <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "text-success">{opt.settlement_condition}</span></div>
                                                        <div class = "flex-initial w-1/2 p-1 border-b border-b-gray-700"><span class = "font-extralight text-white">{format_number_en(val, 2)}</span></div>
                                                    </div>
                                                }
                                            }).collect_view()
                                        }
                                    }
                            // </Transition>
                    </div>
                    <div class = "flex justify-end gap-3 mt-2">
                        <div class = "">
                            <button class="rounded btn btn-success btn-sm" prop:disabled=move || can_submit.get() == false on:click = move |_| set_show_confirm_modal.set(true)>SUBMIT</button>
                        </div>
                        <div>
                            <Form action = "/trades/recents" method = "get">
                            <button class="rounded btn btn-warning btn-sm">START OVER</button>
                            </Form>
                        </div>
                    </div>
                    </Show>
                </div>
                // </div>
            </div>

            // REUSABLE CONFIRM MODAL
            // <Transition fallback = move || view! {<span class="loading loading-bars loading-sm"></span>}>
            {
                move || {
                    view! {
                        <ConfirmModal
                            when = show_confirm_modal.get()
                            write_signal = set_show_confirm_modal
                            function = dispatch_new_term_sheet_with_group_id
                            action = new_term_sheet_action_with_id
                        />
                    }
                
                }
            }
            // </Transition>

            {
                move || match submit_new_term_sheet_response_show.get() {
                    true => if submit_new_term_sheet_response.get().status == -1 {
                        view! {
                            <ErrorModal
                                read_signal = show_error_modal
                                write_signal = set_show_error_modal
                                message = "Something unexpected went wrong".to_string()
                            />
                        }.into_view()
                    } else if submit_new_term_sheet_response.get().status == 0 {
                        view! {
                            <SuccessModalWithRedirection
                                read_signal = show_success_modal
                                write_signal = set_show_success_modal
                                message = submit_new_term_sheet_response.get().refid
                                url= String::from("/trades/recents")
                            />
                        }.into_view()
                    } else {
                        view!{<div></div>}.into_view()
                    },
                    false => view!{<div></div>}.into_view(),
                }
            }

        </div>

    }
}
