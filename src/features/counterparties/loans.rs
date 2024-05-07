use leptos::*;

use crate::components::data_table_local::DataTable;

use crate::{commons::models::loan::{ExtractedLoan, GetLoanHistory}, components::menu_button::MenuButton};

use super::services::get_loans;

/// Component for CounterParty Loans Page.
/// Has the actual view for Loans Page.

#[allow(non_snake_case)]
#[component]
pub fn CounterPartyLoansPage(counterparty: RwSignal<String>) -> impl IntoView {
    let loans_resource: Resource<String, Result<GetLoanHistory, ServerFnError>> =
        create_local_resource(counterparty, move |e| get_loans(e));
    let loans = RwSignal::new(Vec::<ExtractedLoan>::default());
    let selected_page = RwSignal::new("Active Loans".to_string());
    let json_value = Signal::derive(move || {
        loans
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
                        let page_keys = vec![String::from("Active Loans"), String::from("Loan History")];
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
                    if let Some(data) = loans_resource.and_then(|e| {e.clone()}) {
                        match data {
                            Ok(t) => {
                                let headersource = RwSignal::new(vec![String::from("Currency Pair"), String::from("Loan To Value"), String::from("Interest Rate"), String::from("Reference Rate"), String::from("Base CCY Amount"), String::from("Initial Exchnage Amount"), String::from("Transaction Type"), String::from("Status"), String::from("Date Created")]);
                                let datakeysource = RwSignal::new(vec![String::from("currency_pair"), String::from("loan_to_value"), String::from("interest_rate"), String::from("reference_rate"),String::from("base_ccy_amount"), String::from("initial_exchange_amount"), String::from("transaction_type"), String::from("status"), String::from("date_created")]) ;
                                let key_to_display_in_mobile = RwSignal::new(String::from("date_created"));
                                let number_keys = RwSignal::new(vec![String::from("loan_to_value"), String::from("interest_rate"), String::from("reference_rate"),String::from("base_ccy_amount"), String::from("initial_exchange_amount")]) ;
                                let row_slice = RwSignal::new(15);
                                let no_data_message = RwSignal::new(String::from("No loan record found"));
                                let download_file_name = RwSignal::new(String::from("Loans"));
                                let file_content = RwSignal::new(String::from("Test content"));
                                if selected_page.get() == "Active Loans".to_string() {
                                    loans.set(t.extract_active_loan());
                                    file_content.set(t.extract_csv_by_loan_status(Option::None, false));
                                   view! {
                                        <DataTable
                                            headers = headersource
                                            keys = datakeysource
                                            key_to_display_in_mobile
                                            data = json_value
                                            row_slice = row_slice
                                            hasfilter = true
                                            hasdownload = true
                                            nodatamessage = no_data_message
                                            file_content = file_content
                                            file_name = download_file_name
                                            number_keys = number_keys
                                            haspageslenght=true
                                        />
                                   }
                                } else if selected_page.get() == "Loan History".to_string() {
                                    loans.set(t.extract());
                                    file_content.set(t.extract_csv_by_loan_status(Option::None, false));
                                   view! {
                                    <DataTable
                                            headers = headersource
                                            keys = datakeysource
                                            key_to_display_in_mobile
                                            data = json_value
                                            row_slice = row_slice
                                            hasfilter = true
                                            hasdownload = true
                                            nodatamessage = no_data_message
                                            file_content = file_content
                                            file_name = download_file_name
                                            number_keys = number_keys
                                            haspageslenght=true
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
                                <span class="opacity-50 font-extralight">No loan record found</span>
                            </div>
                        }.into_view()
                    }
                }
            }
            </Suspense>
        }.into_view()
}