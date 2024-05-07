use leptos::*;
use serde_json::json;
use crate::commons::models::quote::ExtractedQuoteOption;
use crate::commons::models::quote::QuoteOptionHistory;
use crate::components::data_table_local::DataTable;
use crate::components::menu_button::MenuButton;
use crate::features::counterparties::services::get_quote_history;

/// Component for CounterParty Quotes Page.
/// Has the actual view for Quotes page.

#[allow(non_snake_case)]
#[component]
pub fn CounterPartyQuotesPage(counterparty: RwSignal<String>) -> impl IntoView {
    let quote_history_resource: Resource<String, Result<QuoteOptionHistory, ServerFnError>> =
        create_local_resource(counterparty, move |e| get_quote_history(e));
    let quotes_history_table = RwSignal::new(Vec::<ExtractedQuoteOption>::default());
    let selected_page = RwSignal::new("All".to_string());
    let json_value = Signal::derive(move || {
        quotes_history_table
            .get()
            .into_iter()
            .map(serde_json::to_value)
            .collect::<Result<Vec<serde_json::Value>, _>>()
            .expect("Failed to serialize to JSON")
    });
    view! {
        <div  class = "py-4">
            <div class = "flex justify-between">
                <div class = "flex justify-start gap-4 flex-0">
                {
                    let page_keys = vec![String::from("All"), String::from("Quote Option"), String::from("Perpetual Futures")];
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
                if let Some(data) = quote_history_resource.and_then(|e| {e.clone()}) {
                    match data {
                        Ok(t) => {
                            let color_vec = RwSignal::new(vec!(json!({"side":[{"key":"sell", "style":"text-error"},{"key":"buy", "style": "text-success"}],"status":[{"key":"rejected", "style":"text-error"},{"key":"approved", "style": "text-success"}],"price":[{"key":"-", "style":"text-error"},{"key":"", "style": "text-success"}],"size":[{"key":"-", "style":"text-error"},{"key":"", "style": "text-success"}]})));
                            // color_vec.push(json!({"side":[{"key":"sell", "style":"text-error"},{"key":"buy", "style": "text-success"}],"status":[{"key":"rejected", "style":"text-error"},{"key":"approved", "style": "text-success"}],"price":[{"key":"-", "style":"text-error"},{"key":"", "style": "text-success"}],"size":[{"key":"-", "style":"text-error"},{"key":"", "style": "text-success"}]}));
                            let headersource= RwSignal::new(vec![String::from("Market"), String::from("Status"), String::from("Side"), String::from("Kind"), String::from("Type"), String::from("Size"), String::from("Price"), String::from("Date Created")]);
                            let datakeysource= RwSignal::new(vec![String::from("market"), String::from("status"), String::from("side"), String::from("kind"), String::from("trans_type"), String::from("size"), String::from("price"), String::from("date_created")]);
                            let key_to_display_in_mobile = RwSignal::new(String::from("market"));
                            let number_keys= RwSignal::new(vec![String::from("side"), String::from("size"), String::from("price")]);
                            let currency_vec = RwSignal::new(vec![json!({"key": "price", "value": "premium_ccy"})]);
                            let row_slice = RwSignal::new(15);
                            let no_data_message = RwSignal::new(String::from("No Quote History Available"));
                            let download_file_name = RwSignal::new(String::from("Quotes"));
                            let file_content = RwSignal::new(String::from("Test content"));
                            if selected_page.get() == "All".to_string() {
                                quotes_history_table.set(t.extract());
                                file_content.set(t.extract_csv_by_quote_status(Option::None, false));
                                view! {
                                    <DataTable headers = headersource
                                        key_to_display_in_mobile
                                        keys = datakeysource
                                        data = json_value
                                        color=color_vec
                                        currency = currency_vec
                                        row_slice = row_slice
                                        hasfilter = true
                                        nodatamessage = no_data_message
                                        hasdownload = true
                                        file_content = file_content
                                        file_name = download_file_name
                                        number_keys = number_keys
                                        haspageslenght=true
                                    />
                                }
                            } else if selected_page.get() == "Quote Option".to_string() {
                                quotes_history_table.set(t.extract());
                                file_content.set(t.extract_csv_by_quote_status(Option::None, false));
                                no_data_message.set(String::from("No Quote Options History Available"));
                                view! {
                                    <DataTable headers = headersource
                                        key_to_display_in_mobile
                                        keys = datakeysource
                                        data = json_value
                                        color=color_vec
                                        currency = currency_vec
                                        row_slice = row_slice
                                        hasfilter = true
                                        nodatamessage = no_data_message
                                        hasdownload = true
                                        file_content = file_content
                                        file_name = download_file_name
                                        number_keys = number_keys
                                        haspageslenght=true
                                    />
                                }
                            } else if selected_page.get() == "Perpetual Futures".to_string() {
                                view! {
                                    <div class = "p-5">
                                        <span class = "opacity-50 font-extralight">Perpetual Future is not currently supported.</span>
                                    </div>
                                }.into_view()
                            } else {
                                view! {
                                    <div class = "mt-5"><span class = "opacity-50 font-extralight">The selected page does not exist.</span></div>
                                }.into_view()
                            }

                        },
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
                            <span class = "opacity-50 font-extralight">No quotes record found</span>
                        </div>
                    }.into_view()
                }
            }
        }
        </Suspense>
    }
}