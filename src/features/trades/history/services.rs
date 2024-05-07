use leptos::ServerFnError;
use crate::commons::models::defaults::BlankRequest;
use crate::commons::models::trade::{Trade, TradeHistory};
use crate::utilities::cookies::{get_jabra_cookie, JabraCookie};
use crate::utilities::http_wrapper::{call_and_parse, HttpMethod};

pub async fn get_trade_history() -> Result<TradeHistory, ServerFnError> {
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    if jwt_cookie.is_expired() {
        return Ok(Default::default());
    }
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/trade?filter[counterparty_id][ticker][_eq]={}&sort[]=-date_created&filter[party_a][_neq]=null&filter[party_b][_neq]=null&fields={}&limit=100", url.unwrap_or_default(), String::from("JABRA"), Trade::get_query());
    // log::info!("path: {:?}", path);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response =
        call_and_parse::<BlankRequest, TradeHistory>(Option::None, path, headers, HttpMethod::GET)
            .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}
