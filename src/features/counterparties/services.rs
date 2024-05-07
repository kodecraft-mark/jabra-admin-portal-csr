
use leptos::ServerFnError;

use crate::commons::models::defaults::BlankRequest;
use crate::commons::models::loan::GetLoanHistory;
use crate::commons::models::loan::Loan;
use crate::commons::models::quote::QuoteOption;
use crate::commons::models::quote::QuoteOptionHistory;
use crate::commons::models::trade::Trade;
use crate::commons::models::trade::TradeHistory;
use crate::commons::models::wallet::WalletTransaction;
use crate::commons::models::wallet::WalletTransactionHistory;
use crate::utilities::cookies::get_jabra_cookie;
use crate::utilities::cookies::JabraCookie;
use crate::utilities::http_wrapper::call_and_parse;
use crate::utilities::http_wrapper::HttpMethod;

use super::models::PortfolioOverviewResponse;

/// Server function for getting the trade history based on the ticker.

pub async fn get_trade_history(
    ticker: String,
) -> Result<TradeHistory, ServerFnError> {
    log::info!("Called: {:?}", ticker);

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/trade?filter[counterparty_id][ticker][_eq]={}&sort[]=-date_created&filter[party_a][_neq]=null&filter[party_b][_neq]=null&fields={}", url.unwrap_or_default(), ticker, Trade::get_query());
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

/// Server function for getting the transfers data based on the ticker.
pub async fn fetch_transfers_data(
    ticker: String,
) -> Result<WalletTransactionHistory, ServerFnError> {
    log::info!("Called: {:?}", ticker);

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/wallet_transaction?filter[counterparty_id][ticker][_eq]={}&sort[]=-date_created&fields={}", url.unwrap_or_default(), ticker, WalletTransaction::get_query());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, WalletTransactionHistory>(
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

/// Server function for getting the quote history based on the ticker.

pub async fn get_quote_history(
    ticker: String,
) -> Result<QuoteOptionHistory, ServerFnError> {
    log::info!("Called: {:?}", ticker);

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!(
        "{}/items/quotes_option?sort[]=-date_created&filter[quote_expiry][_neq]=null&filter[quote_status][_neq]=active&filter[counterparty_id][ticker][_eq]={}&filter[party_a][_neq]=null&filter[party_b][_neq]=null&fields={}",
        url.unwrap_or_default(),
        ticker,
        QuoteOption::get_query()
    );
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, QuoteOptionHistory>(
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

/// Server function for getting loans based on the ticker.

pub async fn get_loans(
    ticker: String,
) -> Result<GetLoanHistory, ServerFnError> {
    log::info!("Called: {:?}", ticker);

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("DIRECTUSURL").unwrap();
    let url = option_env!("DIRECTUSURL");
    let path = format!(
        "{}/items/loan?sort[]=-date_created&filter[counterparty_id][ticker][_eq]={}&fields={}",
        url.unwrap_or_default(),
        ticker,
        Loan::get_query()
    );
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, GetLoanHistory>(
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

/// Server function to get the overview data.
/// It takes a ticker of type String as input and returns a [`PortfolioOverviewResponse`] as output.

pub async fn fetch_overview_data(
    ticker: String,
) -> Result<PortfolioOverviewResponse, ServerFnError> {
    log::info!("Called: {:?}", ticker);

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    // let url = std::env::var("WASMCLOUDURL").unwrap();
    let url = option_env!("WASMCLOUDURL");

    let path = format!(
        "{}/portfolios/summaries?counterparty={}&currency=USD",
        url.unwrap_or_default(), ticker
    );
    let mut headers = reqwest::header::HeaderMap::new();
    // log::info!("Bearer: {:?}", bearer.clone());
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    // let mut headers = reqwasm::http::Headers::new();
    // headers.append("Authorization", &bearer);

    // log::info!("Headers: {:?}", headers);

    // let mode =  reqwasm::http::RequestMode::NoCors;

    // let request = reqwasm::http::Request::get(&path).mode(mode).headers(headers).send().await;
    // let request = reqwasm::http::Request::get(&path).headers(headers).send().await;

    // let response = match request {
    //     Ok(res) => {
    //         if res.status() != 200 {
    //             // log::info!("Request !=200: {:?}", res);
    //             Err(crate::common::errors::JabraError::APIResponseError(res.status().to_string()))
    //         } else {
    //             // log::info!("Request: {:?}", res);
    //             let response = res.text().await.map_err(|e| crate::common::errors::JabraError::from(e)).unwrap_or_default();
    //             PortfolioOverviewResponse::de(&response).map_err(|e| crate::common::errors::JabraError::from(e))
    //         }
    //     }
    //     Err(e) => Err(crate::common::errors::JabraError::from(e))
    // };


    let response = call_and_parse::<BlankRequest, PortfolioOverviewResponse>(
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