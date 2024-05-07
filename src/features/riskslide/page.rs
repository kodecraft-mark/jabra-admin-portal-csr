use leptos::*;
use wasm_bindgen::JsValue;
use web_sys::js_sys::Array;

use crate::commons::models::interestrates::*;
use crate::commons::services::interestrates::*;
use crate::commons::models::trade::*;
use crate::commons::models::currency::*;
use crate::commons::services::currency::*;
use crate::components::component_size::ComponentSize;
use crate::components::component_type::ComponentType;
use crate::components::default_none::DefaultNone;
use crate::components::download_anchor::DownloadCsvAnchor;
use crate::components::loading_spinners::Spinners;
use crate::components::menu_button::*;
use crate::commons::models::counterparty::*;
use crate::commons::services::counterparty::*;
use crate::components::select::{Checkbox, Checkboxes, SelectWithCheckbox};
use crate::features::riskslide::services::*;
use crate::utilities::date_util::*;
use crate::utilities::number_util::*;
use crate::utilities::string_util::*;

#[allow(non_snake_case)]
#[component]
pub fn RiskSlide() -> impl IntoView {
    let counterparty = RwSignal::new(String::from("ALL"));
    let currency_pair = RwSignal::new(String::from("BTC/USD"));

    let currency_config_resource = Resource::once(move || fetch_currencies());
    let currency_config = RwSignal::new(CurrencyConfigurationResponse::default());
    let get_currency_display_scale = Signal::derive(move || {
        let cx = currency_config
            .get()
            .data
            .into_iter()
            .find(|c| c.ticker == currency_pair.get().split('/').next().unwrap_or(""));
        match cx {
            Some(c) => c.display_scale,
            None => 6u8,
        }
    });
    
    let positions = RwSignal::new(TradeHistoryWithSpot::default());
    let current_interest_rates_resource = Resource::once(move || get_interest_rates());
    let current_interest_rate = RwSignal::new(InterestRate::default());
    let counter_parties = RwSignal::new(Checkboxes::default());

    let counterparties_resource: Resource<(),Result<GetCounterPartiesResponse, ServerFnError>,> = Resource::once(move || get_counter_parties());

    let counter_parties_signal = Signal::derive(move || {

    if counter_parties.get().checkboxes.len() > 0 {
        counter_parties.get().checkboxes.clone()
                .iter()
                .filter(|item| item.checkbox_state)
                .map(|item| item.value.clone())
                .collect::<Vec<_>>()
                .join(",")
        } else {
            String::from("ALL")
        }
           
    });
    let positions_resource = Resource::once(move || get_all_available_positions(currency_pair.get(), counter_parties_signal.get()));

    
    let deribit_positions_resource = Resource::once(move || get_deribit_positions(currency_pair.get(), counter_parties_signal.get()));
    let deribit_positions = RwSignal::new(Vec::<DeribitPositions>::default());
    let deribit_risk_slide = RwSignal::new(DeribitRiskSlide::default());

    let spot = move || positions.get().spot;
    let itm_otm_positions_resource = Resource::once(move || get_itm_otm_positions(currency_pair.get(), counter_parties_signal.get(), spot()));
    let itm_otm_positions = RwSignal::new(Vec::<ITMOTMPositions>::default());
    let itm_otm_risk_slide = RwSignal::new(ITMOTMRiskSlide::default());

    let collateral_resource = Resource::once(move || get_collateral(currency_pair.get(), counter_parties_signal.get(), spot()));
    let collateral_data = RwSignal::new(Vec::<CollateralData>::default());
    let collateral_risk_slide = RwSignal::new(Collateral::default());
    
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
        <div class = "w-full h-full p-4">
            <div class = "flex justify-between p-4 mt-2 bg-opacity-50 bg-base-300 rounded-xl">
                <div class = "flex-1 pb-5 ml-2 text-xl font-bold text-white">
                    <span>Risk Slide</span>
                </div>
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
                    move || 
                    {
    
                        counterparties_resource.and_then(|c| {
                            let mut checkboxes: Vec<Checkbox> = c.data.clone()
                            .into_iter()
                            .filter(|text| text.name != "JABRA")
                            .map(|text| Checkbox::new(false, text.name,text.ticker))
                            .collect();
                            checkboxes.insert(0, Checkbox::new(true, String::from("All"), String::from("ALL")));
                            counter_parties.set(Checkboxes {
                                checkboxes:checkboxes,
                            });
                        });
                    }
                } 
                <div class="flex justify-end flex-auto gap-3">
                    <div class="flex items-center w-full ">
                        <div class="flex items-center justify-end w-full">
                        <label class="mb-2 mr-2 text-xs font-bold text-gray-700 truncate md:text-sm" for="counter-parties">
                            SELECT COUNTERPARTY
                        </label>
                        </div>
                        <SelectWithCheckbox items=counter_parties placeholder=RwSignal::new(String::from("Select Counter Party")) />
                    </div>
                </div>
                </Suspense>
            </div>
            <div class = "flex justify-start gap-4 p-4 flex-0 ">
            {
                let page_keys = vec![String::from("BTC/USD"), String::from("ETH/USD")];
                page_keys.into_iter().map(|k| {
                    view! {
                        
                        <MenuButtonWithIcon selected_page = currency_pair page = k.clone() name = k.clone() icon=k.clone().split("/").next().expect("BTC").to_string() />
                    }
                }).collect_view()
            }
            </div>
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
                move || 
                {
                    current_interest_rates_resource.and_then(|i| {
                        current_interest_rate.set(i.get_newest_interest_rate());
                    });

                }
            } 
            </Suspense>
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
                move || 
                {

                    positions_resource.and_then(|c| {
                        positions.set(c.clone());
                    });

                }
            } 
            {
                move || {
                    if positions.get().data.len() <= 0  || current_interest_rate.get().rate == 0.0{
                        view!{
                            <div class = "items-center mt-5">
                                <div class = "p-5">
                                    <DefaultNone text = RwSignal::new(String::from("No Available Positions to calculate"))/>
                                </div>
                            </div>
                        }.into_view()
                    } else{
                        view!{
                            <RiskSlidePage data = positions r2 = current_interest_rate currency_pair = currency_pair deribit_risk_slide=deribit_risk_slide itm_otm_risk_slide=itm_otm_risk_slide collateral_risk_slide=collateral_risk_slide/>
                        }.into_view()
                    }
                }
            }
            
            </Suspense>
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
                move || 
                {

                    deribit_positions_resource.and_then(|c| {
                        deribit_positions.set(c.data.positions.clone());
                        deribit_risk_slide.set(c.data.deribit_risk_slide.clone());
                    });

                }
            } 
            {
                move || {
                    if deribit_positions.get().len() <= 0 {
                        if counter_parties_signal.get().contains("ALL"){
                            view!{
                                <div class = "items-center mt-5">
                                    <div class = "p-5">
                                        <DefaultNone text = RwSignal::new(String::from("No Available Deribit Positions"))/>
                                    </div>
                                </div>
                            }.into_view()
                        }else{
                            view!{
                                <div>  
                                </div>
                            }.into_view()
                        }
                        
                    } else{
                        view!{
                            <RiskSlideDeribitPage data = deribit_positions currency_pair = currency_pair currency_scale=get_currency_display_scale/>
                        }.into_view()
                    }
                }
            }
            </Suspense>
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
                move || 
                {
                    itm_otm_positions_resource.and_then(|c| {                     
                        itm_otm_positions.set(c.data.positions.clone());
                        itm_otm_risk_slide.set(c.data.positions_itm_otm_risk_slide.clone());
                    });

                }
            } 
            {
                move || {
                    if itm_otm_positions.get().len() <= 0 {
                        view!{
                            <div class = "items-center mt-5">
                                <div class = "p-5">
                                    <DefaultNone text = RwSignal::new(String::from("No Available ITM OTM"))/>
                                </div>
                            </div>
                        }.into_view()                      
                    } else{
                        view!{
                            <RiskSlideITMOTMPage data = itm_otm_positions currency_pair = currency_pair currency_scale=get_currency_display_scale/>
                        }.into_view()
                    }
                }
            }
            </Suspense>
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
                move || 
                {
                    collateral_resource.and_then(|c| {                     
                        collateral_data.set(c.data.exchanges_unwind.clone());
                        collateral_risk_slide.set(c.data.unwind_risk_slide.clone());
                    });

                }
            } 
            {
                move || {
                    if collateral_data.get().len() <= 0 {
                        view!{
                            <div class = "items-center mt-5">
                                <div class = "p-5">
                                    <DefaultNone text = RwSignal::new(String::from("No Available Exchanges"))/>
                                </div>
                            </div>
                        }.into_view()                      
                    } else{
                        view!{
                            <RiskSlideCollateral data = collateral_data currency_pair = currency_pair/>
                        }.into_view()
                    }
                }
            }
            </Suspense>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
