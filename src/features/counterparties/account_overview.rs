use leptos::*;

use crate::commons::services::currency::fetch_currencies;
use crate::commons::models::currency::CurrencyConfigurationResponse;
use crate::components::arrow_down::ArrowDown;
use crate::components::arrow_up::ArrowUp;
use crate::components::icons::CurrencyIcon;
use crate::utilities::number_util::format_currency_with_scale;
use crate::utilities::number_util::format_money;

use super::models::PortfolioOverviewResponse;
use super::models::PortfolioCurrency;
use super::services::fetch_overview_data;

/// Component for the Account Overview Page.
/// Has the actual view for the Account Overview.

#[allow(non_snake_case)]
#[component]
pub fn AccountOverviewPage(counterparty: RwSignal<String>) -> impl IntoView {
    let overview_resource: Resource<String, Result<PortfolioOverviewResponse, ServerFnError>> =
        create_local_resource(counterparty, move |e| fetch_overview_data(e));
    let equity = RwSignal::new(0f64);
    let negative_style = move |num: f64| {
        if num < 0.0 {
            String::from("text-error")
        } else {
            String::from("text-success")
        }
    };
    view! {
        <div class = "py-4">
            <Suspense
                fallback = move || view! {
                    <div class = "items-center mt-5">
                        <div class = " flex justify-center">
                            <crate::components::loading_spinners::Spinners size=crate::components::component_size::ComponentSize::SMALL _type=crate::components::component_type::ComponentType::SUCCESS />
                        </div>
                    </div>
                }
            >
            {
                move || {
                    overview_resource.and_then(|or| {
                        equity.set(or.total_equity.clone());
                        view! {
                            <div class = "bg-gray-500 p-4 rounded-lg bg-opacity-20">
                                <div class = "flex items-end">
                                    <div class="stats shadow mr-2">
                                        <div class="stat">
                                        <div class="stat-title font-light">Equity (USD)</div>
                                        <div class="stat-value bg-opacity-75"><span class = {negative_style(equity.get())}>{format_money(equity.get().to_string(), ",", 2)}</span></div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                            <div class = "mt-4">
                                <table class = "table table-sm table-zebra-zebra">
                                    <thead class = "bg-success bg-opacity-50 text-white uppercase text-base">
                                        <tr class = "hidden px924:contents">
                                            <th>Asset</th>
                                            <th>Balance</th>
                                            <th>Available Balance</th>
                                            <th>Equity</th>
                                            // <th>Live Pnl</th>
                                            <th>Exercised Balances</th>
                                            <th>Interest Payments</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <PortfolioCurrency portfolio_currency = or.currencies.clone()/>
                                    </tbody>
                                </table>
                            </div>
                        }
                    })
                }
            }
        </Suspense>
        </div>
    }.into_view()
}

/// Component for the Portfolio Currency.
/// Shows the Table body for the Portfolio Overview.
/// It takes a vector of [`PortfolioCurrency`] as input.

