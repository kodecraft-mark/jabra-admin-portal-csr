use serde::{Deserialize, Serialize};

/// This struct is used to get the details of a counterparty.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CounterParty {
    pub id: u16,
    pub ticker: String,
    pub name: String,
    pub short_name: Option<String>,
    pub is_exchange: bool,
}

impl CounterParty {
    pub fn get_query(key: &str) -> String {
        format!(
            "{}.id, {}.ticker, {}.name, {}.short_name, {}.is_exchange",
            key, key, key, key, key
        )
    }
}

/// This struct is the response of the [`get_counter_parties`] server function.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetCounterPartiesResponse {
    pub data: Vec<CounterParty>,
}

impl GetCounterPartiesResponse {
    pub fn get_counterparty_by_name(&self, name: &str) -> Option<&CounterParty> {
        self.data.iter().find(|cp| cp.name == name)
    }
}
