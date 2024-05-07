use leptos::*;
use serde_json::json;

use crate::{commons::models::trade::ExtractedTrade, components::{component_size::ComponentSize, component_type::ComponentType, data_table_local::DataTable, default_none::DefaultNone, loading_spinners::Spinners, menu_button::MenuButton}, features::trades::expiring::services::get_trade_history};

#[allow(non_snake_case)]
#[component]
pub fn ExpiringTrades() -> impl IntoView {

    let trade_history_resource = create_local_resource(|| (), move |_| get_trade_history());
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
        <div class = "p-4">
            <div class="pb-5 ml-2 text-xl font-bold text-white">
                <span>Expiring Trades</span>
            </div>
            <div class = "flex justify-between">
                <div class = "flex flex-wrap justify-start gap-4 flex-0">
                {
                    let page_keys = vec![String::from("All"), String::from("Upcoming Expiry"),String::from("Expired")];
                    page_keys.into_iter().map(|k| {
                        view! {
                            <MenuButton selected_page = selected_page page = k.clone() name = k.clone() />
                        }
                    }).collect_view()
                }
                </div>
            </div>
        
        <Suspense
            fallback = move || view! {
                    <div class = "items-center mt-5">
                        <div class = "flex justify-center ">
                            <Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS />
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
                            let headersource = RwSignal::new(vec![String::from("Date Created"),String::from("Client"),String::from("Market"), String::from("Side"), String::from("Type"), String::from("Trade Type"), String::from("Kind"), String::from("Size"), String::from("Price"), String::from("Index Price"), String::from("Realized PNL"), String::from("Status")]);
                            let datakeysource = RwSignal::new(vec![String::from("date_created"),String::from("party_b"), String::from("market"), String::from("side_status"), String::from("trans_type"), String::from("trade_type"), String::from("kind"), String::from("size"),String::from("price"), String::from("index_price"), String::from("realized_pnl"), String::from("trade_status")]);
                            let key_to_display_in_mobile = RwSignal::new(String::from("market"));
                            let number_keys = RwSignal::new(vec![String::from("size"),String::from("price"), String::from("realized_pnl"), String::from("index_price")]);
                            let currency_vec = RwSignal::new(vec![
                                json!({"key": "price", "value": "premium_ccy"}),
                                json!({"key": "realized_pnl", "value": "realized_pnl_ccy"}),
                            ]);
                            let download_file_name = RwSignal::new(String::from("ITM_OTM"));
                            let file_content = RwSignal::new(String::from("Test conetent"));
                            let row_slice = RwSignal::new(25);
                            let no_data_message = RwSignal::new(String::from("No trades with upcoming expiry and expired trades at the moment."));
                            if selected_page.get() == "All".to_string() {
                                trade_table.set(t.extract());
                                file_content.set(t.extract_csv_by_trade_status(Option::None, false));
                            view! {
                                <DataTable headers=headersource
                                keys=datakeysource
                                key_to_display_in_mobile
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
                            } else if selected_page.get() == "Upcoming Expiry".to_string() {
                                trade_table.set(t.extract_by_trade_status(String::from("OPEN"), false));
                                file_content.set(t.extract_csv_by_trade_status(Some(String::from("OPEN")), false));
                                download_file_name.set(String::from("UPCOMING_EXPIRY_LAST_7_DAYS"));
                                let no_data_message = RwSignal::new(String::from("No trades with upcoming expiry at the moment."));
                                view! {
                                    <DataTable headers=headersource
                                    keys=datakeysource
                                    key_to_display_in_mobile
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
                            }  else if selected_page.get() == "Expired".to_string() {
                                trade_table.set(t.extract_by_trade_status(String::from("OPEN"), true));
                                file_content.set(t.extract_csv_by_trade_status(Some(String::from("OPEN")), true));
                                download_file_name.set(String::from("EXPIRED_LAST_7_DAYS"));
                                let no_data_message = RwSignal::new(String::from("No expired trades at the moment."));
                                view! {
                                    <DataTable headers=headersource
                                    keys=datakeysource
                                    key_to_display_in_mobile
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
                            }else {
                                view! {
                                    <div class = "items-center mt-5">
                                        <div class = "flex justify-center">
                                            <DefaultNone text = RwSignal::new("The selected page does not exist.".to_string()) />
                                        </div>
                                    </div>
                                }.into_view()
                            }
                        }
                        Err(_) => view! {
                            <div class = "items-center mt-5">
                                <div class = "flex justify-center ">
                                    <DefaultNone text = RwSignal::new("Cannot connect to server. Please refresh page.".to_string()) />
                                </div>
                            </div>
                        }.into_view(),
                    }
                } else {
                    view! {
                        <div class = "p-5">
                            <DefaultNone text = RwSignal::new("No trade record found".to_string()) />
                        </div>
                    }.into_view()
                }
            }
        }
        </Suspense>
        </div>
    }
}
