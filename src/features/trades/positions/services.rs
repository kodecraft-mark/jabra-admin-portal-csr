use leptos::ServerFnError;

use crate::{commons::models::{defaults::BlankRequest, trade::{PositionGreekRequest, PositionsGreeksRequest, PositionsGreeksResponse, Trade, TradeHistory}}, utilities::{cookies::{get_jabra_cookie, JabraCookie}, date_util::time_to_expiry, http_wrapper::{call_and_parse, HttpMethod}}};


pub async fn get_positions_with_live_pnl(
    pair: String,
    spot: f64,
    countery_party: String,
) -> Result<TradeHistory, ServerFnError> {
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    if jwt_cookie.is_expired() {
        return Ok(Default::default());
    }
    let bearer_token = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/trade?filter[counterparty_id][ticker][_eq]={}&filter[expiry_timestamp][_gte]=$NOW&filter[activity][_eq]=open&filter[pair_id][name][_eq]={}&sort[]=-expiry_timestamp&fields={}", url.unwrap_or_default(), countery_party, pair, Trade::get_query());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer_token).unwrap(),
    );

    let response =
        call_and_parse::<BlankRequest, TradeHistory>(Option::None, path, headers, HttpMethod::GET)
            .await;
    match response {
        Ok(res) => {
            // log::info!("ressa: {:?}", res);
            let mut new_data = res.data.clone();
            let mut positions_greeks = Vec::<PositionGreekRequest>::default();
            let mut res = res;
            for d in &mut *res.data {
                let ps = PositionGreekRequest::new(
                    d.side.clone(),
                    d.option_kind.clone().unwrap(),
                    d.amount.unwrap_or_default().abs(),
                    d.strike,
                    time_to_expiry(d.expiry_timestamp.as_str()),
                    d.px_in_quote_ccy.unwrap_or_default().abs(),
                    Some(spot),
                    d.r2,
                    Some(0.0),
                    d.iv,
                    Some(d.expiry_timestamp.clone()),
                    Some(d.id.to_string()),
                );
                positions_greeks.push(ps);
            }
            let positions_greeks_request = PositionsGreeksRequest {
                positions: positions_greeks,
                current_spot: spot,
                spot_bump: 0.05,
                bump_times: 3,
            };
            // let pricer_url = std::env::var("PRICERENGINEURL").unwrap();
            let pricer_url = option_env!("PRICERENGINEURL");
            let pricer_path = format!("{}/quote/greeks", pricer_url.unwrap_or_default());
            let mut pricer_headers = reqwest::header::HeaderMap::new();
            pricer_headers.insert(
                "Authorization",
                reqwest::header::HeaderValue::from_str(&bearer_token).unwrap(),
            );
            let resp = call_and_parse::<PositionsGreeksRequest, PositionsGreeksResponse>(
                Some(positions_greeks_request),
                pricer_path,
                pricer_headers,
                HttpMethod::POST,
            )
            .await;
            match resp {
                Ok(r) => {
                    // log::info!("res: {:?}", r);
                    for p in &r.data.positions {
                        let req_id = p.req_id.clone().unwrap_or("0".to_string());
                        if let Some(v) = new_data
                            .iter_mut()
                            .find(|x| x.id == req_id.parse::<u32>().unwrap())
                        {
                            v.live_pnl_percentage = Some(p.pnl_percentage);
                            v.pnl = Some(p.pnl);
                        }
                    }
                    Ok(TradeHistory { data: new_data })
                }
                Err(e) => {
                    log::error!("error: {:?}", e);
                    Err(ServerFnError::new(e.to_string()))
                }
            }
        }
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
    // Err(ServerFnError::new(e.to_string()))
}