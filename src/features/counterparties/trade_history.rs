use leptos::*;
use serde_json::json;

use crate::commons::models::trade::Trade;

use crate::{commons::models::trade::{ExtractedTrade, TradeHistory}, components::{data_table_local::DataTable, menu_button::MenuButton}};

use super::services::get_trade_history;

/// Component for CounterParty Trade History Page.
/// Has the actual view for Trade History page.

#[allow(non_snake_case)]
#[component]
pub fn CounterPartyTradeHistoryPage(counterparty: RwSignal<String>) -> impl IntoView {
    let trade_history_resource: Resource<String, Result<TradeHistory, ServerFnError>> =
        create_local_resource(counterparty, move |e| get_trade_history(e));
    let trade_table = RwSignal::new(Vec::<ExtractedTrade>::default());
    let selected_page = RwSignal::new("All".to_string());
    let json_value = Signal::derive(move || {
        trade_table
            .get()
            .into_iter()
            .map(serde_json::to_value)
            .collect::<Result<Vec<serde_json::Value>, _>>()
            .expect("Failed to serialize to JSON")
    });
    view! {
            <div  class = "py-4">
                <div class = "flex justify-between">
                    <div class = "flex flex-wrap justify-start gap-4 flex-0">
                    {
                        let page_keys = vec![String::from("All"), String::from("Option"),String::from("Spot"), String::from("Perpetual Futures")];
                        page_keys.into_iter().map(|k| {
                            view! {
                                <MenuButton selected_page = selected_page page = k.clone() name = k.clone() />
                            }
                        }).collect_view()
                    }
                    </div>
                </div>
            </div>
            <Suspense
                fallback = move || view! {
                        <div class = "items-center mt-5">
                            <div class = "flex justify-center ">
                                <crate::components::loading_spinners::Spinners size=crate::components::component_size::ComponentSize::SMALL _type=crate::components::component_type::ComponentType::SUCCESS />
                            </div>
                        </div>
                    }
            >
            {
                move || {
                    if let Some(data) = trade_history_resource.and_then(|e| {e.clone()}) {
                        match data {
                            Ok(t) => {
                                let color_vec = RwSignal::new(vec!(json!({"side_status":[{"key":"sell", "style":"text-error"},{"key":"buy", "style": "text-success"}],"price":[{"key":"-", "style":"text-error"},{"key":"", "style": "text-success"}],"size":[{"key":"-", "style":"text-error"},{"key":"", "style": "text-success"}],"realized_pnl":[{"key":"-", "style":"text-error"},{"key":"", "style": "text-success"}]})));
                                let headersource = RwSignal::new(vec![String::from("Date Created"),String::from("Market"), String::from("Side"), String::from("Type"), String::from("Trade Type"), String::from("Kind"), String::from("Size"), String::from("Price"), String::from("Index Price"), String::from("Realized PNL"), String::from("Status")]);
                                let datakeysource = RwSignal::new(vec![String::from("date_created"),String::from("market"), String::from("side_status"), String::from("trans_type"), String::from("trade_type"), String::from("kind"), String::from("size"),String::from("price"), String::from("index_price"), String::from("realized_pnl"), String::from("trade_status")]);
                                let key_to_display_in_mobile = RwSignal::new(String::from("market"));
                                let number_keys = RwSignal::new(vec![String::from("size"),String::from("price"), String::from("realized_pnl"), String::from("index_price")]);
                                let currency_vec = RwSignal::new(vec![
                                    json!({"key": "price", "value": "premium_ccy"}),
                                    json!({"key": "realized_pnl", "value": "realized_pnl_ccy"}),
                                ]);
                                let download_file_name = RwSignal::new(String::from("ITM_OTM"));
                                let file_content = RwSignal::new(String::from("Test content"));
                                // let fields = RwSignal::new(vec![String::from("venue_instrument_name"),String::from("side"),String::from("activity"),String::from("option_kind"),String::from("amount"),String::from("instrument_kind"),String::from("pnl"),String::from("pnl_ccy"),String::from("pnl_ccy"),String::from("trade_status"),String::from("index_price"),String::from("date_created")]);
                                // let table = RwSignal::new(String::from("/utils/export/trade"));
                                // let conditions = RwSignal::new(QueryBuilder::new());
                                let row_slice = RwSignal::new(15);
                                let no_data_message = RwSignal::new(String::from("No Trade History Available"));
                                if selected_page.get() == "All".to_string() {
                                    trade_table.set(t.extract());
                                    file_content.set(t.extract_csv_by_trade_status(Option::None, false));
                                   view! {
                                       <DataTable headers=headersource
                                            key_to_display_in_mobile
                                            keys=datakeysource
                                            data=json_value
                                            color=color_vec
                                            currency=currency_vec
                                            row_slice=row_slice
                                            hasdownload=true
                                            hasfilter=true
                                            nodatamessage=no_data_message
                                            file_content = file_content
                                            file_name = download_file_name
                                            number_keys = number_keys
                                            haspageslenght=true
                                       />
                                   }
                                } else if selected_page.get() == "Option".to_string() {
                                    trade_table.set(t.extract_by_type(String::from("OPTION")));
                                    let no_data_message = RwSignal::new(String::from("No Option Trade History Available"));
                                    view! {
                                        <DataTable headers=headersource
                                            key_to_display_in_mobile
                                            keys=datakeysource
                                            data=json_value
                                            color=color_vec
                                            currency=currency_vec
                                            row_slice=row_slice
                                            hasdownload=true
                                            hasfilter=true
                                            nodatamessage=no_data_message
                                            file_content = file_content
                                            file_name = download_file_name
                                            number_keys = number_keys
                                            haspageslenght=true
                                        />
                                   }
                                }  else if selected_page.get() == "Spot".to_string() {
                                    trade_table.set(t.extract_by_type(String::from("SPOT")));
                                    let no_data_message = RwSignal::new(String::from("No Spot Trade History Available"));
                                    view! {
                                        <DataTable headers=headersource
                                            key_to_display_in_mobile
                                            keys=datakeysource
                                            data=json_value
                                            color=color_vec
                                            currency=currency_vec
                                            row_slice=row_slice
                                            hasdownload=true
                                            hasfilter=true
                                            nodatamessage=no_data_message
                                            file_content = file_content
                                            file_name = download_file_name
                                            number_keys = number_keys
                                            haspageslenght=true
                                        />
                                   }
                                }else if selected_page.get() == "Perpetual Futures".to_string() {
                                    view! {
                                        <div class = "p-5">
                                            <span class = "opacity-50 font-extralight">Perpetual Future is not currently supported.</span>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! {
                                        <div class = "items-center mt-5">
                                            <div class = "flex justify-center">
                                                <span class = "opacity-50 font-extralight">The selected page does not exist.</span>
                                            </div>
                                        </div>
                                    }.into_view()
                                }
                            }
                            Err(_) => view! {
                                <div class = "items-center mt-5">
                                    <div class = "flex justify-center ">
                                        <span class = "opacity-50 font-extralight">Cannot connect to server. Please refresh page.</span>
                                    </div>
                                </div>
                            }.into_view(),
                        }
                    } else {
                        view! {
                            <div class = "p-5">
                                <span class="opacity-50 font-extralight">No trade record found</span>
                            </div>
                        }.into_view()
                    }
                }
            }
            </Suspense>
        }.into_view()
}