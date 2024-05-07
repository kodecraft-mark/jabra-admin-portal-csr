use leptos::*;
use crate::{commons::services::coinbase::get_spot_price, features::trades::positions::page::PerCurrencyPosition};



/// Component for CounterParty Positions Page.
/// Has the actual view for Positions page.

#[allow(non_snake_case)]
#[component]
pub fn CounterPartyPositionsPage(counterparty: RwSignal<String>) -> impl IntoView {
    let btc_spot_price_resource = create_local_resource(|| (), move |_| get_spot_price(String::from("BTC-USD")));
    let eth_spot_price_resource = create_local_resource(|| (), move |_| get_spot_price(String::from("ETH-USD")));

    // let trade_history_resource: Resource<String, Result<TradeHistory, ServerFnError>> =
    //     Resource::new(counterparty, move |e| get_positions(e));
    // let filtered_trade_table =
    //     RwSignal::new(HashMap::<String, HashMap<String, Vec<ExtractedTrade>>>::default());
    // let option_filtered_trade_table =
    //     RwSignal::new(HashMap::<String, HashMap<String, Vec<ExtractedTrade>>>::default());
    // let selected_page = RwSignal::new("All".to_string());

    view! {
        <div  class = "py-4">
            // <div class = "flex justify-between">
            //     <div class = "flex justify-start gap-4 flex-0">
            //     {
            //         let page_keys = vec![String::from("All"), String::from("Option"), String::from("Perpetual Futures")];
            //         page_keys.into_iter().map(|k| {
            //             view! {
            //                 <MenuButton selected_page = selected_page page = k.clone() name = k.clone() />
            //             }
            //         }).collect_view()
            //     }
            //     </div>
            // </div>
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
                    btc_spot_price_resource.and_then(|t| {
                        let spot = t.data.amount.parse::<f64>().unwrap_or_default();
                        view! {
                            <PerCurrencyPosition spot = spot pair_name =  String::from("BTC/USD") currency_name = String::from("BTC") counter_party= counterparty.get() />
                        }
                    })
                }
            }
            </Suspense>
            <div class="px-4 opacity-50 divider divider-neutral font-extralight"></div>
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
                    eth_spot_price_resource.and_then(|t| {
                        let spot = t.data.amount.parse::<f64>().unwrap_or_default();
                        view! {
                            <PerCurrencyPosition spot = spot pair_name =  String::from("ETH/USD") currency_name = String::from("ETH") counter_party= counterparty.get() />
                        }
                    })
                }
            }
            </Suspense>
        </div>
        // <Suspense
        //     fallback = move || view! {
        //             <div class = "items-center mt-5">
        //                 <div class = "flex justify-center ">
        //                 <span class="loading loading-bars loading-sm text-success"></span>
        //                 </div>
        //             </div>
        //         }
        // >
        // {
        //     move || {
        //         if let Some(data) = trade_history_resource.and_then(|e| {e.clone()}) {
        //             match data {
        //                 Ok(t) => {
        //                     if selected_page.get() == "All".to_string() {
        //                         filtered_trade_table.set(t.extract_group());
        //                             view! {
        //                                 <AllPositions trade_data = filtered_trade_table />
        //                             }
        //                     } else if selected_page.get() == "Option".to_string() {
        //                         option_filtered_trade_table.set(t.extract_group_by_type(String::from("Option")));
        //                             view! {
        //                                 <OptionPositions trade_data = option_filtered_trade_table />
        //                             }
        //                     } else if selected_page.get() == "Perpetual Futures".to_string() {
        //                         view! {
        //                             <PerpetualFuturePositions/>
        //                         }
        //                     } else {
        //                         view! {
        //                             <div class = "mt-5"><span class = "opacity-50 font-extralight">The selected page does not exist.</span></div>
        //                         }.into_view()
        //                     }
        //                 }
        //                 Err(_) => view! {
        //                     <div class = "p-5">
        //                         <div class = "items-center mt-5">
        //                             <div class = "flex justify-center ">
        //                             <span class = "opacity-50 font-extralight">Cannot connect to server. Please refresh page.</span>
        //                             </div>
        //                         </div>
        //                     </div>
        //                 }.into_view(),
        //             }
        //         } else {
        //             view! {
        //                 <div class = "p-5">
        //                     <span class = "opacity-50 font-extralight">No positions record found</span>
        //                 </div>
        //             }.into_view()
        //         }
        //     }
        // }
        // </Suspense>
    }.into_view()
}

