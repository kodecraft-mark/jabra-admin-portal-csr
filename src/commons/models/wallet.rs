use serde::{Deserialize, Serialize};

use crate::{commons::models::user::User, utilities::{date_util::format_utc_str_to_local_str, number_util::format_currency}};

use super::{counterparty::CounterParty, currency::Currency};



/// Response Struct of TradeHistory Request, return a vector of Trade Struct.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransactionHistory {
    pub data: Vec<WalletTransaction>,
}

impl WalletTransactionHistory {
    /// Extract Trade Struct into a Vector that can be shown in the data Table.
    pub fn extract(&self, action: String) -> Vec<ExtractedWalletTransaction> {
        self.data
            .iter()
            .filter(|t| action == "ALL" || t.transaction_type.to_uppercase() == action.clone())
            .map(|t| ExtractedWalletTransaction {
                id: t.id,
                action: t.transaction_type.clone(),
                amount: format_currency(t.amount, t.currency_id.display_scale),
                currency: t.currency_id.ticker.clone(),
                time: format_utc_str_to_local_str(t.venue_transaction_datetime.clone()),
                fee_amount: format_currency(t.fee_amount, t.currency_id.display_scale),
                description: t.description.clone(),
            })
            .collect()
    }
}

/// Struct for the data that can be shown in the data table.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ExtractedWalletTransaction {
    pub id: u32,
    pub action: String,
    pub amount: String,
    pub currency: String,
    pub time: String,
    pub fee_amount: String,
    pub description: String,
}
/// Struct for the wallet transaction data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletTransaction {
    pub id: u32,
    pub user_created: User,
    pub date_created: String,
    pub currency_id: Currency,
    pub amount: f64,
    pub txn_hash: String,
    pub transaction_type: String,
    pub fee_amount: f64,
    pub venue_transaction_datetime: String,
    pub description: String,
    pub is_submitted: bool,
    pub reference: String,
    pub counterparty_id: CounterParty,
}

impl WalletTransaction {
    pub fn get_query() -> String {
        format!(
            "id, date_created, amount, txn_hash, transaction_type, fee_amount, venue_transaction_datetime, description, reference, is_submitted, {}, {}, {}",
            Currency::get_query("currency_id"),
            CounterParty::get_query("counterparty_id"),
            User::get_query("user_created")
        )
    }
}

/// Function for Sorting the data table.
pub fn sort(
    mut data_table: Vec<ExtractedWalletTransaction>,
    sort_type: bool,
    sort_by: String,
) -> Vec<ExtractedWalletTransaction> {
    match sort_by.to_uppercase().as_str() {
        "ID" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.id.cmp(&b.id));
            }
            false => {
                data_table.sort_by(|a, b| b.id.cmp(&a.id));
            }
        },
        "ACTION" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.action.cmp(&b.action));
            }
            false => {
                data_table.sort_by(|a, b| b.action.cmp(&a.action));
            }
        },
        "AMOUNT" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.amount.partial_cmp(&b.amount).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.amount.partial_cmp(&a.amount).unwrap());
            }
        },
        "CURRENCY" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.currency.cmp(&b.currency));
            }
            false => {
                data_table.sort_by(|a, b| b.currency.cmp(&a.currency));
            }
        },
        "FEE AMOUNT" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.fee_amount.partial_cmp(&b.fee_amount).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.fee_amount.partial_cmp(&a.fee_amount).unwrap());
            }
        },
        "TIME" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.time.cmp(&b.time));
            }
            false => {
                data_table.sort_by(|a, b| b.time.cmp(&a.time));
            }
        },
        "DESCRIPTION" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.description.cmp(&b.description));
            }
            false => {
                data_table.sort_by(|a, b| b.description.cmp(&a.description));
            }
        },
        _ => (),
    }
    data_table
}