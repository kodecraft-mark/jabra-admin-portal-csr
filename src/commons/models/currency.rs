use serde::{Deserialize, Serialize};

/// Struct used to represent the base/quote currency.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Currency {
    pub id: u16,
    pub ticker: String,
    pub name: String,
    pub is_active: bool,
    pub display_scale: u8,
    pub sign: Option<String>,
}

impl Currency {
    pub fn get_query(key: &str) -> String {
        format!(
            "{}.id, {}.ticker, {}.name, {}.is_active, {}.display_scale, {}.sign",
            key, key, key, key, key, key
        )
    }
    pub fn get_default_query() -> String {
        format!("id, ticker, name, is_active, display_scale, sign")
    }
}

/// Struct for the Currency Configuration Response.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CurrencyConfigurationResponse {
    pub data: Vec<Currency>,
}
