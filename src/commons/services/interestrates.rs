use leptos::ServerFnError;
use crate::commons::models::defaults::BlankRequest;
use crate::commons::models::interestrates::{InterestRate, InterestRateRequest, InterestRatesResponse};
use crate::utilities::cookies::{get_jabra_cookie, JabraCookie};
use crate::utilities::http_wrapper::{call_and_parse,call, HttpMethod};

pub async fn get_interest_rates() -> Result<InterestRatesResponse, ServerFnError> {
    
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    if jwt_cookie.is_expired() {
        return Ok(Default::default());
    }
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/interest_rates?fields={}&sort=-id&limit=1", url.unwrap_or_default(), InterestRate::get_query());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, InterestRatesResponse>(
        Option::None,
        path,
        headers,
        HttpMethod::GET,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error fetching interest rates: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

pub async fn update_interest_rate(interest_rate: InterestRateRequest) -> Result<bool, ServerFnError> {

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    if jwt_cookie.is_expired() {
        return Ok(Default::default());
    }
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/interest_rates", url.unwrap_or_default());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call::<InterestRateRequest>(
        Some(interest_rate),
        path,
        headers,
        HttpMethod::POST,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error updating interest rates: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}