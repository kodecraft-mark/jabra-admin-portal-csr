use serde::{Deserialize, Serialize};

use super::currency::Currency;

/// This struct is used to get the currency pair details.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CurrencyPair {
    pub id: u16,
    pub name: String,
    pub is_active: bool,
    pub base: Currency,
    pub quote: Currency,
}
impl CurrencyPair {
    pub fn get_query(key: &str) -> String {
        let base_currency = Currency::get_query(format!("{}.base", key).as_str());
        let quote_currency = Currency::get_query(format!("{}.quote", key).as_str());
        format!(
            "{}.id, {}.name, {}.is_active, {}, {}",
            key, key, key, base_currency, quote_currency
        )
    }
}
