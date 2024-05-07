use std::collections::HashMap;

use leptos::*;

use crate::{commons::{models::trade::{sort, ExtractedTrade}, services::coinbase::get_spot_price}, components::{arrow_down::ArrowDown, arrow_up::ArrowUp, component_size::ComponentSize, component_type::ComponentType, data_table_header::GenericDataTableHeader, icons::CurrencyIcon, loading_spinners::Spinners, menu_button::MenuButton}, features::trades::positions::services::get_positions_with_live_pnl};

#[allow(non_snake_case)]
#[component]
pub fn Positions() -> impl IntoView {

    let btc_spot_price_resource = create_local_resource(|| (), move |_| get_spot_price(String::from("BTC-USD")));
    let eth_spot_price_resource = create_local_resource(|| (), move |_| get_spot_price(String::from("ETH-USD")));

    view! {
        <div class = "p-4">
            <div class="pb-5 ml-2 text-xl font-bold text-white">
                <span>Positions</span>
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
            //         positions_resource.and_then(|c| {
            //             positions.set(c.clone());
            //             if positions.get().data.len() <= 0 {
            //                 view!{
            //                     <div class = "items-center mt-5">
            //                         <div class = "p-5">
            //                             <span class="opacity-50 font-extralight">No trade record found</span>
            //                         </div>
            //                     </div>
            //                 }.into_view()
            //             } else {
            //                 view!{
            //                     <PositionsPage data = positions/>
            //                 }.into_view()
            //             }
            //         })

            //     }
            // }
            // </Suspense>
            <Suspense
                fallback = move || view! {
                        <div class = "items-center mt-5">
                            <div class = "flex justify-center ">
                                <Spinners size = ComponentSize::SMALL _type = ComponentType::SUCCESS />
                            </div>
                        </div>
                    }
            >
            {
                move || {
                    btc_spot_price_resource.and_then(|t| {
                        let spot = t.data.amount.parse::<f64>().unwrap_or_default();
                        view! {
                            <PerCurrencyPosition spot = spot pair_name =  String::from("BTC/USD") currency_name = String::from("BTC") counter_party= String::from("JABRA") />
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
                                <Spinners size = ComponentSize::SMALL _type = ComponentType::SUCCESS />
                            </div>
                        </div>
                    }
            >
            {
                move || {
                    eth_spot_price_resource.and_then(|t| {
                        let spot = t.data.amount.parse::<f64>().unwrap_or_default();
                        view! {
                            <PerCurrencyPosition spot = spot pair_name =  String::from("ETH/USD") currency_name = String::from("ETH") counter_party= String::from("JABRA") />
                        }
                    })
                }
            }
            </Suspense>
        </div>
    }
}

/// Component for Perpetual Futures Positions Tab.
/// Calls the data table with `Perpetual Futures` filter (currently not implemented).

#[allow(non_snake_case)]
#[component]
pub fn PerpetualFuturePositions() -> impl IntoView {
    view! {
        <div class = "p-5">
            <span class = "opacity-50 font-extralight">Perpetual Future is not currently supported.</span>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn PerCurrencyPosition(
    spot: f64,
    pair_name: String,
    currency_name: String,
    counter_party: String,
) -> impl IntoView {
    let pair = RwSignal::new(pair_name);
    let pair_display_name = move || pair.get().replace("/", "-");
    let trade_history_resource = create_local_resource(|| (), move |_| {
        get_positions_with_live_pnl(pair.get(), spot, counter_party.clone())
    });
    let selected_page = RwSignal::new("All".to_string());
    let all_data = RwSignal::new(HashMap::<String, Vec<ExtractedTrade>>::default());
    let option_data = RwSignal::new(HashMap::<String, Vec<ExtractedTrade>>::default());
    let currency = RwSignal::new(currency_name);
    let show = RwSignal::new(false);
    view! {
        <Suspense
            fallback = move || view! {
                    <div class = "items-center mt-5">
                        <div class = "flex justify-center ">
                            <Spinners size = ComponentSize::SMALL _type = ComponentType::SUCCESS />
                        </div>
                    </div>
                }
        >
        {
            move || {
                trade_history_resource.and_then(|t| {
                    show.set(t.data.len() > 0);
                    view!{
                        <div class = "flex justify-between pb-2">
                            <div class = "flex items-center gap-4 text-center">
                                <CurrencyIcon name = currency.get() class = "w-7 h-7".to_string() />
                                <div class = "text-base font-bold">{pair_display_name()}</div>
                                <div class = "text-xs text-gray-500">{format!("{} Positions", t.data.len())}</div>
                            </div>
                            <Show when = move  || show.get()>
                                <div class = "flex justify-between">
                                    <div class = "flex flex-wrap justify-start gap-4 flex-0">
                                    {
                                        let page_keys = vec![String::from("All"), String::from("Option"), String::from("Perpetual Futures")];
                                        page_keys.into_iter().map(|k| {
                                            view! {
                                                <MenuButton selected_page = selected_page page = k.clone() name = k.clone() />
                                            }
                                        }).collect_view()
                                    }
                                    </div>
                                </div>
                            </Show>
                        </div>
                        {
                            if selected_page.get() == "All".to_string() {
                                // filtered_trade_table.set(t.extract_group());
                                all_data.set(t.extract_and_group_by_date());
                                    view! {
                                        <FilteredByDateDataTable data = all_data />
                                    }
                            } else if selected_page.get() == "Option".to_string() {
                                // option_filtered_trade_table.set(t.extract_group_by_type(String::from("Option")));
                                option_data.set(t.extract_by_instrument_kind_and_group_by_date(String::from("Option")));
                                    view! {
                                        <FilteredByDateDataTable data = option_data />
                                    }
                            } else if selected_page.get() == "Perpetual Futures".to_string() {
                                view! {
                                    <PerpetualFuturePositions/>
                                }
                            } else {
                                view! {
                                    <div class = "mt-5"><span class = "opacity-50 font-extralight">The selected page does not exist.</span></div>
                                }.into_view()
                            }
                        }
                    }
                })
            }
        }
        </Suspense>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn FilteredByDateDataTable(
    data: RwSignal<HashMap<String, Vec<ExtractedTrade>>>,
) -> impl IntoView {
    let selected_header = RwSignal::new(String::from(""));
    let sort_asc = RwSignal::new(true);
    let data_len = move || data.get().len();
    // let selected_data = move || sort_map(data.get(), sort_asc.get(), selected_header.get());

    view! {
        <Show when = move || {data_len() > 0}>
        <div class = "px-4 overflow-auto">
            <table class = "table table-xs table-zebra-zebra">
            <thead>
                    <tr class = "text-sm uppercase bg-base-100 text-success hidden px924:contents">
                    {
                        let header_keys = vec![String::from("Market"), String::from("Side"), String::from("Kind"), String::from("Size"), String::from("Price"), String::from("Date Created"), String::from("PnL"), String::from("PnL Percentage"), String::from("Last Updated")];
                        header_keys.into_iter().map(|k| {
                            view! {
                                <GenericDataTableHeader header_selector = selected_header sorter = k.clone() sort_direction = sort_asc name = k.clone() />
                            }
                        }).collect_view()
                    }
                    </tr>
                </thead>
                <tbody>
                     {
                         move || {
                             data.get().into_iter().map(|(k, v)| {
                                 let hide_per_date = RwSignal::new(true);
                                 let tr = move || sort(v.clone(), sort_asc.get(), selected_header.get());
                                 view! {
                                     <tr class = "bg-base-100">
                                         <td colspan = "9">
                                             <button class = "flex justify-start w-full gap-2 border-l-2 rounded-none btn btn-ghost btn-md bg-base-100 border-l-success" on:click = move |_| hide_per_date.update(|c| *c = !*c)>
                                                 <span class = "font-light">{k.clone()}</span>
                                                 <div class = "flex justify-end flex-1">
                                                     <Show when = move || hide_per_date.get() fallback = move || view! {<ArrowDown />}>
                                                         <ArrowUp />
                                                     </Show>
                                                 </div>
                                             </button>
                                         </td>
                                     </tr>
                                     {
                                         move || {
                                             tr().iter().map(|d| {
                                                 let class_side = if d.side == "Sell" {"text-success"} else {"text-error"};

                                                 let amount_size: f64 = d.size.clone().parse::<f64>().unwrap();
                                                 let class_size = if amount_size >= 0.0 {"text-success"} else {"text-error"};
                                                 let value_size = if amount_size >= 0.0 {format!("+{}",d.size)} else {format!("{}", d.size)};

                                                 let amount_price: f64 = d.price.clone().parse::<f64>().unwrap();
                                                 let class_price = if amount_price >= 0.0 {"text-success"} else {"text-error"};
                                                 let value_price = if amount_price >= 0.0 {format!("+{}",d.price)} else {format!("{}", d.price)};

                                                 let live_pnl: f64 = d.live_pnl.clone().parse::<f64>().unwrap();
                                                 let class_live_pnl = if live_pnl >= 0.0 {"text-success"} else {"text-error"};
                                                 let value_live_pnl = if live_pnl >= 0.0 {format!("+{}",d.live_pnl)} else {format!("{}", d.live_pnl)};

                                                 let live_pnl_pctg: f64 = d.live_pnl.clone().parse::<f64>().unwrap();
                                                 let class_live_pnl_pctg = if live_pnl_pctg >= 0.0 {"text-success"} else {"text-error"};
                                                 let value_live_pnl_pctg = if live_pnl_pctg >= 0.0 {format!("+{}",d.pnl_percentage)} else {format!("{}", d.pnl_percentage)};
                                                 view! {
                                                    <tr class = "uppercase border-b border-b-base-100" prop:hidden = move || {hide_per_date.get()}>
                                                        <div class="hidden px924:contents">
                                                            <td>{d.market.clone()}</td>
                                                            <td class = {class_side}>{d.side.clone()}</td>
                                                            <td>{d.kind.clone()}</td>
                                                            <td class = {class_size}>{value_size.clone()}</td>
                                                            <td><span class = {class_price}>{value_price.clone()}</span><span class = "text-xs opacity-50 font-extralight">{format!(" {}", d.premium_ccy)}</span></td>
                                                            // <td><span>{d.live_pnl.clone()}</span><span class = "text-xs opacity-50 font-extralight">{format!(" {}", d.live_pnl_ccy.clone())}</span></td>
                                                            <td>{d.date_created.clone()}</td>
                                                            <td><span class = {class_live_pnl}>{value_live_pnl.clone()}</span><span class = "text-xs opacity-50 font-extralight">USD</span></td>
                                                            <td><span class = {class_live_pnl_pctg}>{value_live_pnl_pctg.clone()}</span><span class = "text-xs opacity-50 font-extralight">"%"</span></td>
                                                            <td>{d.last_updated.clone()}</td>
                                                        </div>
                                                    </tr>

                                                    // ================ MOBILE VIEW ==================
                                                    
                                                    // ================ MOBILE VIEW ==================

                                                    // ================ MOBILE VIEW ==================

                                                    
                                                    <tr prop:hidden = move || hide_per_date.get() class = "uppercase">
                                                        <td colspan = "9" class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">MARKET: </span><span class = "text-xs px924:hidden">{format!(" {}", d.market.clone())}</span></td>
                                                        // <td colspan = "9" class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">MARKET: </span><span class="px924:hidden" class = {if stringval != "- -" {style.clone()}else{None}}>{stringval}</span><span class = "text-xs px924:hidden">{format!(" {}",currencytxt)}</span></td>
                                                    </tr>
                                                    <tr prop:hidden = move || hide_per_date.get() class = "uppercase">
                                                        <td colspan = "9" class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">SIDE: </span><span class = {format!("px924:hidden {}", class_side)}>{format!(" {}", d.side.clone())}</span></td>
                                                    </tr>
                                                    <tr prop:hidden = move || hide_per_date.get() class = "uppercase">
                                                        <td colspan = "9" class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">KIND: </span><span class = "text-xs px924:hidden">{format!(" {}", d.kind.clone())}</span></td>
                                                    </tr>
                                                    <tr prop:hidden = move || hide_per_date.get() class = "uppercase">
                                                        <td colspan = "9" class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">SIZE: </span><span class = {format!("px924:hidden {}", class_size)}>{value_size}</span></td>
                                                    </tr>
                                                    <tr prop:hidden = move || hide_per_date.get() class = "uppercase">
                                                        <td colspan = "9" class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">PRICE: </span><span class = {format!("px924:hidden {}", class_price)}>{value_price}</span><span class = "text-xs opacity-50 font-extralight">{format!(" {}", d.premium_ccy)}</span></td>
                                                    </tr>
                                                    <tr prop:hidden = move || hide_per_date.get() class = "uppercase">
                                                        <td colspan = "9" class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">DATE CREATED: </span><span class = "text-xs px924:hidden">{format!(" {}", d.date_created.clone())}</span></td>
                                                    </tr>
                                                    <tr prop:hidden = move || hide_per_date.get() class = "uppercase">
                                                        <td colspan = "9" class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">PNL: </span><span class = {format!("px924:hidden {}", class_live_pnl)}>{value_live_pnl}</span><span class = "text-xs opacity-50 font-extralight">USD</span></td>
                                                    </tr>
                                                    <tr prop:hidden = move || hide_per_date.get() class = "uppercase">
                                                        <td colspan = "9" class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">PNL PERCENTAGE: </span><span class = {format!("px924:hidden {}", class_live_pnl_pctg)}>{value_live_pnl_pctg}</span><span class = "text-xs opacity-50 font-extralight">"%"</span></td>
                                                    </tr>
                                                    <tr prop:hidden = move || hide_per_date.get() class = "uppercase">
                                                        <td colspan = "9" class="px924:hidden"><span class="mr-2 text-sm text-success px924:hidden">LAST UPDATED: </span><span class = "text-xs px924:hidden">{format!(" {}", d.last_updated.clone())}</span></td>
                                                    </tr>

                                                    {
                                                        if tr().len() <= 1 {
                                                            view! {
                                                                <div></div>
                                                            }
                                                        } else {
                                                            view! {
                                                                <div class="divider divider-ghost px924:hidden" class=("hidden", move || hide_per_date.get())></div>
                                                            }
                                                        }
                                                    }
                                                 }
                                             }).collect_view()
                                         }
                                     }
                                 }
                             }).collect_view()
                         }
                     }
                </tbody>
            </table>
        </div>
        </Show>
    }
}
