use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CoinbaseSpotPriceResponse {
    pub data: CoinbaseSpotPriceResponseData,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CoinbaseSpotPriceResponseData {
    pub amount: String,
    pub base: String,
    pub currency: String,
}