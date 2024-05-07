use leptos::ServerFnError;

use crate::{commons::models::{defaults::BlankRequest, trade::{Trade, TradeHistory}}, utilities::{cookies::{get_jabra_cookie, JabraCookie}, http_wrapper::{call_and_parse, HttpMethod}}};


pub async fn get_trade_history() -> Result<TradeHistory, ServerFnError> {

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/trade?filter[counterparty_id][ticker][_eq]=JABRA&sort[]=-expiry_timestamp&filter[party_a][_nnull]=true&filter[party_b][_nnull]=true&filter[expiry_timestamp][_gte]=$NOW(-15)&filter[expiry_timestamp][_lte]=$NOW(7)&filter[trade_type][_eq]=trade&fields={}", url.unwrap_or_default(), Trade::get_query());
    //log::info!("path: {:?}", path);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<
        BlankRequest,
        TradeHistory,
    >(Option::None, path, headers, HttpMethod::GET)
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}
