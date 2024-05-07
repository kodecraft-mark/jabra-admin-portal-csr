use crate::{
    commons::models::{
        defaults::BlankRequest,
        quote::{
            GetQuoteOptionResponse, QuoteOption, QuotesOptionForStatusChange,
            QuotesOptionsForModification,
        },
    },
    utilities::{
        cookies::{refresh_token, set_jabra_cookie, JabraCookie},
        http_wrapper::{call, call_and_parse, HttpMethod},
    },
};
use leptos::*;

/// This is a server function that gets the quotes option based on the quote status.
/// The input string variations are `active`, `approved`, `rejected`, and `expired`.

pub async fn get_quotes_option(
    quote_status: String,
) -> Result<std::collections::BTreeMap<String, Vec<QuoteOption>>, ServerFnError> {
    use std::collections::BTreeMap;
    let (cookie, _set_cookie) =
        leptos_use::use_cookie::<String, leptos_use::utils::FromToStringCodec>("admin_portal_csr");
    let cookie = cookie.get_untracked().map(|c| c).unwrap_or_default();
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    // let path = format!("{}/items/quotes_option?filter[quote_status][_eq]={}&filter[modified_date][_between]=[{}, {}]&fields={}", url, quote_status, QuoteOption::get_query());
    let path = format!(
        "{}/items/quotes_option?filter[quote_status][_eq]={}&fields={}",
        url.unwrap_or_default(),
        quote_status,
        QuoteOption::get_query()
    );
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, GetQuoteOptionResponse>(
        Option::None,
        path,
        headers,
        HttpMethod::GET,
    )
    .await;

    match response {
        Ok(res) => {
            let mut trade_quotes_map: BTreeMap<String, Vec<QuoteOption>> = BTreeMap::new();
            let mut admin_trade_quotes: Vec<QuoteOption> = Vec::<QuoteOption>::default();
            for trade_quote in res.data {
                if trade_quote.counterparty_id.ticker != "JABRA" {
                    let key = format!(
                        "{}~{}",
                        trade_quote.counterparty_id.name, trade_quote.counterparty_id.id
                    );
                    trade_quotes_map
                        .entry(key)
                        .or_insert(vec![])
                        .push(trade_quote);
                } else {
                    admin_trade_quotes.push(trade_quote);
                }
            }

            let mut admin_trade_quotes_map = BTreeMap::<String, Vec<QuoteOption>>::new();
            //Iterate Over BTreeMap
            for (_, value) in trade_quotes_map.iter_mut() {
                //Iterate over Vector in Traders Quotes
                for trade_quote in value.iter_mut() {
                    //Iterate over Vector in Admin Quotes
                    for tq in admin_trade_quotes.iter() {
                        if trade_quote.group_id == tq.group_id {
                            let key = format!(
                                "{}~{}",
                                trade_quote.counterparty_id.name, trade_quote.counterparty_id.id
                            );
                            admin_trade_quotes_map
                                .entry(key)
                                .or_insert(vec![])
                                .push(tq.clone());
                        }
                    }
                }
            }

            for (key, value) in admin_trade_quotes_map.iter_mut() {
                if trade_quotes_map.contains_key(key) {
                    let v = trade_quotes_map.get_mut(key).unwrap();
                    v.append(value);
                } else {
                    trade_quotes_map.insert(key.clone(), value.clone());
                }
            }
            // log::info!("trade_quotes_map: {:?}", trade_quotes_map);
            Ok(trade_quotes_map)
        }
        Err(e) => {
            log::info!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// This is a server function that gets the quotes option based on the quote status and date range.
/// The format for the date is `%Y-%m-%dT%H:%M:%S%.3fZ`.

pub async fn get_quotes_option_under_24_hrs(
    quote_status: String,
) -> Result<std::collections::BTreeMap<String, Vec<QuoteOption>>, ServerFnError> {
    use std::collections::BTreeMap;

    let (cookie, _set_cookie) =
        leptos_use::use_cookie::<String, leptos_use::utils::FromToStringCodec>("admin_portal_csr");
    let cookie = cookie.get_untracked().map(|c| c).unwrap_or_default();
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");

    let start_date = (chrono::Utc::now() - chrono::Duration::hours(24))
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string();
    let end_date = chrono::Utc::now()
        .format("%Y-%m-%dT%H:%M:%S%.3fZ")
        .to_string();

    let path = format!("{}/items/quotes_option?filter[quote_expiry][_nnull]=true&filter[quote_status][_eq]={}&filter[modified_date][_between]=[{}, {}]&fields={}", url.unwrap_or_default(), quote_status, start_date, end_date, QuoteOption::get_query());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, GetQuoteOptionResponse>(
        Option::None,
        path,
        headers,
        HttpMethod::GET,
    )
    .await;

    match response {
        Ok(res) => {
            let mut trade_quotes_map: BTreeMap<String, Vec<QuoteOption>> = BTreeMap::new();
            let mut admin_trade_quotes: Vec<QuoteOption> = Vec::<QuoteOption>::default();
            for trade_quote in res.data {
                if trade_quote.counterparty_id.ticker != "JABRA" {
                    let key = format!(
                        "{}~{}",
                        trade_quote.counterparty_id.name, trade_quote.counterparty_id.id
                    );
                    trade_quotes_map
                        .entry(key)
                        .or_insert(vec![])
                        .push(trade_quote);
                } else {
                    admin_trade_quotes.push(trade_quote);
                }
            }

            let mut admin_trade_quotes_map = BTreeMap::<String, Vec<QuoteOption>>::new();
            //Iterate Over BTreeMap
            for (_, value) in trade_quotes_map.iter_mut() {
                //Iterate over Vector in Traders Quotes
                for trade_quote in value.iter_mut() {
                    //Iterate over Vector in Admin Quotes
                    for tq in admin_trade_quotes.iter() {
                        if trade_quote.group_id == tq.group_id {
                            let key = format!(
                                "{}~{}",
                                trade_quote.counterparty_id.name, trade_quote.counterparty_id.id
                            );
                            admin_trade_quotes_map
                                .entry(key)
                                .or_insert(vec![])
                                .push(tq.clone());
                        }
                    }
                }
            }

            for (key, value) in admin_trade_quotes_map.iter_mut() {
                if trade_quotes_map.contains_key(key) {
                    let v = trade_quotes_map.get_mut(key).unwrap();
                    v.append(value);
                } else {
                    trade_quotes_map.insert(key.clone(), value.clone());
                }
            }
            // log::info!("trade_quotes_map: {:?}", trade_quotes_map);
            Ok(trade_quotes_map)
        }
        Err(e) => {
            log::info!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// This is a server function that approves or rejects a quote option.
/// ## Examples
///
/// ```rust
/// approve_reject_quotes_option(vec![QuotesOptionForStatusChange::new(1, "approved".to_string())]) {
///     Ok(true)
/// };
/// ```
///
/// ```rust
/// approve_reject_quotes_option(vec![QuotesOptionForStatusChange::new(2, "rejected".to_string())]) {
///     Ok(true)
/// };
/// ```

pub async fn approve_reject_quotes_option(
    request: Vec<QuotesOptionForStatusChange>,
) -> Result<bool, ServerFnError> {
    log::info!("request: {:?}", request);

    let (cookie, _set_cookie) =
        leptos_use::use_cookie::<String, leptos_use::utils::FromToStringCodec>("admin_portal_csr");
    let cookie = cookie.get_untracked().map(|c| c).unwrap_or_default();

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
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/quotes_option", url.unwrap_or_default());

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response =
        call::<Vec<QuotesOptionForStatusChange>>(Some(request), path, headers, HttpMethod::PATCH)
            .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::info!("error-: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
    // Err(ServerFnError::ServerError("Transaction failed".to_string()))
    // log::info!("request: {:?}", request);
    // Ok(false)
}

/// This is a server function that modifies a quote option.
/// It accepts a vector of [QuotesOptionsForModification] as input.

pub async fn edit_quotes_option(
    request: Vec<QuotesOptionsForModification>,
) -> Result<bool, ServerFnError> {
    log::info!("request: {:?}", request);

    let (cookie, _set_cookie) =
        leptos_use::use_cookie::<String, leptos_use::utils::FromToStringCodec>("admin_portal_csr");
    let cookie = cookie.get_untracked().map(|c| c).unwrap_or_default();

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
    let path = format!("{}/items/quotes_option", url.unwrap_or_default());

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response =
        call::<Vec<QuotesOptionsForModification>>(Some(request), path, headers, HttpMethod::PATCH)
            .await;
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
