use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CreateDealResponse {
    pub success: bool,
    pub message: String,
}

impl Default for CreateDealResponse {
    fn default() -> Self {
        Self {
            success: false,
            message: "".to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deals {
    pub base_currency: String,
    pub qoute_currency: String,
    pub currency: String,
    pub expiry_in_days: f64,
    pub ccy1_amount: f64,
    pub ccy2_amount: f64,
    pub strike: f64,
    pub amount: f64,
    pub option_kind: String,
    pub spot: f64,
    pub r1: f64,
    pub r2: f64,
    pub iv_t1: f64,
    pub px_in_base_ccy: f64,
    pub px_in_quote_ccy: f64,
    pub jabra_side: String,
    pub expiry_timestamp: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeForModification {
    pub query: TradeQueryForModification,
    pub data: TradeDataForModification,
}
impl TradeForModification {
    pub fn new(query: TradeQueryForModification, data: TradeDataForModification) -> Self {
        Self { query, data }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeDataForModification {
    pub expiry_timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeFilterForModification {
    pub group_id: TradeGroupidForModification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeGroupidForModification {
    pub _eq: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeQueryForModification {
    pub filter: TradeFilterForModification,
}
/// This struct is used for the response when a quote is modified.
/// It is used to display the success or failure message modal.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyTradeResponse {
    pub success: bool,
    pub message: String,
}

impl Default for ModifyTradeResponse {
    fn default() -> Self {
        Self {
            success: false,
            message: "".to_string(),
        }
    }
}