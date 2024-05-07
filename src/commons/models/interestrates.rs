use serde::{Deserialize, Serialize};
use crate::commons::models::currency::Currency;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InterestRatesResponse {
    pub data: Vec<InterestRate>,
}

impl InterestRatesResponse {
    pub fn get_newest_interest_rate(&self) -> InterestRate {
        self.data.first().unwrap().clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InterestRate {
    pub rate: f64,
    pub currency_id: Currency
}
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InterestRateRequest {
    pub rate: f64,
    pub currency_id: u16
}

impl InterestRate {
    pub fn get_query() -> String {
        format!(
            "rate, {}", Currency::get_query("currency_id")
        )
    }
    pub fn get_request(&self) -> InterestRateRequest {
        InterestRateRequest {
            rate: self.rate,
            currency_id: self.currency_id.id
        }
    }

}