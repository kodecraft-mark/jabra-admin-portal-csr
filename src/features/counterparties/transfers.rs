use leptos::*;

use crate::commons::models::wallet::WalletTransaction;
use crate::commons::models::wallet::WalletTransactionHistory;

use crate::commons::models::wallet::ExtractedWalletTransaction;
use crate::components::data_table_local::DataTable;
use crate::components::menu_button::MenuButton;

use super::services::fetch_transfers_data;

/// Component for CounterParty Transfers Page.
/// Has the actual view for the Transfers page.

#[allow(non_snake_case)]
#[component]
pub fn CounterPartyTransfersPage(counterparty: RwSignal<String>) -> impl IntoView {
    let transfers_resource: Resource<String, Result<WalletTransactionHistory, ServerFnError>> =
        create_local_resource(counterparty, move |e| fetch_transfers_data(e));
    let data_table = RwSignal::new(Vec::<ExtractedWalletTransaction>::default());
    let selected_page = RwSignal::new("All".to_string());
    let json_value = Signal::derive(move || {
        data_table
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
                        let page_keys = vec![String::from("All"), String::from("Deposit"), String::from("Withdrawal"), String::from("Transfer")];
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
                    if let Some(data) = transfers_resource.and_then(|tr| {tr.clone()}) {
                        match data {
                            Ok(t) => {
                                let headersource = RwSignal::new(vec![String::from("Action"), String::from("Amount"), String::from("Currency"), String::from("Fee Amount"), String::from("Description"), String::from("Time")]);
                                let keysource = RwSignal::new(vec![String::from("action"), String::from("amount"), String::from("currency"), String::from("fee_amount"),String::from("description"), String::from("time")]);
                                let number_keys = RwSignal::new(vec![String::from("action"), String::from("amount"), String::from("currency"), String::from("fee_amount"),String::from("description"), String::from("time")]);
                                let row_slice = RwSignal::new(15);
                                let no_data_message = RwSignal::new(String::from("No transaction record found"));
                                if selected_page.get() == "All".to_string() {
                                    data_table.set(t.extract("ALL".to_string()));

                                   view! {
                                       <DataTable headers=headersource
                                       keys=keysource
                                       data=json_value
                                       row_slice=row_slice
                                       hasfilter=true
                                       nodatamessage=no_data_message
                                       number_keys = number_keys
                                       haspageslenght=true
                                       />
                                   }
                                } else if selected_page.get() == "Deposit".to_string() {
                                    data_table.set(t.extract("DEPOSIT".to_string()));
                                    no_data_message.set(String::from("No deposit record found"));

                                   view! {
                                        <DataTable headers=headersource
                                        keys=keysource
                                        data=json_value
                                        row_slice=row_slice
                                        hasfilter=true
                                        nodatamessage=no_data_message
                                        number_keys = number_keys
                                        haspageslenght=true
                                        />
                                   }
                                } else if selected_page.get() == "Withdrawal".to_string() {
                                    data_table.set(t.extract("WITHDRAWAL".to_string()));
                                    no_data_message.set(String::from("No withdrawal record found"));

                                   view! {
                                        <DataTable headers=headersource
                                        keys=keysource
                                        data=json_value
                                        row_slice=row_slice
                                        hasfilter=true
                                        nodatamessage=no_data_message
                                        number_keys = number_keys
                                        haspageslenght=true
                                        />
                                   }
                                } else if selected_page.get() == "Transfer".to_string() {
                                    data_table.set(t.extract("TRANSFER".to_string()));
                                    no_data_message.set(String::from("No transfer record found"));

                                   view! {
                                        <DataTable headers=headersource
                                        keys=keysource
                                        data=json_value
                                        row_slice=row_slice
                                        hasfilter=true
                                        nodatamessage=no_data_message
                                        number_keys = number_keys
                                        // haspageslenght=true
                                        // pagesheight=RwSignal::new(String::from("max-h-[580px]"))
                                        />
                                   }
                                } else {
                                    view! {
                                        <div class = "items-center mt-5">
                                            <div class = "flex justify-center">
                                                <span class = "opacity-50 font-extralight">The selected page does not exist.</span>
                                            </div>
                                        </div>
                                    }.into_view()
                                }
                            },
                            Err(_) => view! {
                                <div class = "items-center mt-5">
                                    <div class = "flex justify-center ">
                                        <span class = "opacity-50 font-extralight">Cannot connect to server. Please refresh page.</span>
                                    </div>
                                </div>
                            }.into_view()
                        }
                    } else {
                        view! {
                            <div class = "p-5">
                                <span class = "opacity-50 font-extralight">No transfers record found</span>
                            </div>
                        }.into_view()
                    }
                }
            }
            </Suspense>
        }.into_view()
}