use std::collections::BTreeMap;

use crate::utilities::{date_util::format_utc_str_to_local_str, number_util::format_currency};

use super::{
    counterparty::CounterParty, currency::Currency, currency_pair::CurrencyPair, user::User,
};
use leptos::*;
use serde::{Deserialize, Serialize};

/// This enum represents the menu tabs in the quotes page.

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuotesTab {
    Active,
    Approved,
    Rejected,
}

/// This struct is used to get the quote option details.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuoteOption {
    pub id: u32,
    pub date_created: String,
    pub quote_id: String,
    pub amount: f64,
    pub option_kind: String,
    pub r1: f64,
    pub r2: f64,
    pub offstrike_percentage: f64,
    pub strike: f64,
    pub iv: f64,
    pub px_in_base_ccy: f64,
    pub px_in_quote_ccy: f64,
    pub side: String,
    pub quote_expiry: String,
    pub expiry_timestamp: Option<String>,
    pub modified_date: String,
    pub quote_status: String,
    pub instrument_name: String,
    pub spot: f64,
    pub ttm: f64,
    pub gtc: bool,
    pub group_id: String,
    pub delta: Option<f64>,
    pub gamma: Option<f64>,
    pub theta: Option<f64>,
    pub payout_ccy: Option<String>,
    pub user_created: User,
    pub pair_id: CurrencyPair,
    pub ccy_id: Currency,
    pub counterparty_id: CounterParty,
    pub party_a: Option<CounterParty>,
    pub party_b: Option<CounterParty>,
}

impl QuoteOption {
    pub fn get_query() -> String {
        format!(
            "id, date_created, quote_id, amount, option_kind, r1, r2, offstrike_percentage, strike, iv, px_in_base_ccy, px_in_quote_ccy, side, quote_expiry, expiry_timestamp, modified_date, quote_status, delta, instrument_name, spot, ttm, gtc, group_id, gamma, theta, payout_ccy, gtc, {}, {}, {}, {}, {}, {}",
            User::get_query("user_created"),
            CurrencyPair::get_query("pair_id"),
            Currency::get_query("ccy_id"),
            CounterParty::get_query("counterparty_id"),
            CounterParty::get_query("party_a"),
            CounterParty::get_query("party_b"),
        )
    }
}

/// This is the response struct for the [`get_quotes_option`] server function.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuoteOptionResponse {
    pub data: Vec<QuoteOption>,
}

/// This struct is used to handle all quotes option status change requests.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesOptionForStatusChange {
    pub id: u32,
    pub quote_status: String,
    pub modified_date: String,
}

impl QuotesOptionForStatusChange {
    pub fn new(id: u32, quote_status: String, modified_date: String) -> Self {
        Self {
            id,
            quote_status,
            modified_date,
        }
    }
}

/// This struct is used to handle all quotes option modification requests.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotesOptionsForModification {
    pub id: u32,
    pub amount: f64,
    pub counterparty_id: u16,
    pub px_in_base_ccy: f64,
    pub px_in_quote_ccy: f64,
    pub quote_expiry: String,
    pub payout_ccy: Option<String>,
    pub party_a: u16,
    pub party_b: u16,
    pub gtc: bool,
}
impl QuotesOptionsForModification {
    pub fn new(
        id: u32,
        amount: f64,
        counterparty_id: u16,
        px_in_base_ccy: f64,
        px_in_quote_ccy: f64,
        quote_expiry: String,
        payout_ccy: Option<String>,
        party_a: u16,
        party_b: u16,
        gtc: bool,
    ) -> Self {
        Self {
            id,
            amount,
            counterparty_id,
            px_in_base_ccy,
            px_in_quote_ccy,
            quote_expiry,
            payout_ccy,
            party_a,
            party_b,
            gtc,
        }
    }
}

/// This struct is used for the response when a quote is modified.
/// It is used to display the success or failure message modal.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ModifyQuoteResponse {
    pub success: bool,
    pub message: String,
}

impl Default for ModifyQuoteResponse {
    fn default() -> Self {
        Self {
            success: false,
            message: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApproveTradeQuoteResponse {
    pub success: bool,
    pub message: String,
}

impl Default for ApproveTradeQuoteResponse {
    fn default() -> Self {
        Self {
            success: false,
            message: "".to_string(),
        }
    }
}

/// Response Struct of TradeHistory Request, return a vector of Trade Struct.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuoteOptionHistory {
    pub data: Vec<QuoteOption>,
}

impl QuoteOptionHistory {
    /// Extract Trade Struct into a Vector that can be shown in the data Table.
    pub fn extract(&self) -> Vec<ExtractedQuoteOption> {
        self.data
            .iter()
            .map(|t| ExtractedQuoteOption {
                id: t.id,
                market: t.instrument_name.clone(),
                status: t.quote_status.clone(),
                side: t.side.clone(),
                kind: t.option_kind.clone(),
                trans_type: String::from("Option"),
                size: format_currency(t.amount, t.pair_id.base.display_scale.clone()),
                price: match t.payout_ccy.clone() {
                    Some(p) => {
                        if p == "base" {
                            format_currency(
                                t.px_in_base_ccy.clone(),
                                t.pair_id.base.display_scale.clone(),
                            )
                        } else {
                            format_currency(
                                t.px_in_quote_ccy.clone(),
                                t.pair_id.quote.display_scale.clone(),
                            )
                        }
                    }
                    None => format_currency(
                        t.px_in_quote_ccy.clone(),
                        t.pair_id.quote.display_scale.clone(),
                    ),
                },
                group_id: t.group_id.clone(),
                expiration_date: match t.expiry_timestamp.clone() {
                    Some(d) => Some(format_utc_str_to_local_str(d)),
                    _ => Some(String::from("N/A")),
                },
                date_created: format_utc_str_to_local_str(t.date_created.clone()),
                premium_ccy: match t.payout_ccy.clone() {
                    Some(p) => {
                        if p == "base" {
                            t.pair_id.name.split("/").collect::<Vec<&str>>()[0].to_string()
                        } else {
                            t.pair_id.name.split("/").collect::<Vec<&str>>()[1].to_string()
                        }
                    }
                    None => t.pair_id.name.split("/").collect::<Vec<&str>>()[1].to_string(),
                },
            })
            .collect()
    }
    pub fn extract_csv_by_quote_status(&self, quote_status: Option<String>, reverse: bool) -> String {
        let data = match quote_status {
            Some(value) => self.extract(),
            None => self.extract(),
        };
        let mut content = String::new();
        content.push_str("Date Created, Market, Status, Side, Kind, Type, Size, Price, Price Currency\n");
        for i in data {
            let line = format!("{},{},{},{},{},{},{},{},{}\n", i.date_created, i.market, i.status, i.side, i.kind, i.trans_type, i.size, i.price, i.premium_ccy);
            content.push_str(&line.as_str());
        }
        content
    }
}

/// Struct for the data that can be shown in the data table.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ExtractedQuoteOption {
    pub id: u32,
    pub market: String,
    pub status: String,
    pub side: String,
    pub kind: String,
    pub trans_type: String,
    pub size: String,
    pub price: String,
    pub group_id: String,
    pub expiration_date: Option<String>,
    pub date_created: String,
    pub premium_ccy: String,
}