#[allow(non_snake_case)]
#[component]
pub fn PortfolioCurrency(
    portfolio_currency: Vec<PortfolioCurrency>,
) -> impl IntoView {

    let currency_config_resource = create_local_resource(|| (), move |_| fetch_currencies());

    let currency_config = RwSignal::new(CurrencyConfigurationResponse::default());
    let get_currency_display_scale = move |currency: String| {
        let cx = currency_config
            .get()
            .data
            .into_iter()
            .find(|c| c.ticker == currency);
        match cx {
            Some(c) => c.display_scale,
            None => 6u8,
        }
    };
    let get_currency_sign = move |currency: String| {
        let cx = currency_config
            .get()
            .data
            .into_iter()
            .find(|c| c.ticker == currency);
        match cx {
            Some(c) => c.sign.unwrap_or_else(|| String::from("")),
            None => String::from(""),
        }
    };

    let negative_num_style = move |num: f64| {
        if num < 0.0 {
            String::from("text-error")
        } else {
            String::from("")
        }
    };
    
    view! {
        <Suspense>
        {
            move || {
                currency_config_resource.and_then(|c| {
                    currency_config.set(c.clone());
                })
            }
        }
        </Suspense>
        {
            move || {
                portfolio_currency.iter().map(|c| {

                    let hide_per_cell = RwSignal::new(true);

                    view! {
                        <tr class="px924:hidden">
                            <td colspan = "6">
                            {
                                view! {
                                    <button class = "btn btn-ghost btn-md w-full flex gap-2 bg-base-100 justify-start border-l-2 border-l-success rounded-none" on:click = move |_| hide_per_cell.update(|c| *c = !*c) >
                                        <div class = "text-xs">
                                        <div class = "flex flex-wrap items-center gap-2 text-left">
                                            {
                                                get_asset_with_icon(c.currency.clone())
                                            }
                                        </div>
                                        </div>
                                        <div class = "flex flex-1 justify-end">
                                            <Show when = move || hide_per_cell.get() fallback = move || view! {<ArrowDown />}>
                                                <ArrowUp />
                                            </Show>
                                        </div>
                                    </button>
                                }.into_view()
                            }
                            </td>
                        </tr>

                        // MOBILE VIEW

                        <tr prop:hidden=move || hide_per_cell.get()>
                            <td colspan="6" class="px924:hidden">
                                <span class="text-sm text-success px924:hidden">
                                    "BALANCE: "
                                </span>
                                <span class = {negative_num_style(c.balance)}>{format_currency_with_scale(c.balance, get_currency_display_scale(c.currency.clone()), ",")}</span>
                            </td>
                        </tr>

                        <tr prop:hidden=move || hide_per_cell.get()>
                            <td colspan="6" class="px924:hidden">
                                <span class="text-sm text-success px924:hidden">
                                    "AVAILABLE BALANCE: "
                                </span>
                                <span class = {negative_num_style(c.available_balance)}>{format_currency_with_scale(c.available_balance, get_currency_display_scale(c.currency.clone()), ",")}</span>
                            </td>
                        </tr>

                        <tr prop:hidden=move || hide_per_cell.get()>
                            <td colspan="6" class="px924:hidden">
                                <span class="text-sm text-success px924:hidden">
                                    "EQUITY: "
                                </span>
                                <span class = "opacity-50 mr-1">{get_currency_sign(String::from("USD"))}</span><span class = {negative_num_style(c.equity_usd)}>{format_currency_with_scale(c.equity_usd, get_currency_display_scale(String::from("USD")), ",")}</span>
                            </td>
                        </tr>

                        <tr prop:hidden=move || hide_per_cell.get()>
                            <td colspan="6" class="px924:hidden">
                                <span class="text-sm text-success px924:hidden">
                                    "EXERCISED BALANCES: "
                                </span>
                                <span class = "opacity-50 mr-1">{get_currency_sign(c.currency.clone())}</span><span class = {negative_num_style(c.exercised_balances)}>{format_currency_with_scale(c.exercised_balances, get_currency_display_scale(c.currency.clone()), ",")}</span>
                            </td>
                        </tr>

                        <tr prop:hidden=move || hide_per_cell.get()>
                            <td colspan="6" class="px924:hidden">
                                <span class="text-sm text-success px924:hidden">
                                    "INTEREST PAYMENTS: "
                                </span>
                                <span class = {negative_num_style(c.interest_payments)}>{format_currency_with_scale(c.interest_payments, get_currency_display_scale(c.currency.clone()), ",")}</span>
                            </td>
                        </tr>
                        

                        // DESKTOP VIEW

                        <div class="hidden px924:contents">
                        // <div>
                            <tr>
                                <td class="flex items-center gap-2">
                                {
                                    get_asset_with_icon(c.currency.clone())
                                }
                                </td>
                                <td><span class = {negative_num_style(c.balance)}>{format_currency_with_scale(c.balance, get_currency_display_scale(c.currency.clone()), ",")}</span></td>
                                <td><span class = {negative_num_style(c.available_balance)}>{format_currency_with_scale(c.available_balance, get_currency_display_scale(c.currency.clone()), ",")}</span></td>
                                <td><span class = "opacity-50 mr-1">{get_currency_sign(String::from("USD"))}</span><span class = {negative_num_style(c.equity_usd)}>{format_currency_with_scale(c.equity_usd, get_currency_display_scale(String::from("USD")), ",")}</span></td>
                                // <td><span class = "opacity-50 mr-1">"$"</span><span class = {negative_num_style(c.live_pnl)}>{format_money(format_currency(c.live_pnl, get_currency_display_scale(String::from("USD"))), ",")}</span></td>
                                <td><span class = "opacity-50 mr-1">{get_currency_sign(c.currency.clone())}</span><span class = {negative_num_style(c.exercised_balances)}>{format_currency_with_scale(c.exercised_balances, get_currency_display_scale(c.currency.clone()), ",")}</span></td>
                                <td><span class = {negative_num_style(c.interest_payments)}>{format_currency_with_scale(c.interest_payments, get_currency_display_scale(c.currency.clone()), ",")}</span></td>
                            </tr>
                        </div>

                    }
                }).collect_view()
            }
        }
    }
}