// pub fn RiskSlidePage(data: RwSignal<QuoteOptionHistoryWithSpot>, refetcher: RwSignal<bool>) -> impl IntoView {
pub fn RiskSlidePage(data: RwSignal<TradeHistoryWithSpot>, r2: RwSignal<InterestRate>, currency_pair:RwSignal<String>,deribit_risk_slide:RwSignal<DeribitRiskSlide>, itm_otm_risk_slide:RwSignal<ITMOTMRiskSlide>, collateral_risk_slide:RwSignal<Collateral>) -> impl IntoView {
    let bump_percentage = RwSignal::new(1.0_f64);
    let has_data = move || data.get_untracked().data.len() > 0;
    let show_bump = RwSignal::new(false);

    let spot = move || data.get().spot;
    let positions_greeks = RwSignal::new(Vec::<RiskSlideTrade>::default());
    let atm_risk_slide = RwSignal::new(AtmRiskSlide::default());
    let bump_greeks = RwSignal::new(Vec::<BumpedGreek>::default());
   // let deribit_positions = RwSignal::new(Vec::<DeribitPositionsResponse>::default());

    let positions = RwSignal::new(
        data.get_untracked().extract_risk_slide_positions_by_currency_pair()
    );

    let csv_file = move || {
        let mut content = String::new();
        let header = "Counterparty,Instrument,Amount,Side,R2,Inception Price,Time to expiry,IV,Current Price,Delta,Gamma,Theta(USD),PnL(USD),PnL Percentage,Last Updated,Expiration Date\n";
        content.push_str(header);
        for i in positions_greeks.get() {
            let line = format!("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n", i.counterparty_name, i.instrument_name, i.amount, i.side, i.r2, i.inception_price, i.time_to_expiry, i.iv, i.current_price, i.delta, i.gamma, i.theta, i.pnl, i.pnl_percentage, i.last_updated, i.expiry_timestamp);
            content.push_str(&line.as_str());
        }
        content
    };
    let positions_resource = create_resource(
        move || (r2.get().rate, positions.get(), bump_percentage.get(), spot(), currency_pair.get()),
        move |(w, x, y, z, currency)| get_spot_and_greeks(w, x, y, z, currency)
    );

    let on_change_iv = move |iv: f64, id: u32| {
        positions.update(|p| {
            for r in p {
                if r.id == id {
                    r.iv = iv;
                    r.last_updated = get_current_local_time();
                }
            }
        });
    };

    let update_quote_iv_action = create_action(
        move |(new_iv, group_id): &(f64, String)| {
            let group_ids = vec![group_id.clone()];
            let request = UpdateIVRequest::new(group_ids, new_iv.clone());
            log::info!("request-atc: {:?}", request.deserialize());
            async move {
                let result = update_quote_iv(request).await;
                match result {
                    Ok(_res) => {
                        log::info!("Success");
                        // refetcher.update(|v| *v = !*v);

                    }
                    Err(e) => {
                        log::info!("error>: {:?}", e);
                    }
                }
            }
        }
    );

    let deribit_risk_slide_delta = move || deribit_risk_slide.get().delta;
    let deribit_risk_slide_gamma = move || deribit_risk_slide.get().gamma;
    let deribit_risk_slide_theta = move || deribit_risk_slide.get().theta;
    let deribit_risk_slide_pnl = move || deribit_risk_slide.get().pnl;

    let atm_risk_slide_delta = move || atm_risk_slide.get().delta;
    let atm_risk_slide_gamma = move || atm_risk_slide.get().gamma;
    let atm_risk_slide_theta = move || atm_risk_slide.get().theta;
    let atm_risk_slide_pnl = move || atm_risk_slide.get().pnl;

    let itm_otm_risk_slide_delta = move || itm_otm_risk_slide.get().delta;
    let itm_otm_risk_slide_pnl = move || itm_otm_risk_slide.get().pnl_in_mark_price;

    let collateral_risk_slide_pnl = move || collateral_risk_slide.get().pnl;

    let total_atm_risk_slide_delta = move || atm_risk_slide_delta() + deribit_risk_slide_delta() + itm_otm_risk_slide_delta();
    let total_atm_risk_slide_gamma = move || atm_risk_slide_gamma() + deribit_risk_slide_gamma();
    let total_atm_risk_slide_theta = move || atm_risk_slide_theta() + deribit_risk_slide_theta();
    let total_atm_risk_slide_pnl = move || atm_risk_slide_pnl() + deribit_risk_slide_pnl() + itm_otm_risk_slide_pnl() + collateral_risk_slide_pnl();

    let show_tooltip_by_id = move |id:String| {
        let doc = leptos_dom::document();
        let array = Array::new();
        array.push(&JsValue::from_str("hidden"));
        doc.get_element_by_id(id.as_str()).unwrap().class_list().remove(&array).unwrap();
    };
    let hide_tooltip_by_id = move |id:String| {
        let doc = leptos_dom::document();
        let array = Array::new();
        array.push(&JsValue::from_str("hidden"));
        doc.get_element_by_id(id.as_str()).unwrap().class_list().add(&array).unwrap();
    };

    view! {
        <div class="">
            <Transition
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
                        positions_resource.and_then(|p| {
                            positions_greeks.set(p.positions.clone());
                            atm_risk_slide.set(p.atm_risk_slide.clone());
                            bump_greeks.set(p.agg_bumped_greeks.clone());
                            //deribit_positions.set(p.deribit_positions.clone());
                            show_bump.set(true);
                        });
                        
                    }
                }
            </Transition>
            <Show when = move || atm_risk_slide.get() != AtmRiskSlide::default() >
                <div class = "py-3 mb-3">
                    <div class = "border border-success border-opacity-40">
                        <table class = "table table-xs table-zebra-zebra">
                            <thead>
                            <tr class = "font-semibold text-center text-white bg-success bg-opacity-30"><th colspan = "6">ATM RISK SLIDE</th></tr>
                                <tr class = "font-light text-center text-success bg-base-300">
                                    <th>"Pair"</th>
                                    <th>"Spot"</th>
                                    <th>"Delta"
                                    <button tabindex="0" aria-label="tooltip 3" role="link" class="relative focus:outline-none focus:ring-offset-2 focus:ring-2" 
                                        on:mouseover= move |_| {                                  
                                            show_tooltip_by_id("tooltip-delta".to_string());
                                            }
                                        on:focus= move |_| {
                                            
                                            hide_tooltip_by_id("tooltip-delta".to_string());
                                        } 
                                        on:mouseout= move |_| {
                                            hide_tooltip_by_id("tooltip-delta".to_string());
                                    }>
                                        <div class="cursor-pointer text-warning">
                                            <svg aria-haspopup="true" xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-info-circle text-warning" width="20" height="20" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" hover="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                                <path stroke="none" d="M0 0h24v24H0z" />
                                                <circle cx="12" cy="12" r="9" />
                                                <line x1="12" y1="8" x2="12.01" y2="8" />
                                                <polyline points="11 12 12 12 12 16 13 16" />
                                            </svg>
                                        </div>
                                        <div id="tooltip-delta" role="tooltip" class="absolute right-0 z-20 justify-center hidden w-auto p-4 mr-8 -mt-20 transition duration-150 ease-in-out rounded shadow-lg bg-base-300 item-center">
                                            <div class="">
                                                <div class="justify-center flex-1 bg-inherit">
                                                    <div class="">
                                                        <div class="text-sm text-success">Delta</div>
                                                    </div>
                                                </div>
                                                <table class = "table overflow-auto table-zebra table-xs">
                                                    <thead >
                                                        <tr>
                                                            <th class = "text-xs ">Active Positions</th>
                                                            <th class="text-white font-extralight">{atm_risk_slide_delta()}</th>
                                                        </tr>
                                                        <tr>
                                                            <th class = "text-xs ">Deribit Positions</th>
                                                            <th class="text-white font-extralight">{deribit_risk_slide_delta()}</th>
                                                        </tr>
                                                        <tr>
                                                            <th class = "text-xs ">ITM OTM Positions</th>
                                                            <th class="text-white font-extralight">{itm_otm_risk_slide_delta()}</th>
                                                        </tr>
                                                    </thead>
                                                </table>    
                                            </div>
                                        </div>
                                    </button>
                                    </th>
                                    <th>"Gamma"
                                    <button tabindex="0" aria-label="tooltip 3" role="link" class="relative focus:outline-none focus:ring-offset-2 focus:ring-2" 
                                        on:mouseover= move |_| {                                  
                                            show_tooltip_by_id("tooltip-gamma".to_string());
                                            }
                                        on:focus= move |_| {
                                            
                                            hide_tooltip_by_id("tooltip-gamma".to_string());
                                        } 
                                        on:mouseout= move |_| {
                                            hide_tooltip_by_id("tooltip-gamma".to_string());
                                    }>                                   
                                        <div class="cursor-pointer text-warning">
                                            <svg aria-haspopup="true" xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-info-circle text-warning" width="20" height="20" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" hover="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                                <path stroke="none" d="M0 0h24v24H0z" />
                                                <circle cx="12" cy="12" r="9" />
                                                <line x1="12" y1="8" x2="12.01" y2="8" />
                                                <polyline points="11 12 12 12 12 16 13 16" />
                                            </svg>
                                        </div>
                                        <div id="tooltip-gamma" role="tooltip" class="absolute right-0 z-20 justify-center hidden w-auto p-4 mr-8 -mt-20 transition duration-150 ease-in-out rounded shadow-lg bg-base-300 item-center">
                                            <div class="">
                                                <div class="justify-center flex-1 bg-inherit">
                                                    <div class="">
                                                        <div class="text-sm text-success">Gamma</div>
                                                    </div>
                                                </div>
                                                <table class = "table overflow-auto table-zebra table-xs">
                                                    <thead >
                                                        <tr>
                                                            <th class = "text-xs ">Active Positions</th>
                                                            <th class="text-white font-extralight">{atm_risk_slide_gamma()}</th>
                                                        </tr>
                                                        <tr>
                                                            <th class = "text-xs ">Deribit Positions</th>
                                                            <th class="text-white font-extralight">{deribit_risk_slide_gamma()}</th>
                                                        </tr>
                                                    </thead>
                                                </table>    
                                            </div>
                                        </div>
                                    </button>
                                    </th>
                                    <th>"Theta"
                                    <button tabindex="0" aria-label="tooltip 3" role="link" class="relative focus:outline-none focus:ring-offset-2 focus:ring-2" 
                                        on:mouseover= move |_| {                                  
                                            show_tooltip_by_id("tooltip-theta".to_string());
                                            }
                                        on:focus= move |_| {
                                            
                                            hide_tooltip_by_id("tooltip-theta".to_string());
                                        } 
                                        on:mouseout= move |_| {
                                            hide_tooltip_by_id("tooltip-theta".to_string());
                                    }>
                                        <div class="cursor-pointer text-warning">
                                            <svg aria-haspopup="true" xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-info-circle text-warning" width="20" height="20" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" hover="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                                <path stroke="none" d="M0 0h24v24H0z" />
                                                <circle cx="12" cy="12" r="9" />
                                                <line x1="12" y1="8" x2="12.01" y2="8" />
                                                <polyline points="11 12 12 12 12 16 13 16" />
                                            </svg>
                                        </div>
                                        <div id="tooltip-theta" role="tooltip" class="absolute right-0 z-20 justify-center hidden w-auto p-4 mr-8 -mt-20 transition duration-150 ease-in-out rounded shadow-lg bg-base-300 item-center">
                                            <div class="">
                                                <div class="justify-center flex-1 bg-inherit">
                                                    <div class="">
                                                        <div class="text-sm text-success">Theta</div>
                                                    </div>
                                                </div>
                                                <table class = "table overflow-auto table-zebra table-xs">
                                                    <thead >
                                                        <tr>
                                                            <th class = "text-xs ">Active Positions</th>
                                                            <th class="text-white font-extralight">{atm_risk_slide_theta()}</th>
                                                        </tr>
                                                        <tr>
                                                            <th class = "text-xs ">Deribit Positions</th>
                                                            <th class="text-white font-extralight">{deribit_risk_slide_theta()}</th>
                                                        </tr>
                                                    </thead>
                                                </table>    
                                            </div>
                                        </div>                               
                                    </button>
                                    </th>
                                    <th>"Pnl"
                                    <button tabindex="0" aria-label="tooltip 3" role="link" class="relative focus:outline-none focus:ring-offset-2 focus:ring-2" 
                                        on:mouseover= move |_| {                                  
                                            show_tooltip_by_id("tooltip".to_string());
                                            }
                                        on:focus= move |_| {
                                            
                                            hide_tooltip_by_id("tooltip".to_string());
                                        } 
                                        on:mouseout= move |_| {
                                            hide_tooltip_by_id("tooltip".to_string());
                                    }>
                                    <div class="cursor-pointer text-warning">
                                            <svg aria-haspopup="true" xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-info-circle text-warning" width="20" height="20" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" hover="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round">
                                                <path stroke="none" d="M0 0h24v24H0z" />
                                                <circle cx="12" cy="12" r="9" />
                                                <line x1="12" y1="8" x2="12.01" y2="8" />
                                                <polyline points="11 12 12 12 12 16 13 16" />
                                            </svg>
                                        </div>
                                        <div id="tooltip" role="tooltip" class="absolute right-0 z-20 justify-center hidden w-auto p-4 mr-8 -mt-20 transition duration-150 ease-in-out rounded shadow-lg bg-base-300 item-center">
                                            <div class="">
                                                <div class="justify-center flex-1 bg-inherit">
                                                    <div class="">
                                                        <div class="text-sm text-success">PnL</div>
                                                    </div>
                                                </div>
                                                <table class = "table overflow-auto table-zebra table-xs">
                                                    <thead >
                                                        <tr>
                                                            <th class = "text-xs ">Active Positions</th>
                                                            <th class="font-extralight"> <span class = {move || {if atm_risk_slide_pnl() < 0.0 { "text-error"} else { "text-success"}}}>{move || format_currency_with_scale(atm_risk_slide_pnl(), 2u8, ",")}</span></th>
                                                        </tr>
                                                        <tr>
                                                            <th class = "text-xs ">Deribit Positions</th>
                                                            <th class="font-extralight"> <span class = {move || {if deribit_risk_slide_pnl() < 0.0 { "text-error"} else { "text-success"}}}>{move || format_currency_with_scale(deribit_risk_slide_pnl(), 2u8, ",")}</span></th>
                                                        </tr>
                                                        <tr>
                                                            <th class = "text-xs ">ITM OTM Positions</th>
                                                            <th class="font-extralight"> <span class = {move || {if itm_otm_risk_slide_pnl() < 0.0 { "text-error"} else { "text-success"}}}>{move || format_currency_with_scale(itm_otm_risk_slide_pnl(), 2u8, ",")}</span></th>
                                                        </tr>
                                                        <tr>
                                                            <th class = "text-xs ">Collateral</th>
                                                            <th class="font-extralight"> <span class = {move || {if collateral_risk_slide_pnl() < 0.0 { "text-error"} else { "text-success"}}}>{move || format_currency_with_scale(collateral_risk_slide_pnl(), 2u8, ",")}</span></th>
                                                        </tr>
                                                    </thead>
                                                </table>    
                                            </div>
                                        </div> 
                                    </button>
                                    </th>
                                </tr>
                            </thead>
                            <tbody class = "text-center font-extralight">
                                <tr>
                                    <td>{currency_pair.get()}</td>
                                    <td>{move || atm_risk_slide.get().spot}</td>
                                    <td>{total_atm_risk_slide_delta()}</td>
                                    <td>{total_atm_risk_slide_gamma()}</td>
                                    <td>
                                        <span class = "opacity-40">"$ "</span>
                                        <span>{total_atm_risk_slide_theta()}</span>
                                    </td>
                                    <td>
                                        <span class = "opacity-40">"$ "</span>
                                        <span class = {move || {if total_atm_risk_slide_pnl() < 0.0 { "text-error"} else { "text-success"}}}>{move || format_currency_with_scale(total_atm_risk_slide_pnl(), 2u8, ",")}</span>
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </Show>
            <Show when = move || show_bump.get() >
            <div class = "flex w-2/5 py-3 mb-3">
                <div class = "flex flex-col p-3 border border-success border-opacity-40">
                    <div class = "flex justify-between flex-grow my-2 font-light">
                        <div class="mr-2">
                            <label class = "block text-xs">Bump Table</label>
                        </div>
                        // <span class="text-white indicator-item badge badge-success badge-sm">{move || bump_percentage.get()}</span>
                        <div class="mb-2 ml-2 join">
                        <input prop:value={move || bump_percentage.get() }class = "block w-4/5 text-xs border-gray-800 rounded shadow-md join-item input-sm text-success hover:shadow-sm hover:shadow-success bg-base-100" type="number" step="0.1" name="off_strike"
                            on:change = move |e| {
                                            let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                                            bump_percentage.set(val);
                                            if bump_percentage.get() > 15.0 {
                                                bump_percentage.set(15.0);
                                            }   
                                        }
                                    />
                                <button tabindex="-1" class="w-1/5 pointer-events-none btn join-item btn-sm b text-success">"%"</button>
                        </div>
                    </div>
                    
                    <input class = "range range-success range-xs" type="range" name="off_strike" prop:step = "0.01" prop:value = bump_percentage prop:min = 0.0 prop:max = 15.0
                        on:change = move |e| {
                            let val = event_target_value(&e).parse::<f64>().unwrap_or_default();
                            bump_percentage.set(val);
                        }
                    />
                    <div class = "flex justify-between flex-grow w-full text-sm font-light">
                            <span>0.0</span>
                            <span>15.0</span>
                    </div>
                </div>
            </div>
            </Show>
            <div class = "py-3 mb-3">
                <Show when = move || {bump_greeks.get() != Vec::<BumpedGreek>::default()}>
                    <div class = "border border-success border-opacity-40">
                        <table class = "table table-xs table-zebra-zebra">
                            <thead>
                            <tr class = "font-semibold text-center text-white bg-success bg-opacity-30"><th colspan = "6">{ move || {format!("{} x SPOT x GREEKS", currency_pair.get())}}</th></tr>
                            </thead>
                            <tbody>
                            {
                                move || {
                                    bump_greeks.get().into_iter().map(|greek| {
                                        //Add dollar sign to Theta
                                        let has_dollar_sign = if greek.name.to_uppercase() == "THETA" {"opacity-40"} else {"hidden"};
                                        view! {
                                            <tr class = "text-center font-extralight first:text-success first:font-semibold">
                                            <td>{greek.name.to_uppercase()}</td>
                                            {
                                                greek.values.into_iter().map(|val| {
                                                    view! {
                                                        <td><span class = {has_dollar_sign}>"$ "</span>{val}</td>
                                                    }
                                                }).collect_view()
                                            }
                                            </tr>
                                        }
                                    }).collect_view()

                                }
                            }
                            </tbody>
                        </table>
                    </div>
                </Show>
            </div>
            <div class = "py-3 mb-3 ">
                <Suspense
                    fallback = move || view! {
                            <div class = "items-center mt-5">
                                <div class = "flex justify-center ">
                                    <Spinners size = ComponentSize::SMALL _type = ComponentType::SUCCESS />
                                </div>
                            </div>
                        }
                >
                    <Show when = move || {positions_greeks.get().len() > 0}>
                    <DownloadCsvAnchor content = csv_file() file_name = String::from("BTC-USD-Active-Positions")/>
                    <div class = "overflow-auto border border-success border-opacity-40">
                        <table class = "table table-xs table-zebra-zebra">
                            <thead>
                                <tr class = "font-semibold text-center text-white bg-success bg-opacity-30"><th colspan = "16">{ move || {format!("{} ACTIVE POSITIONS", currency_pair.get())}}</th></tr>
                                <tr class = "font-light text-center text-success bg-base-300">
                                    <th>"Counterparty"</th>
                                    <th>"Instrument"</th>
                                    <th>"Amount"</th>
                                    <th>"Side"</th>
                                    <th>"R2"</th>
                                    <th>"Inception Price"</th>
                                    <th>"Time to expiry"</th>
                                    <th>"IV"</th>
                                    <th>"Current Price"</th>
                                    <th>"Delta"</th>
                                    <th>"Gamma"</th>
                                    <th>"Theta"</th>
                                    <th>"PnL"</th>
                                    <th>"PnL Percentage"</th>
                                    <th>"Last Updated"</th>
                                    <th>"Expiration Date"</th>
                                </tr>
                            </thead>
                            <tbody class = "text-center font-extralight">
                                {
                                    move || {
                                        positions_greeks.get().into_iter().map(|position| {
                                            let pos = position.clone();
                                            let colored_value_pnl = if pos.pnl < 0.0 { "text-error"} else { "text-success"};
                                            let colored_value_pnl_pctg = if pos.pnl_percentage < 0.0 { "text-error"} else { "text-success"};
                                            view! {
                                                <tr>
                                                    <td>{pos.counterparty_name}</td>
                                                    <td>{pos.instrument_name}</td>
                                                    <td>{pos.amount}</td>
                                                    <td>{pos.side}</td>
                                                    <td>{pos.r2}</td>
                                                    <td>{pos.inception_price}</td>
                                                    <td>{pos.time_to_expiry}</td>
                                                    <td>
                                                    <input class = "text-xs border-gray-800 rounded shadow-md input-xs text-success hover:shadow-sm hover:shadow-success" type = "number" prop:value = {position.iv.clone()}
                                                        on:change =  move |event| {
                                                            let val: f64 = event_target_value(&event).parse().unwrap();
                                                            on_change_iv(val.clone(), position.id);
                                                            update_quote_iv_action.dispatch((val, pos.group_id.clone()));
                                                        }
                                                    />
                                                    </td>
                                                    <td>{pos.current_price}</td>
                                                    <td>{pos.delta}</td>
                                                    <td>{pos.gamma}</td>
                                                    <td>
                                                        <span class = "opacity-40">"$ "</span>
                                                        <span>{pos.theta}</span>
                                                    </td>
                                                    <td>
                                                        <span class = "opacity-40">"$ "</span>
                                                        <span class = {colored_value_pnl}>{format_currency_with_scale(pos.pnl, 2u8, ",")}</span>
                                                    </td>
                                                    <td>
                                                        <span class = {colored_value_pnl_pctg}>{pos.pnl_percentage}</span>
                                                        <span class = "opacity-40">" %"</span>
                                                    </td>
                                                    <td>{pos.last_updated}</td>
                                                    <td>{pos.expiry_timestamp}</td>
                                                </tr>
                                            }
                                        }).collect_view()
                                    }
                                }
                            </tbody>
                        </table>
                    </div>
                    </Show>
                </Suspense>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn RiskSlideDeribitPage(data: RwSignal<Vec<DeribitPositions>>, currency_pair:RwSignal<String>, currency_scale:Signal<u8>) -> impl IntoView {
    let csv_file = move || {
        let mut content = String::new();
        let header = "Instrument Name,Amount,Side,Kind,Total PnL,Realized PnL,Delta,Mark Price,Index Price\n";
        content.push_str(header);
        for i in data.get() {
            let line = format!("{},{},{},{},{},{},{},{},{}\n", match i.instrument_name{Some(v) => v,None => String::new(),}, match i.size{Some(v) => v,None => 0.0,},  match i.direction{Some(v) => v,None => String::new(),}, match i.kind{Some(v) => v,None => String::new(),} ,  match i.total_profit_loss{Some(v) => v,None => 0.0,}, match i.realized_profit_loss{Some(v) => v,None => 0.0,} ,  match i.delta{Some(v) => v,None => 0.0,},  match i.mark_price{Some(v) => v,None => 0.0,},  match i.index_price{Some(v) => v,None => 0.0,} );
            content.push_str(&line.as_str());
        }
        content
    };
    let file_name = move || format!("{}-Deribit-Positions", currency_pair.get());
    view! {
        <div class="">
            <div class = "py-3 mb-3">
                <Show when = move || {data.get().len() > 0}>
                <DownloadCsvAnchor content = csv_file() file_name = file_name()/>
                <div class = "overflow-auto border border-success border-opacity-40">
                    <table class = "table table-xs table-zebra-zebra">
                        <thead>
                            <tr class = "font-semibold text-center text-white bg-success bg-opacity-30"><th colspan = "16">{ move || {format!("{} DERIBIT POSITIONS", currency_pair.get())}}</th></tr>
                            <tr class = "font-light text-center text-success bg-base-300">
                                <th>"Instrument Name"</th>
                                <th>"Amount"</th>
                                <th>"Side"</th>
                                <th>"Kind"</th>
                                <th>"Total PnL"</th>
                                <th>"Realized PnL"</th>
                                <th>"Delta"</th>
                                <th>"Gamma"</th>
                                <th>"Theta"</th>
                                <th>"Mark Price"</th>
                                <th>"Index Price"</th>                                 
                            </tr>
                        </thead>
                        <tbody class = "text-center font-extralight">
                            {
                                move || {
                                    data.get().into_iter().map(|d_position| {
                                        let d_pos = d_position.clone();
                                        let colored_value_total_pnl = if match d_pos.total_profit_loss{Some(v) => v,None => 0.0,} < 0.0 { "text-error"} else { "text-success"};
                                        let colored_value_total_realized_pnl = if match d_pos.realized_profit_loss{Some(v) => v,None => 0.0,} < 0.0 { "text-error"} else { "text-success"};
                                        view! {
                                            <tr>
                                                <td>{d_pos.instrument_name}</td>
                                                <td>{d_pos.size}</td>
                                                <td>{capitalize_first(match d_pos.direction{Some(v) => v,None => String::new(),})}</td>
                                                <td>{capitalize_first(match d_pos.kind{Some(v) => v,None => String::new(),})}</td>
                                                <td><span class = {colored_value_total_pnl}>{format_currency(match d_pos.total_profit_loss{Some(v) => v,None => 0.0,},currency_scale.get())}</span></td>
                                                <td><span class = {colored_value_total_realized_pnl}>{format_currency(match d_pos.realized_profit_loss{Some(v) => v,None => 0.0,},currency_scale.get())}</span></td>
                                                <td>{d_pos.delta}</td>
                                                <td>{match d_pos.gamma{Some(v) => v,None => 0.0,}}</td>
                                                <td>{match d_pos.theta{Some(v) => v,None => 0.0,}}</td>
                                                <td>{d_pos.mark_price}</td>
                                                <td>{d_pos.index_price}</td>                                                  
                                            </tr>
                                        }
                                    }).collect_view()
                                }
                            }
                        </tbody>
                    </table>
                </div>
                </Show>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn RiskSlideITMOTMPage(data: RwSignal<Vec<ITMOTMPositions>>, currency_pair:RwSignal<String>, currency_scale:Signal<u8>) -> impl IntoView {
    let csv_file = move || {
        let mut content = String::new();
        let header = "Counterparty,Instrument Name,Amount,Side,Pnl,Index Price,Date Created,Expiry Date,Trade Status\n";
        content.push_str(header);
        for i in data.get() {
            let line = format!("{},{},{},{},{},{},{},{},{}\n", i.counterparty_id.name, i.instrument_name, i.amount, i.side, i.pnl, i.index_price, i.date_created, i.expiry_timestamp, i.trade_status);
            content.push_str(&line.as_str());
        }
        content
    };
    let file_name = move || format!("{}-ITM-OTM-Positions", currency_pair.get());

   
    view! {
        <div class="">
            <div class = "py-3 mb-3">
                <Show when = move || {data.get().len() > 0}>
                <DownloadCsvAnchor content = csv_file() file_name = file_name()/>
                <div class = "overflow-auto border border-success border-opacity-40">
                    <table class = "table table-xs table-zebra-zebra">
                        <thead>
                            <tr class = "font-semibold text-center text-white bg-success bg-opacity-30"><th colspan = "16">{ move || {format!("{} ITM OTM POSITIONS", currency_pair.get())}}</th></tr>
                            <tr class = "font-light text-center text-success bg-base-300">
                                <th>"Counterparty"</th>
                                <th>"Instrument Name"</th>
                                <th>"Amount"</th>
                                <th>"Side"</th>
                                <th>"PnL"</th>
                                <th>"Index Price"</th> 
                                <th>"Date Created"</th>
                                <th>"Expiry Date"</th>
                                <th>"Trade Status"</th>                                                          
                            </tr>
                        </thead>
                        <tbody class = "text-center font-extralight">
                            {
                                move || {
                                    data.get().into_iter().map(|d_position| {
                                        let d_pos = d_position.clone();
                                        let colored_value_pnl = if d_pos.pnl < 0.0 { "text-error"} else { "text-success"};
                                        view! {
                                            <tr>
                                                <td>{d_pos.counterparty_id.name}</td>
                                                <td>{d_pos.instrument_name}</td>
                                                <td>{d_pos.amount}</td>
                                                <td>{d_pos.side}</td>
                                                <td><span class = {colored_value_pnl}>{format_currency(d_pos.pnl,currency_scale.get())}</span></td>
                                                <td>{d_pos.index_price}</td>
                                                <td>{format_utc_str_to_local_str(d_pos.date_created)}</td>
                                                <td>{format_utc_str_to_local_str(d_pos.expiry_timestamp)}</td>     
                                                <td>{d_pos.trade_status}</td>                                               
                                            </tr>
                                        }
                                    }).collect_view()
                                }
                            }
                        </tbody>
                    </table>
                </div>
                </Show>
            </div>
        </div>
    }
}

#[allow(non_snake_case)]
#[component]
pub fn RiskSlideCollateral(data: RwSignal<Vec<CollateralData>>, currency_pair:RwSignal<String>) -> impl IntoView {
    let csv_file = move || {
        let mut content = String::new();
        let header = "Exchange Name,Initial USD,Current USD,Notional,Pnl\n";
        content.push_str(header);
        for i in data.get() {
            let line = format!("{},{},{},{},{}\n", i.exchange_name, i.initial_usd, i.current_usd, i.notional, i.pnl);
            content.push_str(&line.as_str());
        }
        content
    };
    let file_name = move || format!("{}-Collateral", currency_pair.get());

    view! {
        <div class="">
            <div class = "py-3 mb-3">
                <Show when = move || {data.get().len() > 0}>
                <DownloadCsvAnchor content = csv_file() file_name = file_name()/>
                <div class = "overflow-auto border border-success border-opacity-40">
                    <table class = "table table-xs table-zebra-zebra">
                        <thead>
                            <tr class = "font-semibold text-center text-white bg-success bg-opacity-30"><th colspan = "16">{ move || {format!("{} COLLATERAL ", currency_pair.get())}}</th></tr>
                            <tr class = "font-light text-center text-success bg-base-300">
                                <th>"Exchange Name"</th>
                                <th>"Initial USD"</th>
                                <th>"Current USD"</th>
                                <th>"Notional"</th>
                                <th>"PnL"</th>                                                       
                            </tr>
                        </thead>
                        <tbody class = "text-center font-extralight">
                            {
                                move || {
                                    data.get().into_iter().map(|d_position| {
                                        let d_pos = d_position.clone();
                                        let colored_value_pnl = if d_pos.pnl < 0.0 { "text-error"} else { "text-success"};
                                        let colored_value_initial_usd = if d_pos.initial_usd < 0.0 { "text-error"} else { "text-success"};
                                        let colored_value_current_usd = if d_pos.current_usd < 0.0 { "text-error"} else { "text-success"};
                                        view! {
                                            <tr>
                                                <td>{d_pos.exchange_name}</td>
                                                <td>
                                                    <span class = "opacity-40">"$ "</span>
                                                    <span class = {colored_value_initial_usd}>{format_currency_with_scale(d_pos.initial_usd, 2u8, ",")}</span>
                                                </td>
                                                <td>
                                                    <span class = "opacity-40">"$ "</span>
                                                    <span class = {colored_value_current_usd}>{format_currency_with_scale(d_pos.current_usd, 2u8, ",")}</span>
                                                </td>
                                                <td>{d_pos.notional}</td>
                                                <td>
                                                    <span class = "opacity-40">"$ "</span>
                                                    <span class = {colored_value_pnl}>{format_currency_with_scale(d_pos.pnl, 2u8, ",")}</span>
                                                </td>                                              
                                            </tr>
                                        }
                                    }).collect_view()
                                }
                            }
                        </tbody>
                    </table>
                </div>
                </Show>
            </div>
        </div>
    }
}