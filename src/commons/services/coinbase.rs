use leptos::ServerFnError;

use crate::{commons::models::{coinbase::CoinbaseSpotPriceResponse, defaults::BlankRequest}, utilities::http_wrapper::{call_and_parse, HttpMethod}};

pub async fn get_spot_price(pair: String) -> Result<CoinbaseSpotPriceResponse, ServerFnError> {
    // let url = std::env::var("COINBASE_V2").unwrap();
    let url = option_env!("COINBASE_V2");
    let path = format!("{}/prices/{}/spot", url.unwrap_or_default(), pair);

    let call_and_parse = call_and_parse::<BlankRequest, CoinbaseSpotPriceResponse>(
        Option::None,
        path,
        reqwest::header::HeaderMap::default(),
        HttpMethod::GET,
    );
    let response = call_and_parse.await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::info!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}