fn get_asset_with_icon(currency: String) -> View {
    match currency.as_str() {
        "USD" => view! {
            <CurrencyIcon name = "USD".to_string() class = "w-7 h-7".to_string() />
            <div class = "grid grid-cols-1">
                <div class = "text-base font-bold">{currency}</div>
                <div class = "text-xs text-gray-500">USD</div>
            </div>
        }.into_view(),
        "USDC" => view! {
            <CurrencyIcon name = "USDC".to_string() class = "w-7 h-7".to_string() />
            <div class = "grid grid-cols-1">
                <div class = "text-base font-bold">{currency}</div>
                <div class = "text-xs text-gray-500">USD Coin</div>
            </div>
        }.into_view(),
        "USDT" => view! {
            <CurrencyIcon name = "USDT".to_string() class = "w-7 h-7".to_string() />
            <div class = "grid grid-cols-1">
                <div class = "text-base font-bold">{currency}</div>
                <div class = "text-xs text-gray-500">TetherUS</div>
            </div>
        }.into_view(),
        "BTC" => view! {
            <CurrencyIcon name = "BTC".to_string() class = "w-7 h-7".to_string() />
            <div class = "grid grid-cols-1">
                <div class = "text-base font-bold">{currency}</div>
                <div class = "text-xs text-gray-500">Bitcoin</div>
            </div>
        }.into_view(),
        "ETH" => view! {
            <CurrencyIcon name = "ETH".to_string() class = "w-7 h-7".to_string() />
            <div class = "grid grid-cols-1">
                <div class = "text-base font-bold">{currency}</div>
                <div class = "text-xs text-gray-500">Ethereum</div>
            </div>
        }.into_view(),
        "SOL" => view! {
            <CurrencyIcon name = "SOL".to_string() class = "w-7 h-7".to_string() />
            <div class = "grid grid-cols-1">
                <div class = "text-base font-bold">{currency}</div>
                <div class = "text-xs text-gray-500">Solana</div>
            </div>
        }.into_view(),
        "FIL" => view! {
            <CurrencyIcon name = "FIL".to_string() class = "w-7 h-7".to_string() />
            <div class = "grid grid-cols-1">
                <div class = "text-base font-bold">{currency}</div>
                <div class = "text-xs text-gray-500">File Coin</div>
            </div>
        }.into_view(),
        "ORCA" => view! {
            <CurrencyIcon name = "ORCA".to_string() class = "w-7 h-7".to_string() />
            <div class = "grid grid-cols-1">
                <div class = "text-base font-bold">{currency}</div>
                <div class = "text-xs text-gray-500">Orca</div>
            </div>
        }.into_view(),
        _ => view! {
            <CurrencyIcon name = "ELSE".to_string() class = "w-7 h-7".to_string() />
            <div class = "grid grid-cols-1">
                <div class = "text-base font-bold">{currency.clone()}</div>
                <div class = "text-xs text-gray-500">{currency}</div>
            </div>
        }.into_view()

    }
}