// Component for All Positions Tab.
// Calls the data table with `All` positions filter.

// #[allow(non_snake_case)]
// #[component]
// pub fn AllPositions(
//     trade_data: RwSignal<HashMap<String, HashMap<String, Vec<ExtractedTrade>>>>,
// ) -> impl IntoView {
//     move || {
//         if trade_data.get().len() > 0 {
//             view! {
//                 <div class = "rounded-lg">
//                     <FilteredDataTable data = trade_data />
//                 </div>
//             }
//         } else {
//             view! {
//                 <div class = "p-5">
//                     <span class = "opacity-50 font-extralight">No positions record found</span>
//                 </div>
//             }
//         }
//     }
// }

// /// Component for Option Positions Tab.
// /// Calls the data table with `Option` filter.

// #[allow(non_snake_case)]
// #[component]
// pub fn OptionPositions(
//     trade_data: RwSignal<HashMap<String, HashMap<String, Vec<ExtractedTrade>>>>,
// ) -> impl IntoView {
//     move || {
//         if trade_data.get().len() > 0 {
//             view! {
//                 <div class = "rounded-lg">
//                     <FilteredDataTable data = trade_data />
//                 </div>
//             }
//         } else {
//             view! {
//                 <div class = "p-5">
//                     <span class = "opacity-50 font-extralight">No option positions record found</span>
//                 </div>
//             }
//         }
//     }
// }

// /// Component for Perpetual Futures Positions Tab.
// /// Calls the data table with `Perpetual Futures` filter (currently not implemented).

// #[allow(non_snake_case)]
// #[component]
// pub fn PerpetualFuturePositions() -> impl IntoView {
//     view! {
//         <div class = "p-5">
//             <span class = "opacity-50 font-extralight">Perpetual Future is not currently supported.</span>
//         </div>
//     }
// }

// /// Component for Filtered Data Table.
// /// Has the actual data table.

// #[allow(non_snake_case)]
// #[component]
// pub fn FilteredDataTable(
//     data: RwSignal<HashMap<String, HashMap<String, Vec<ExtractedTrade>>>>,
// ) -> impl IntoView {
//     let selected_header = RwSignal::new(String::from(""));
//     let sort_asc = RwSignal::new(true);
//     // let selected_data = move || sort_map(data.get(), sort_asc.get(), selected_header.get());

