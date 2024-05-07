use leptos::*;
use crate::utilities::{
    cookies::{get_jabra_cookie, JabraCookie},
    http_wrapper::{call_and_parse, HttpMethod},
};

use crate::commons::models::defaults::BlankRequest;
use crate::commons::models::currency::{Currency, CurrencyConfigurationResponse};
/// Server function to fetch the currencies.

pub async fn fetch_currencies() -> Result<CurrencyConfigurationResponse, ServerFnError> {
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!(
        "{}/items/supported_ccy?fields={}",
        url.unwrap_or_default(),
        Currency::get_default_query()
    );
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, CurrencyConfigurationResponse>(
        Option::None,
        path,
        headers,
        HttpMethod::GET,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}