use leptos::*;

use crate::{commons::{models::counterparty::{CounterParty, GetCounterPartiesResponse}, services::counterparty::get_counter_parties}, components::{component_size::ComponentSize, component_type::ComponentType, loading_spinners::Spinners}, features::counterparties::{account_overview::AccountOverviewPage, loans::CounterPartyLoansPage, positions::CounterPartyPositionsPage, quotes::CounterPartyQuotesPage, trade_history::CounterPartyTradeHistoryPage, transfers::CounterPartyTransfersPage}};

#[allow(non_snake_case)]
#[component]
pub fn CounterParties() -> impl IntoView {
    let counterparties = RwSignal::new(Vec::<CounterParty>::default());
    let counterparties_resource: Resource<
        (),
        Result<GetCounterPartiesResponse, ServerFnError>,
    > = create_local_resource(|| (), move |_| get_counter_parties());

    view!{
        <Suspense
            fallback = move || view! {
                <div class = "flex w-full h-screen items-center justify-center">
                    <Spinners size=ComponentSize::SMALL _type=ComponentType::SUCCESS />
                </div>
            }
        >
            {
                move || {
                    counterparties_resource.and_then(|c| {
                        counterparties.set(c.data.clone());
                        if counterparties.get().len() == 0 {
                            view!{
                                <div></div>
                            }.into_view()
                        }else{
                            view!{
                                <CounterPartyPage counterparties = counterparties/>
                            }.into_view()
                        }
                    })
                }
            }
        </Suspense>
    }
}

/// Component for CounterParty Page.
/// Sets the counterparty and pages for [`CounterPartyPageManager`].

#[allow(non_snake_case)]
#[component]
pub fn CounterPartyPage(counterparties: RwSignal<Vec<CounterParty>>) -> impl IntoView {
    let counterparty_oper = vec![
        String::from("Account Overview"),
        String::from("Positions"),
        String::from("Quotes"),
        String::from("Loans"),
        String::from("Trade History"),
        String::from("Transfers"),
    ];

    let counterparty = RwSignal::new(String::from(""));
    let oper = RwSignal::new(counterparty_oper.first().unwrap().clone());

    view! {
        <div class="p-4">
            <div class = "flex px924:flex-row flex-col gap-2 justify-between bg-base-300 bg-opacity-50 p-4 rounded-xl">
                <div class="pb-5 ml-2 text-xl font-bold">
                    <span class = "text-white">Counterparties - </span><span class = "opacity-50 font-semibold">{move || oper.get()}</span>
                </div>
                <div class = "flex flex-1 justify-between px924:justify-end gap-3">
                    <div class = "join flex-0">
                        <button class = "hidden px924:inline-flex join-item btn btn-outline pointer-events-none btn-sm bg-base-100 border-gray-800 rounded-l-lg text-opacity-70">COUNTERPARTY</button>
                        <div class="flex-col px924:form-control">
                            <div class="px924:hidden label">
                                <span class="label-text">Select CounterParty</span>
                            </div>
                            <select class = "select-sm text-xs rounded-lg px924:rounded-none hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md rounded-r-lg" prop:value = counterparty on:change = move |event| {
                                let val: String = event_target_value(&event);
                                counterparty.set(val);
                            }>
                                {
                                    move || {
                                        counterparty.set(counterparties.get().first().unwrap().ticker.clone());
                                        counterparties.get().into_iter().map(|cp| {
                                            view! {
                                                <option value = {cp.ticker}>{cp.name}</option>
                                            }
                                        }).collect_view()
                                    }
                                }
                            </select>
                        </div>
                    </div>
                    <div class = "join flex-0">
                        <button class = "hidden px924:inline-flex join-item btn btn-outline pointer-events-none btn-sm bg-base-100 border-gray-800 rounded-l-lg text-opacity-70">VIEW</button>
                        <div class="flex-col px924:form-control">
                            <div class="px924:hidden label">
                                <span class="label-text">Select View</span>
                            </div>
                            <select class = "select-sm text-xs rounded-lg px924:rounded-none hover:shadow-sm hover:shadow-success bg-base-100 border-gray-800 shadow-md rounded-r-lg" on:change = move |event| {
                                let val: String = event_target_value(&event);
                                oper.set(val);
                            }>
                                {
                                    counterparty_oper.iter().map(|oper| {
                                        view! {
                                            <option value = {oper}>{oper}</option>
                                        }
                                    }).collect_view()
                                }
                            </select>
                        </div>
                    </div>
                </div>
            </div>
            <div>
                <CounterPartyPageManager counterparty = counterparty oper = oper/>
            </div>
        </div>
    }
}

/// Component for CounterParty Page Manager.
/// Handles the page and changes view based on the input.

#[allow(non_snake_case)]
#[component]
pub fn CounterPartyPageManager(
    counterparty: RwSignal<String>,
    oper: RwSignal<String>,
) -> impl IntoView {
    view! {
        <div>
            {
                move || {
                    match oper.get().as_str() {
                        "Account Overview" => view!{<AccountOverviewPage counterparty = counterparty/>},
                        "Positions" => view!{<CounterPartyPositionsPage counterparty = counterparty/>},
                        "Quotes" => view!{<CounterPartyQuotesPage counterparty = counterparty/>},
                        "Loans" => view!{<CounterPartyLoansPage counterparty = counterparty/>},
                        "Trade History" => view!{<CounterPartyTradeHistoryPage counterparty = counterparty/>},
                        "Transfers" => view!{<CounterPartyTransfersPage counterparty = counterparty/>},
                        _ => view! {<span class = "opacity-50">Page not available!</span>}.into_view(),
                    }
                }
            }
        </div>
    }
}