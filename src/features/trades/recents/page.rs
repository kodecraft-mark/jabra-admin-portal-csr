use std::collections::HashMap;

use base64::engine::general_purpose;
use base64::Engine;
use leptos::*;
use leptos_router::Form;

use crate::commons::models::trade::Trade;
use super::models::{TradeDataForModification, TradeFilterForModification, TradeForModification, TradeGroupidForModification, TradeQueryForModification, ModifyTradeResponse};
use crate::components::component_size::ComponentSize;
use crate::components::component_type::ComponentType;
use crate::components::error_modal::ErrorModal;
use crate::components::loading_spinners::Spinners;
use crate::components::success_then_refetch_modal::SuccessModalWithRefetch;
use crate::features::trades::recents::models::Deals;
use crate::utilities::configuration::get_environment;
use crate::utilities::date_util::{convert_utc_to_local, get_expiry};
use crate::features::trades::recents::services::fetch_recent_trades;
use super::services::edit_trade;

#[allow(non_snake_case)]
#[component]
pub fn RecentTrades() -> impl IntoView {
    let environment_resource = create_local_resource(|| (), move |_| async move { get_environment().await });
    let recent_trades_resource: Resource<(), Result<HashMap<String, Vec<Trade>>, ServerFnError>> = create_local_resource(|| (), move |_| async move { fetch_recent_trades().await });
    view! {
        <div class="p-4">
            <div class="pb-5 ml-2 text-xl font-bold text-white">
                <span>Recent Trades</span>
            </div>
                <div class = "py-3 mb-3">
                    <Transition fallback = move || view! {<div class = "flex justify-center "><Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS /></div>}>
                        {
                        move || {
                            recent_trades_resource.and_then(|e| {
                                if e.is_empty() {
                                    return view! {<div class = "flex justify-center border-b border-b-gray-700"><p>No available trades</p></div>}.into_view();
                                }
                                e.into_iter().map(|(counter_party, trade_quotes)| {

                                    let counterparty_name = counter_party.split("~").collect::<Vec<&str>>()[0].to_string();
                                    let _counterparty_id = counter_party.split("~").collect::<Vec<&str>>()[1].to_string();

                                    let (counterparty_name_list, _set_counterparty_name_list) = create_signal(counterparty_name.clone());

                                    //Clone all trade quotes of a certain counter party here to enable reactivity
                                    let (trade_quotes_list, _set_trade_quotes_list) = create_signal(trade_quotes.clone());



                                    view! {
                                        <div class = "mb-5 border border-opacity-50 shadow border-success">
                                            <div>
                                                <span class = "block px-2 text-sm font-semibold text-white uppercase bg-base-300">{counterparty_name.clone()}</span>
                                            </div>
                                        <table class = "table table-xs table-zebra-zebra ">
                                            <thead>
                                                // <tr class = "bg-opacity-20">
                                                //     <th colspan = "7" >{counterparty_name.clone()}</th>
                                                // </tr>
                                                <tr class = "font-semibold text-white bg-success bg-opacity-30">
                                                    <th>INSTRUMENT</th>
                                                    <th>AMOUNT</th>
                                                    <th>SIDE</th>
                                                    <th>BASE QUOTE</th>
                                                    <th>QUOTE COST</th>
                                                    <th>DATE CREATED</th>
                                                    <th>ACTION</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {
                                                    move ||  {
                                                        trade_quotes_list.get().into_iter().map(|tq| {

                                                            let mod_trades = RwSignal::new(tq.clone());
                                                            // log::info!("Trade Quotes: {:?}", mod_trades.get());
                                                            let show_edit_modal = RwSignal::new(false);
                                                            let group_id = move || tq.group_id.clone();
                                                            let date_created = move || tq.date_created.clone();

                                                            let is_pos_class_s = if tq.side == "Sell" {"text-error uppercase hover:opacity-50"} else {"text-success uppercase hover:opacity-50"};

                                                            let is_pos_class_a = if tq.amount.unwrap_or(0.0) >= 0.0 {"text-success uppercase hover:opacity-50"} else {"text-error uppercase hover:opacity-50"};
                                                            let is_pos_value_a = if tq.amount.unwrap_or(0.0) >= 0.0 {format!("+{}",tq.amount.unwrap_or(0.0))} else {tq.amount.unwrap_or(0.0).to_string()};
                                                            let is_pos_class_b = if tq.px_in_base_ccy.unwrap_or(0.0) >= 0.0 {"text-success uppercase hover:opacity-50"} else {"text-error uppercase hover:opacity-50"};
                                                            let is_pos_value_b = if tq.px_in_base_ccy.unwrap_or(0.0) >= 0.0 {format!("+{}",tq.px_in_base_ccy.unwrap_or(0.0))} else {tq.px_in_base_ccy.unwrap_or(0.0).to_string()};
                                                            let is_pos_class_q = if tq.px_in_quote_ccy.unwrap_or(0.0) >= 0.0 {"text-success uppercase hover:opacity-50"} else {"text-error uppercase hover:opacity-50"};
                                                            let is_pos_value_q = if tq.px_in_quote_ccy.unwrap_or(0.0) >= 0.0 {format!("+{}",tq.px_in_quote_ccy.unwrap_or(0.0))} else {tq.px_in_quote_ccy.unwrap_or(0.0).to_string()};


                                                            let trade_deal = Deals {
                                                                base_currency: tq.base_currency_id.ticker,
                                                                qoute_currency: tq.quote_currency_id.ticker,
                                                                currency: tq.ccy_id.ticker,
                                                                expiry_in_days: tq.ttm.unwrap_or(0.0),
                                                                ccy1_amount: tq.px_in_base_ccy.unwrap_or(0.0),
                                                                ccy2_amount: tq.px_in_quote_ccy.unwrap_or(0.0),
                                                                strike: tq.strike,
                                                                amount: tq.amount.unwrap_or(0.0),
                                                                option_kind: tq.option_kind.unwrap(),
                                                                spot: tq.spot.unwrap_or(0.0),
                                                                r1: tq.r1.unwrap_or(0.0),
                                                                r2: tq.r2.unwrap_or(0.0),
                                                                iv_t1: tq.iv.unwrap_or(0.0),
                                                                px_in_base_ccy: tq.px_in_base_ccy.unwrap_or(0.0),
                                                                px_in_quote_ccy: tq.px_in_quote_ccy.unwrap_or(0.0),
                                                                jabra_side: tq.side.to_lowercase(),
                                                                expiry_timestamp: tq.expiry_timestamp
                                                            };


                                                            let encrypted_deal_data = move || {
                                                                general_purpose::URL_SAFE_NO_PAD.encode(serde_json::to_string(&trade_deal).unwrap())
                                                            };
                                                            let show_modify_alert_modal = RwSignal::new(false);
                                                            let modify_trade_response = RwSignal::new(ModifyTradeResponse::default());
                                                            let (show_modify_success_modal, set_show_modify_success_modal) = create_signal(true);
                                                            let (show_modify_error_modal, set_show_modify_error_modal) = create_signal(true);
                                                            let (show_test, set_show_test) = create_signal(false);
                                                            let modify_trade_action: Action<TradeForModification, ()> = create_action(move |req: &TradeForModification| {

                                                                        let request = req.clone();

                                                                        async move {
                                                                            if request.data.expiry_timestamp=="" {

                                                                                show_modify_alert_modal.set(true);
                                                                                            modify_trade_response.update(|v| {
                                                                                                v.success = false;
                                                                                                v.message = String::from("No changes made.");
                                                                                            });

                                                                            }else{
                                                                                // approve_trade_quote(request).await
                                                                                let result = edit_trade(request).await;
                                                                                match result {
                                                                                    Ok(res) => {
                                                                                        if res {
                                                                                            show_modify_alert_modal.set(true);
                                                                                            modify_trade_response.update(|v| {
                                                                                                v.success = true;
                                                                                                v.message = String::from("Trade Expiry updated successful.");
                                                                                            });
                                                                                            // active_quotes_resource.refetch();
                                                                                        } else {
                                                                                            show_modify_alert_modal.set(true);
                                                                                            modify_trade_response.update(|v| {
                                                                                                v.success = false;
                                                                                                v.message =  String::from("Failed request, Please try again!.")
                                                                                            });
                                                                                        }
                                                                                    },
                                                                                    Err(_e) => {
                                                                                        show_modify_alert_modal.set(true);
                                                                                        modify_trade_response.update(|v| {
                                                                                            v.success = false;
                                                                                            v.message = String::from("Your session has ended. Please relog your account.")
                                                                                        });
                                                                                    }
                                                                                }
                                                                            }

                                                                        }
                                                                    });

                                                            view! {
                                                                <tr>
                                                                    <td class = "uppercase hover:opacity-50">{tq.venue_instrument_name}</td>
                                                                    <td class = {is_pos_class_a}>{is_pos_value_a}</td>
                                                                    <td class = {is_pos_class_s}>{tq.side}</td>
                                                                    <td class = {is_pos_class_b}>{is_pos_value_b}</td>
                                                                    <td class = {is_pos_class_q}>{is_pos_value_q}</td>
                                                                    <td>{convert_utc_to_local(&date_created().clone().unwrap())}</td>
                                                                    <td class="flex flex-row">
                                                                        <Form method = "get" action = "/trades/deals">
                                                                            <input type="hidden" name = "query" value = encrypted_deal_data />
                                                                            <input type="hidden" name = "cp_name" prop:value = counterparty_name_list/>
                                                                            <input type="hidden" name = "group_id" value = group_id/>
                                                                            <input type="hidden" name = "date_created" value = date_created/>
                                                                            <button class = "btn btn-xs btn-success mr-7">CREATE DEAL</button>
                                                                        </Form>
                                                                        <Suspense>
                                                                        { move || {

                                                                            let mut envi = String::from("");

                                                                            environment_resource.map(|e| envi =  e.to_uppercase() );
                                                                             if envi==String::from("DEVELOPMENT"){
                                                                                 view!{
                                                                                    <button class = "mr-2 btn btn-xs btn-warning" on:click = move |_| show_edit_modal.set(true) >EDIT</button>
                                                                                }
                                                                           }else{
                                                                                view!{
                                                                                   <button hidden></button>
                                                                               }
                                                                           }

                                                                        }
                                                                        }
                                                                        </Suspense>
                                                                    </td>
                                                                </tr>
                                                                {
                                                                    move || {
                                                                        view! {
                                                                            <EditTradesModal
                                                                                show = show_edit_modal
                                                                                trades = mod_trades
                                                                                action = modify_trade_action
                                                                            />
                                                                        }
                                                                    }
                                                                }
                                                                {
                                                                    move || match show_modify_alert_modal.get() {
                                                                        true => if !modify_trade_response.get().success  {
                                                                            view! {
                                                                                <ErrorModal
                                                                                    read_signal = show_modify_error_modal
                                                                                    write_signal = set_show_modify_error_modal
                                                                                    message = modify_trade_response.get().message
                                                                                />
                                                                                }
                                                                        } else {
                                                                            view! {
                                                                                <SuccessModalWithRefetch
                                                                                    read_signal = show_modify_success_modal
                                                                                    write_signal = set_show_modify_success_modal
                                                                                    message = modify_trade_response.get().message
                                                                                    resource = recent_trades_resource
                                                                                />
                                                                                }
                                                                        }.into_view(),
                                                                        false => view! {   <Show when=move || show_test.get() fallback=|| ()><tr></tr></Show>}.into_view(),
                                                                    }
                                                                }
                                                            }
                                                        }).collect_view()
                                                    }
                                                }
                                                </tbody>
                                                </table>
                                            </div>
                                    }
                                }).collect_view()
                            })
                        }
                    }
                    </Transition>
                </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn EditTradesModal(
    /// The RwSignal of type boolean to show the modal.
    show: RwSignal<bool>,
    /// The RwSignal of type `QuoteOption` containing the quote option to be edited.
    trades: RwSignal<Trade>,

    action: Action<TradeForModification, ()>,
) -> impl IntoView {
    // The RwSignal of type u16 to store the expiry timestamp.
    let expiry: RwSignal<u16> = RwSignal::new(99);

    let dispatch = move || {
        // Creating instances for nested structs
        let group_id = TradeGroupidForModification {
            _eq: String::from(trades.get_untracked().group_id),
        };

        let filter = TradeFilterForModification { group_id };

        let query = TradeQueryForModification { filter };

        // Creating an instance of TradeDataForModification
        let trade_data = TradeDataForModification {
            expiry_timestamp: String::from(if expiry.get() == 99 {
                String::from("")
            } else {
                get_expiry(expiry.get())
            }),
        };

        // Creating an instance of TradeForModification using the new function
        let trade_modification = TradeForModification::new(query, trade_data);

        action.dispatch(trade_modification);
        show.set(false);
    };
    let on_cancel = move || {
        expiry.set(99);
        show.set(false);
    };
    let is_pending = action.pending();

    view! {
        <Show when = move || show.get()>
            <div class="blur-bg">
                <div class="flex items-center justify-center h-screen">
                    <div class = "flex flex-col gap-4 m-2 modal-box">
                        <div class = "items-center content-center text-center text-success"><p>{trades.get().venue_instrument_name}</p></div>
                        <div class="divider"></div>
                        <div class="flex flex-col gap-2 mb-10">
                            <div class = "grid grid-cols-2 mb-3">
                                <label colspan="1" class = "block text-sm font-light">Expiry Timestamp</label>
                                <span colspan="2" class="text-sm text-success">{if expiry.get()==99 {convert_utc_to_local(trades.get().expiry_timestamp.as_str())} else{convert_utc_to_local(&get_expiry(expiry.get()).as_str())}}</span>
                            </div>
                            <div class="grid grid-cols-2">
                                <label colspan="1" class = "block text-sm font-light">Trade Expiry</label>
                                <select colspan="2" class = "block w-full text-xs border-gray-800 rounded shadow-md select-sm text-success hover:shadow-sm hover:shadow-success bg-base-100" name = "expiry"
                                    on:change = move |event| {
                                        let val = event_target_value(&event);
                                        expiry.set(val.parse::<u16>().unwrap());
                                    }
                                >
                                    <option value = "" prop:selected=true prop:disabled=true>Select Trade Expiry</option>
                                    <SelectOption label="Now" is=0 value=expiry.clone() />
                                    <SelectOption label="In 3 minuntes" is=3 value=expiry.clone() />
                                    <SelectOption label="In 5 minuntes" is=5 value=expiry.clone() />
                                    <SelectOption label="In 10 minuntes" is=10 value=expiry.clone() />
                                    <SelectOption label="In 15 minuntes" is=15 value=expiry.clone() />
                                </select>
                            </div>
                        </div>
                        <div class = "grid grid-cols-3 gap-3">
                            <div colspan = "1">
                            </div>
                            <div colspan = "1">
                                {
                                    match is_pending.get() {
                                        true => view! {
                                            <button class = "w-full mt-2 btn btn-sm btn-success"><span class="loading loading-spinner loading-sm"></span></button>
                                        }.into_any(),
                                        false => view! {
                                            <button class = "w-full mt-2 btn btn-sm btn-success" on:click = move |_| dispatch()>SUBMIT</button>
                                        }.into_any(),
                                    }
                                }
                            </div>
                            <div colspan = "1">
                                <button class = "w-full mt-2 btn btn-sm btn-error" on:click = move |_| on_cancel()>CANCEL</button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn SelectOption(label: &'static str, is: u16, value: RwSignal<u16>) -> impl IntoView {
    view! {
        <option
            value=is
            selected=move || value.get() == is
        >
            {label}
        </option>
    }
}
