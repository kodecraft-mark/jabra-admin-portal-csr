use leptos::*;
use serde_json::json;

use crate::commons::models::trade::{ExtractedTrade, TradeHistory};
use crate::components::component_size::ComponentSize;
use crate::components::component_type::ComponentType;
use crate::components::data_table_local::DataTable;
use crate::components::loading_spinners::Spinners;
use crate::components::menu_button::MenuButton;
use crate::components::default_none::DefaultNone;
use crate::features::trades::history::services::get_trade_history;


#[allow(non_snake_case)]
#[component]
pub fn TradeHistoryPage() -> impl IntoView {
    let trade_history_resource = create_local_resource(|| (), move |_| get_trade_history());
    let trade_history = RwSignal::new(TradeHistory::default());
    view! {
        <div class="p-4">
            <div class="pb-5 ml-2 text-xl font-bold text-white">
                <span>Trade History</span>
            </div>
            <Suspense fallback=move || {
                view! {
                    <div class="items-center mt-5">
                        <div class="flex justify-center ">
                            <Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS />
                        </div>
                    </div>
                }
            }>

                {move || {
                    trade_history_resource
                        .and_then(|c| {
                            trade_history.set(c.clone());
                            if trade_history.get().data.len() <= 0 {
                                let text = RwSignal::new(String::from("The selected page does not exist."));
                                view! {
                                    <div class="items-center mt-5">
                                        <DefaultNone text = text/>
                                    </div>
                                }.into_view()
                            } else {
                                let data = trade_history;
                                let trade_table = RwSignal::new(
                                    Vec::<ExtractedTrade>::default(),
                                );
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
                                    <div class="flex justify-start gap-4 ml-4 flex-0 ">

                                        {
                                            let page_keys = vec![
                                                String::from("All"),
                                                String::from("Option"),
                                                String::from("Spot"),
                                                String::from("Perpetual Futures"),
                                            ];
                                            page_keys
                                                .into_iter()
                                                .map(|k| {
                                                    view! {
                                                        <MenuButton
                                                            selected_page=selected_page
                                                            page=k.clone()
                                                            name=k.clone()
                                                        />
                                                    }
                                                })
                                                .collect_view()
                                        }
                                    </div>
                                    <Suspense fallback=move || {
                                        view! {
                                            <div class="items-center mt-5">
                                                <div class="flex justify-center ">
                                                    <Spinners size = ComponentSize::SMALL _type = ComponentType::SUCCESS />
                                                </div>
                                            </div>
                                        }
                                    }>
                                        {move || {
                                            let color_vec = RwSignal::new(
                                                vec![
                                                    json!(
                                                        { "side_status" : [{ "key" : "sell", "style" : "text-error"
                                                        }, { "key" : "buy", "style" : "text-success" }], "price" :
                                                        [{ "key" : "-", "style" : "text-error" }, { "key" : "",
                                                        "style" : "text-success" }], "size" : [{ "key" : "-",
                                                        "style" : "text-error" }, { "key" : "", "style" :
                                                        "text-success" }], "realized_pnl" : [{ "key" : "-", "style"
                                                        : "text-error" }, { "key" : "", "style" : "text-success" }]
                                                        }
                                                    ),
                                                ],
                                            );
                                            let headersource = RwSignal::new(
                                                vec![
                                                    String::from("Date Created"),
                                                    String::from("Market"),
                                                    String::from("Party A"),
                                                    String::from("Party B"),
                                                    String::from("Side"),
                                                    String::from("Type"),
                                                    String::from("Trade Type"),
                                                    String::from("Kind"),
                                                    String::from("Size"),
                                                    String::from("Price"),
                                                    String::from("Index Price"),
                                                    String::from("Realized PNL"),
                                                    String::from("Status"),
                                                ],
                                            );
                                            let datakeysource = RwSignal::new(
                                                vec![
                                                    String::from("date_created"),
                                                    String::from("market"),
                                                    String::from("party_a"),
                                                    String::from("party_b"),
                                                    String::from("side_status"),
                                                    String::from("trans_type"),
                                                    String::from("trade_type"),
                                                    String::from("kind"),
                                                    String::from("size"),
                                                    String::from("price"),
                                                    String::from("index_price"),
                                                    String::from("realized_pnl"),
                                                    String::from("trade_status"),
                                                ],
                                            );
                                            let key_to_display_in_mobile = RwSignal::new(String::from("market"));
                                            let number_keys = RwSignal::new(
                                                vec![
                                                    String::from("size"),
                                                    String::from("price"),
                                                    String::from("realized_pnl"),
                                                    String::from("index_price"),
                                                ],
                                            );
                                            let currency_vec = RwSignal::new(
                                                vec![
                                                    json!({ "key" : "price", "value" : "premium_ccy" }),
                                                    json!(
                                                        { "key" : "realized_pnl", "value" : "realized_pnl_ccy" }
                                                    ),
                                                ],
                                            );
                                            let download_file_name = RwSignal::new(
                                                String::from("ITM_OTM"),
                                            );
                                            let file_content = RwSignal::new(
                                                String::from("Test content"),
                                            );
                                            let no_data_message = RwSignal::new(
                                                String::from("No Trade History Available"),
                                            );
                                            let row_slice = RwSignal::new(25);
                                            if selected_page.get() == "All".to_string() {
                                                trade_table.set(data.get().extract());
                                                file_content
                                                    .set(
                                                        data.get().extract_csv_by_trade_status(Option::None, false),
                                                    );
                                                view! {
                                                    <DataTable
                                                        headers=headersource
                                                        key_to_display_in_mobile
                                                        keys=datakeysource
                                                        data=json_value
                                                        color=color_vec
                                                        currency=currency_vec
                                                        row_slice=row_slice
                                                        hasdownload=true
                                                        hasfilter=true
                                                        nodatamessage=no_data_message
                                                        file_content=file_content
                                                        file_name=download_file_name
                                                        number_keys=number_keys
                                                        haspageslenght=true
                                                    />
                                                }
                                            } else if selected_page.get() == "Option".to_string() {
                                                trade_table
                                                    .set(data.get().extract_by_type(String::from("Option")));
                                                no_data_message
                                                    .set(String::from("No Option Trade History Available"));
                                                file_content
                                                    .set(
                                                        data.get().extract_csv_by_kind(Some(String::from("Option"))),
                                                    );
                                                view! {
                                                    <DataTable
                                                        headers=headersource
                                                        key_to_display_in_mobile
                                                        keys=datakeysource
                                                        data=json_value
                                                        color=color_vec
                                                        currency=currency_vec
                                                        row_slice=row_slice
                                                        hasdownload=true
                                                        hasfilter=true
                                                        nodatamessage=no_data_message
                                                        file_content=file_content
                                                        file_name=download_file_name
                                                        number_keys=number_keys
                                                        haspageslenght=true
                                                    />
                                                }
                                            } else if selected_page.get() == "Spot".to_string() {
                                                trade_table
                                                    .set(data.get().extract_by_type(String::from("Spot")));
                                                no_data_message
                                                    .set(String::from("No Spot Trade History Available"));
                                                file_content
                                                    .set(
                                                        data.get().extract_csv_by_kind(Some(String::from("Spot"))),
                                                    );
                                                view! {
                                                    <DataTable
                                                        headers=headersource
                                                        key_to_display_in_mobile
                                                        keys=datakeysource
                                                        data=json_value
                                                        color=color_vec
                                                        currency=currency_vec
                                                        row_slice=row_slice
                                                        hasdownload=true
                                                        hasfilter=true
                                                        nodatamessage=no_data_message
                                                        file_content=file_content
                                                        file_name=download_file_name
                                                        number_keys=number_keys
                                                        haspageslenght=true
                                                    />
                                                }
                                            } else if selected_page.get() == "Perpetual Futures".to_string(){
                                                let text = RwSignal::new(String::from("Perpetual Future is not yet supported."));
                                                view! {
                                                    <DefaultNone text = text/>
                                                }
                                            } else {
                                                let text = RwSignal::new(String::from("The selected page does not exist."));
                                                view! {
                                                    <div class="items-center mt-5">
                                                        <DefaultNone text = text/>
                                                    </div>
                                                }.into_view()
                                            }
                                        }
                                    }
                                    </Suspense>
                                }.into_view()
                            }
                        })
                    }
                }
            </Suspense>
        </div>
    }
}
