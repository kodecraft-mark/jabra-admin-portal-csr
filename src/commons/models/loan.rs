use serde::{Deserialize, Serialize};

use crate::{commons::models::{currency::Currency, currency_pair::CurrencyPair, user::User}, utilities::date_util::convert_utc_to_local};

use super::counterparty::CounterParty;

/// Struct for a single Loan.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Loan {
    pub id: i32,
    pub date_created: String,
    pub loan_to_value: f64,
    pub interest_rate: f64,
    pub trade_date: String,
    pub reference_rate: u32,
    pub base_ccy_amount: f64,
    pub initial_exchange_amount: u32,
    pub transaction_type: String,
    pub status: String,
    pub user_created: User,
    pub counterparty_id: CounterParty,
    pub pair_id: CurrencyPair,
    pub base_ccy_id: Currency,
    pub term_ccy_id: Currency,
}

/// Struct for the loan history data.

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetLoanHistory {
    pub data: Vec<Loan>,
}

/// Struct for the data that can be shown in the data table.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ExtractedLoan {
    pub date_created: String,
    pub loan_to_value: f64,
    pub interest_rate: f64,
    pub trade_date: String,
    pub reference_rate: u32,
    pub base_ccy_amount: f64,
    pub initial_exchange_amount: u32,
    pub transaction_type: String,
    pub status: String,
    pub currency_pair: String,
    pub base_ccy: String,
    pub term_ccy: String,
}

impl GetLoanHistory {
    pub fn extract(&self) -> Vec<ExtractedLoan> {
        self.data
            .iter()
            .map(|t| ExtractedLoan {
                date_created: convert_utc_to_local(&t.date_created.clone()),
                loan_to_value: t.loan_to_value,
                interest_rate: t.interest_rate,
                trade_date: t.trade_date.clone(),
                reference_rate: t.reference_rate.clone(),
                base_ccy_amount: t.base_ccy_amount.clone(),
                initial_exchange_amount: t.initial_exchange_amount.clone(),
                transaction_type: t.transaction_type.clone(),
                status: t.status.clone(),
                currency_pair: t.pair_id.name.clone(),
                base_ccy: t.base_ccy_id.ticker.clone(),
                term_ccy: t.term_ccy_id.ticker.clone(),
            })
            .collect()
    }

    pub fn extract_active_loan(&self) -> Vec<ExtractedLoan> {
        self.data
            .iter()
            .filter(|t| t.status == "open")
            .map(|t| ExtractedLoan {
                date_created: convert_utc_to_local(&t.date_created.clone()),
                loan_to_value: t.loan_to_value,
                interest_rate: t.interest_rate,
                trade_date: t.trade_date.clone(),
                reference_rate: t.reference_rate.clone(),
                base_ccy_amount: t.base_ccy_amount.clone(),
                initial_exchange_amount: t.initial_exchange_amount.clone(),
                transaction_type: t.transaction_type.clone(),
                status: t.status.clone(),
                currency_pair: t.pair_id.name.clone(),
                base_ccy: t.base_ccy_id.ticker.clone(),
                term_ccy: t.term_ccy_id.ticker.clone(),
            })
            .collect()
    }
    pub fn extract_csv_by_loan_status(&self, loan_status: Option<String>, reverse: bool) -> String {
        let data = match loan_status {
            Some(value) => self.extract(),
            None => self.extract(),
        };
        let mut content = String::new();
        content.push_str("Date Created, Currency Pair, Interest Rate, Loan To Value, Reference Rate, Base CCY Amount, Initial Exchange Amount, Transaction Type, Status\n");
        for i in data {
            let line = format!("{},{},{},{},{},{},{},{},{}\n", i.date_created, i.currency_pair, i.interest_rate, i.loan_to_value, i.reference_rate, i.base_ccy_amount, i.initial_exchange_amount, i.transaction_type, i.status);
            content.push_str(&line.as_str());
        }
        content
    }
}

/// Function for sorting the data table.

pub fn sort(
    mut data_table: Vec<ExtractedLoan>,
    sort_type: bool,
    sort_by: String,
) -> Vec<ExtractedLoan> {
    match sort_by.to_uppercase().as_str() {
        "DATE CREATED" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.date_created.cmp(&b.date_created));
            }
            false => {
                data_table.sort_by(|a, b| b.date_created.cmp(&a.date_created));
            }
        },
        "LOAN TO VALUE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.loan_to_value.partial_cmp(&b.loan_to_value).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.loan_to_value.partial_cmp(&a.loan_to_value).unwrap());
            }
        },
        "INTEREST RATE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.interest_rate.partial_cmp(&b.interest_rate).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.interest_rate.partial_cmp(&a.interest_rate).unwrap());
            }
        },
        "TRADE DATE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.trade_date.cmp(&b.trade_date));
            }
            false => {
                data_table.sort_by(|a, b| b.trade_date.cmp(&a.trade_date));
            }
        },
        "REFERENCE RATE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.reference_rate.cmp(&b.reference_rate));
            }
            false => {
                data_table.sort_by(|a, b| b.reference_rate.cmp(&a.reference_rate));
            }
        },
        "BASE CCY AMOUNT" => match sort_type {
            true => {
                data_table
                    .sort_by(|a, b| a.base_ccy_amount.partial_cmp(&b.base_ccy_amount).unwrap());
            }
            false => {
                data_table
                    .sort_by(|a, b| b.base_ccy_amount.partial_cmp(&a.base_ccy_amount).unwrap());
            }
        },
        "INITIAL EXCHANGE AMOUNT" => match sort_type {
            true => {
                data_table
                    .sort_by(|a, b| a.initial_exchange_amount.cmp(&b.initial_exchange_amount));
            }
            false => {
                data_table
                    .sort_by(|a, b| b.initial_exchange_amount.cmp(&a.initial_exchange_amount));
            }
        },
        "TRANSACTION TYPE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.transaction_type.cmp(&b.transaction_type));
            }
            false => {
                data_table.sort_by(|a, b| b.transaction_type.cmp(&a.transaction_type));
            }
        },
        "STATUS" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.status.cmp(&b.status));
            }
            false => {
                data_table.sort_by(|a, b| b.status.cmp(&a.status));
            }
        },
        "TERM CCY" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.term_ccy.cmp(&b.term_ccy));
            }
            false => {
                data_table.sort_by(|a, b| b.term_ccy.cmp(&a.term_ccy));
            }
        },
        "CURRENCY PAIR" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.currency_pair.cmp(&b.currency_pair));
            }
            false => {
                data_table.sort_by(|a, b| b.currency_pair.cmp(&a.currency_pair));
            }
        },
        "BASE CCY" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.base_ccy.cmp(&b.base_ccy));
            }
            false => {
                data_table.sort_by(|a, b| b.base_ccy.cmp(&a.base_ccy));
            }
        },
        _ => (),
    }
    data_table
}
impl Loan {
    pub fn get_query() -> String {
        format!(
            "id, date_created, loan_to_value, interest_rate, trade_date, reference_rate, base_ccy_amount, initial_exchange_amount, transaction_type, status, {}, {}, {}, {}, {}",
            User::get_query("user_created"),
            CounterParty::get_query("counterparty_id"),
            CurrencyPair::get_query("pair_id"),
            Currency::get_query("base_ccy_id"),
            Currency::get_query("term_ccy_id"),
        )
    }
}