
use leptos::*;

use crate::commons::models::defaults::BlankRequest;
use crate::commons::models::trade::{Trade, TradeHistory};
use crate::utilities::cookies::{get_jabra_cookie, refresh_token, set_jabra_cookie, JabraCookie};
use crate::utilities::http_wrapper::{call, call_and_parse, HttpMethod};
use super::models::TradeForModification;

use std::collections::HashMap;

pub async fn fetch_recent_trades() -> Result<HashMap<String, Vec<Trade>>, ServerFnError> {

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    if jwt_cookie.is_expired() {
        return Ok(Default::default());
    }
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/trade?filter[party_b][_null]=false&filter[party_a][_null]=false&filter[has_termsheet][_eq]=false&limit=-1&filter[expiry_timestamp][_gte]=$NOW()&filter[party_a][ticker][_eq]=JABRA&filter[trade_type][_eq]=trade&fields={}", url.unwrap_or_default(), Trade::get_query());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response =
        call_and_parse::<BlankRequest, TradeHistory>(Option::None, path, headers, HttpMethod::GET)
            .await;
    match response {
        Ok(res) => {
            let mut trade_quotes_map: HashMap<String, Vec<Trade>> = HashMap::new();
            for trade_quote in res.data {
                let key = format!("{}~{}", trade_quote.party_b.name, trade_quote.party_b.id);
                trade_quotes_map
                    .entry(key)
                    .or_insert(vec![])
                    .push(trade_quote);
            }
            Ok(trade_quotes_map)
        }
        Err(e) => {
            log::info!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

pub async fn edit_trade(request: TradeForModification) -> Result<bool, ServerFnError> {
    log::info!("request: {:?}", request);

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let mut bearer = format!("Bearer {}", jwt_cookie.access_token);

    // Check if token expires, this checking will be available only to actions and server action
    // Other resources will still work due to 10 minutes buffer time
    if jwt_cookie.is_expired() {
        let refresh = refresh_token(jwt_cookie.user_id, jwt_cookie.refresh_token).await;
        match refresh {
            Ok(r) => {
                bearer = format!("Bearer {}", r.access_token);
                set_jabra_cookie(r, "admin_portal_csr".to_string()).await;
            }
            Err(e) => {
                log::error!("error-token: {:?}", e);
                // directus_wrapper::set_jabra_cookie(None, "admin_portal_csr".to_string()).await;
                return Err(ServerFnError::ServerError(e.to_string()));
            }
        }
    }
    // log::debug!("request: {:?}", request.deserialize());
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/trade", url.unwrap_or_default());

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response =
        call::<TradeForModification>(Some(request), path, headers, HttpMethod::PATCH).await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::info!("error-: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
    // // Err(ServerFnError::ServerError("Transaction failed".to_string()))
    // log::info!("request: {:?}", request);
    // Ok(true)
}