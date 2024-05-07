use leptos::*;

use crate::commons::models::counterparty::GetCounterPartiesResponse;
use crate::commons::models::defaults::BlankRequest;
use crate::utilities::cookies::JabraCookie;
use crate::utilities::http_wrapper::{call_and_parse, HttpMethod};
/// Server function to get the counterparties.

pub async fn get_counter_parties() -> Result<GetCounterPartiesResponse, ServerFnError> {
    let (cookie, _set_cookie) =
        leptos_use::use_cookie::<String, leptos_use::utils::FromToStringCodec>("admin_portal_csr");
    let cookie = cookie.get_untracked().map(|c| c).unwrap_or_default();

    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    if jwt_cookie.is_expired() {
        return Ok(super::counterparty::GetCounterPartiesResponse::default());
    }
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/counterparty?sort=name", url.unwrap_or_default());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, GetCounterPartiesResponse>(
        Option::None,
        path,
        headers,
        HttpMethod::GET,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::error!("error6: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}