//     view! {
//         <div class = "p-4">
//             <table class = "table table-xs">
//             <thead>
//                     <tr class = "text-sm uppercase bg-base-100 text-success">
//                     {
//                         let header_keys = vec![String::from("Market"), String::from("Side"), String::from("Kind"), String::from("Size"), String::from("Price"), String::from("Date Created")];
//                         header_keys.into_iter().map(|k| {
//                             view! {
//                                 <GenericDataTableHeader header_selector = selected_header sorter = k.clone() sort_direction = sort_asc name = k.clone() />
//                             }
//                         }).collect_view()
//                     }
//                     </tr>
//                 </thead>
//                 <tbody>
//                     {
//                         move || {
//                             data.get().into_iter().map(|(key, value)| {
//                                 let per_currency_data = RwSignal::new(value);
//                                 // let val = value.clone();
//                                 let hide_per_currency = RwSignal::new(true);
//                                 let currency = key.clone();
//                                 let positions_size = move || per_currency_data.get().values().flat_map(|vec| vec.iter()).count();
//                                 view! {
//                                     <tr>
//                                     <td colspan = "6">
//                                         {
//                                             view! {
//                                                 <button class = "flex justify-start w-1/5 gap-2 border-l-2 rounded-none btn btn-ghost btn-md bg-base-100 border-l-success" on:click = move |_| hide_per_currency.update(|c| *c = !*c) >
//                                                 <CurrencyIcon name = currency.clone() class = "w-7 h-7".to_string() />
//                                                 <div class = "grid grid-cols-1">
//                                                     <div class = "text-base font-bold">{currency.clone()}</div>
//                                                     <div class = "text-xs text-gray-500">{positions_size()} Positions</div>
//                                                 </div>
//                                                 <div class = "flex justify-end flex-1">
//                                                     <Show when = move || hide_per_currency.get() fallback = move || view! {<ArrowDown />}>
//                                                         <ArrowUp />
//                                                     </Show>
//                                                 </div>
//                                                 </button>
//                                             }.into_view()
//                                         }
//                                     </td>
//                                     </tr>
//                                         {
//                                             move || {
//                                                 per_currency_data.get().into_iter().map(|(k, v)| {
//                                                     let hide_per_date = RwSignal::new(true);
//                                                     let tr = move || sort(v.clone(), sort_asc.get(), selected_header.get());
//                                                     view! {
//                                                         <tr prop:hidden = move || hide_per_currency.get() >
//                                                             <td colspan = "6">
//                                                                 <button class = "flex justify-between w-1/5 gap-2 border-l-2 rounded-none btn btn-ghost btn-sm bg-base-100 border-l-base-300" on:click = move |_| hide_per_date.update(|c| *c = !*c)>
//                                                                     <span class = "font-light">{k.clone()}</span>
//                                                                     <div class = "flex justify-end flex-1">
//                                                                         <Show when = move || hide_per_date.get() fallback = move || view! {<ArrowDown />}>
//                                                                             <ArrowUp />
//                                                                         </Show>
//                                                                     </div>
//                                                                 </button>
//                                                             </td>
//                                                         </tr>
//                                                         {
//                                                             move || {
//                                                                 tr().iter().map(|d| {
//                                                                     let class_side = if d.side == "Sell" {"text-success"} else {"text-error"};

//                                                                     let amount_size: f64 = d.size.clone().parse::<f64>().unwrap();
//                                                                     let class_size = if amount_size >= 0.0 {"text-success"} else {"text-error"};
//                                                                     let value_size = if amount_size >= 0.0 {format!("+{}",d.size)} else {format!("{}", d.size)};

//                                                                     let amount_price: f64 = d.price.clone().parse::<f64>().unwrap();
//                                                                     let class_price = if amount_price >= 0.0 {"text-success"} else {"text-error"};
//                                                                     let value_price = if amount_price >= 0.0 {format!("+{}",d.price)} else {format!("{}", d.price)};
//                                                                     view! {
//                                                                         <tr class = "uppercase border-b hover:opacity-50 border-b-base-100" prop:hidden = move || {hide_per_date.get() || hide_per_currency.get()}>
//                                                                             <td>{d.market.clone()}</td>
//                                                                             <td class = {class_side}>{d.side.clone()}</td>
//                                                                             <td>{d.kind.clone()}</td>
//                                                                             <td class = {class_size}>{value_size}</td>
//                                                                             <td><span class = {class_price}>{value_price}</span><span class = "text-xs opacity-50 font-extralight">{format!(" {}", d.premium_ccy)}</span></td>
//                                                                             // <td><span>{d.live_pnl.clone()}</span><span class = "text-xs opacity-50 font-extralight">{format!(" {}", d.live_pnl_ccy.clone())}</span></td>
//                                                                             <td>{d.date_created.clone()}</td>
//                                                                         </tr>
//                                                                     }
//                                                                 }).collect_view()
//                                                             }
//                                                         }
//                                                     }
//                                                 }).collect_view()
//                                             }
//                                         }
//                                 }
//                             }).collect_view()
//                         }
//                     }
//                 </tbody>
//             </table>
//         </div>
//     }
// }