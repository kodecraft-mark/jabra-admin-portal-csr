use leptos::ServerFnError;
use crate::commons::models::defaults::BlankRequest;
use crate::utilities::cookies::{get_jabra_cookie, JabraCookie};
use crate::utilities::date_util::format_utc_str_to_local_str;
use crate::utilities::http_wrapper::{call_and_parse,call, HttpMethod};
use crate::commons::models::trade::{Trade, TradeHistory,RiskSlideTrade,PositionGreekRequest,
    PositionsGreeksResponse,
    PositionsGreeksResponseExtraData,UpdateIVRequest,
    PositionsGreeksRequest,TradeHistoryWithSpot,DeribitPositionsRequest,DeribitPositionsResponseData,ITMOTMPositionsRequest,ITMOTMPositionsResponseData,CollateralRequest,CollateralResponseData};
use crate::commons::models::coinbase::CoinbaseSpotPriceResponse;
use crate::utilities::errors::JabraError;

pub async fn get_all_available_positions(
    currency_pair: String,
    counterparty: String,
) -> Result<TradeHistoryWithSpot, ServerFnError> {
    
    /*
       Coibase API call here
    */
    let coinbase_url = option_env!("COINBASE_V2");
    let currency_pair_ = currency_pair.replace("/", "-");
    // let coinbase_path = format!("{}/prices/{}/spot", url, coinbase_name);
    let coinbase_response = call_and_parse::<BlankRequest,CoinbaseSpotPriceResponse,>(
        Option::None,
        format!("{}/prices/{}/spot", coinbase_url.unwrap_or_default(), currency_pair_),
        reqwest::header::HeaderMap::default(),
        HttpMethod::GET,
    )
    .await;
    let spot = match coinbase_response {
        Ok(res) => {
            log::info!("res: {:?}", res);
            res.data.amount.parse::<f64>().unwrap()
        }
        Err(e) => {
            log::info!("error: {:?}", e);
            return Err(ServerFnError::new(e.to_string()));
        }
    };

    /* Coinbase API call end */
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    if jwt_cookie.is_expired() {
        return Ok(Default::default());
    }
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = option_env!("DIRECTUSURL");
    //let path = format!("{}/items/trade?filter[counterparty_id][ticker][_in]={}&filter[expiry_timestamp][_gte]=$NOW&filter[activity][_eq]=open&filter[pair_id][name][_in]={}&sort=-expiry_timestamp&fields={}", url, if counterparty=="ALL" {String::from("JABRA")} else{ counterparty },currency_pair, Trade::get_query());
    let path = format!("{}/items/trade?filter[counterparty_id][ticker][_eq]={}{}&filter[expiry_timestamp][_gte]=$NOW&filter[activity][_eq]=open&filter[pair_id][name][_in]={}&sort=-expiry_timestamp&fields={}", url.unwrap_or_default(), String::from("JABRA"),if counterparty.contains("ALL") {String::from("")} else{ format!("&filter[party_b][ticker][_in]={}",counterparty) },currency_pair, Trade::get_query());
    log::info!("path: {:?}", path);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response: Result<
        TradeHistory,
        JabraError,
    > = call_and_parse::<
        BlankRequest,
        TradeHistory,
    >(Option::None, path, headers.clone(), HttpMethod::GET)
    .await;
    match response {
        Ok(res) => {
            Ok(TradeHistoryWithSpot {
                data: res.data,
                spot,
            })
        }
        Err(e) => {
            log::error!("error3: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

pub async fn get_deribit_positions(
    currency_pair: String,
    counterparty: String,
) -> Result<DeribitPositionsResponseData, ServerFnError> {

    if !counterparty.contains("ALL") {
        return Ok(DeribitPositionsResponseData::default());
    }
    
    let currency = currency_pair.split('/').next().unwrap_or("");
    let pricer_url = option_env!("PRICERENGINEURL");
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());
    let path = format!("{}/risk/deribit", pricer_url.unwrap_or_default());
    //Create a request Greek API
    let deribit_positions_request = DeribitPositionsRequest {
        currency: currency.to_string(),
    };
    let response = call_and_parse::<DeribitPositionsRequest, DeribitPositionsResponseData>(
        Some(deribit_positions_request),
        path,
        headers,
        HttpMethod::POST
    ).await;
    //log::info!("response: {:?}", response);
    match response {
        Ok(res) => {
            //log::info!("res deribit: {:?}", res);
            Ok(res)
        }
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

pub async fn get_itm_otm_positions(
    currency_pair: String,
    counterparty: String,
    spot: f64,
) -> Result<ITMOTMPositionsResponseData, ServerFnError> {

    let currency = currency_pair.split('/').next().unwrap_or("");
    let pricer_url = option_env!("PRICERENGINEURL");
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());
    let path = format!("{}/risk/positions_itm_otm", pricer_url.unwrap_or_default());
    //Create a request Greek API
    let deribit_positions_request = ITMOTMPositionsRequest {
        currency: currency.to_string(),
        counterparty: counterparty.to_string(),
        current_spot: spot,
    };

    log::info!("deribit_positions_request: {:?}", deribit_positions_request.clone());
    let response = call_and_parse::<ITMOTMPositionsRequest, ITMOTMPositionsResponseData>(
        Some(deribit_positions_request),
        path,
        headers,
        HttpMethod::POST
    ).await;
    //log::info!("response: {:?}", response);
    match response {
        Ok(res) => {
            //log::info!("res deribit: {:?}", res);
            Ok(res)
        }
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

pub async fn get_collateral(
    currency_pair: String,
    counterparty: String,
    spot: f64,
) -> Result<CollateralResponseData, ServerFnError> {

    let currency = currency_pair.split('/').next().unwrap_or("");
    let pricer_url = option_env!("PRICERENGINEURL");
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());
    let path = format!("{}/risk/collateral", pricer_url.unwrap_or_default());
    //Create a request Greek API
    let deribit_positions_request = CollateralRequest {
        currency: currency.to_string(),
        counterparty: counterparty.to_string(),
        current_spot: spot,
    };
    let response = call_and_parse::<CollateralRequest, CollateralResponseData>(
        Some(deribit_positions_request),
        path,
        headers,
        HttpMethod::POST
    ).await;
    //log::info!("response: {:?}", response);
    match response {
        Ok(res) => {
            //log::info!("res deribit: {:?}", res);
            Ok(res)
        }
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

pub async fn get_spot_and_greeks(
    r2: f64,
    data: Vec<RiskSlideTrade>,
    bump: f64,
    spot: f64,
    currency_pair: String,
) -> Result<PositionsGreeksResponseExtraData, ServerFnError> {

    let currency = currency_pair.split('/').next().unwrap_or("");
    let pricer_url = option_env!("PRICERENGINEURL");
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer).unwrap());
    let path = format!("{}/quote/greeks", pricer_url.unwrap_or_default());
    //Create a request Greek API
    let mut new_data = data.clone();
    let mut positions_greeks = Vec::<PositionGreekRequest>::default();
    for d in data {
        let ps = PositionGreekRequest::new(
            d.side,
            d.option_kind,
            d.amount.abs(),
            d.strike,
            d.time_to_expiry,
            d.inception_price.abs(),
            Some(spot),
            Some(r2),
            Some(0.0),
            Some(d.iv),
            Some(d.expiry_timestamp),
            Some(d.id.to_string())
        );
        positions_greeks.push(ps);
    }
    let positions_greeks_request = PositionsGreeksRequest {
        positions: positions_greeks,
        current_spot: spot,
        spot_bump: bump/100.0,
        bump_times: 3
    };
    let response = call_and_parse::<PositionsGreeksRequest, PositionsGreeksResponse>(
        Some(positions_greeks_request),
        path,
        headers,
        HttpMethod::POST
    ).await;
    match response {
        Ok(res) => {
            // log::info!("res: {:?}", res);
            for p in &res.data.positions {
                let req_id = p.req_id.clone().unwrap_or("0".to_string());
                if
                    let Some(v) = new_data
                        .iter_mut()
                        .find(|x| x.id == req_id.parse::<u32>().unwrap())
                {
                    v.inception_price = v.inception_price.abs();
                    v.current_price = p.px_in_quote_ccy;
                    v.delta = p.greeks.delta;
                    v.gamma = p.greeks.gamma;
                    v.theta = p.greeks.theta;
                    v.pnl = p.pnl;
                    v.pnl_percentage = p.pnl_percentage;
                    v.r2 = r2;
                    let expiry_timestamp = v.expiry_timestamp.clone();
                    v.expiry_timestamp = format_utc_str_to_local_str(expiry_timestamp);
                }
            }
            Ok(
                PositionsGreeksResponseExtraData::new(
                    res.data.atm_risk_slide,
                    new_data,
                    res.data.agg_bumped_greeks,
                )
            )
        }
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

pub async fn update_quote_iv(request: UpdateIVRequest) -> Result<bool, ServerFnError> {

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer_token = format!("Bearer {}", jwt_cookie.access_token);

    log::debug!("request: {:?}", request.deserialize());
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/quotes_option", url.unwrap_or_default());

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", reqwest::header::HeaderValue::from_str(&bearer_token).unwrap());

    let response = call::<UpdateIVRequest>(
        Some(request),
        path,
        headers,
        HttpMethod::PATCH
    ).await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::info!("error-: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}
