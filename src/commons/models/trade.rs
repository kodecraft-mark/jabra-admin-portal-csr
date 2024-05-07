use leptos::ServerFnError;
use serde::{Deserialize, Serialize};

use super::{
    counterparty::CounterParty, currency::Currency, currency_pair::CurrencyPair, defaults::Greeks,
    user::User,
};
use crate::utilities::cookies::{get_jabra_cookie, refresh_token, set_jabra_cookie, JabraCookie};
use crate::utilities::date_util::{extract_date, format_utc_str_to_local_str, time_to_expiry};
use crate::utilities::http_wrapper::{call, HttpMethod};
use crate::utilities::number_util::format_currency;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RiskSlideTrade {
    pub id: u32,
    pub counterparty_name: String,
    pub instrument_name: String,
    pub spot: f64,
    pub strike: f64,
    pub option_kind: String,
    pub amount: f64,
    pub side: String,
    pub r2: f64,
    pub inception_price: f64,
    pub time_to_expiry: f64,
    pub iv: f64,
    pub current_price: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub pnl: f64,
    pub pnl_percentage: f64,
    pub last_updated: String,
    pub group_id: String,
    pub expiry_timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TradeHistoryWithSpot {
    pub data: Vec<Trade>,
    pub spot: f64,
}

impl TradeHistoryWithSpot {
    pub fn extract_risk_slide_positions_by_currency_pair(
        &self,
    ) -> Vec<RiskSlideTrade> {
        self.data
            .iter()
            .map(|t| RiskSlideTrade {
                id: t.id,
                counterparty_name: t.party_b.name.clone(),
                instrument_name: t.venue_instrument_name.clone(),
                spot: t.spot.unwrap_or(0.0),
                strike: t.strike,
                option_kind: t.option_kind.clone().unwrap_or_default(),
                // amount: t.amount.unwrap_or(0.0).abs(),
                amount: format_currency(t.amount.unwrap_or(0.0), t.pair_id.base.display_scale)
                    .parse::<f64>()
                    .unwrap()
                    .abs(),
                side: t.side.clone(),
                r2: t.r2.unwrap_or(0.0),
                inception_price: format_currency(
                    t.px_in_quote_ccy.unwrap_or_default(),
                    t.pair_id.quote.display_scale,
                )
                .parse::<f64>()
                .unwrap(),
                time_to_expiry: time_to_expiry(t.expiry_timestamp.clone().as_str()),
                iv: t.iv.unwrap_or(0.0),
                current_price: format_currency(
                    t.px_in_quote_ccy.unwrap_or_default(),
                    t.pair_id.quote.display_scale,
                )
                .parse::<f64>()
                .unwrap(),
                delta: 0.0,
                gamma: 0.0,
                theta: 0.0,
                pnl: t.pnl.unwrap_or(0.0),
                pnl_percentage: 0.0,
                last_updated: match t.date_updated.clone() {
                    Some(value) => format_utc_str_to_local_str(value),
                    None => format_utc_str_to_local_str(
                        t.date_created.clone().unwrap_or(String::from("")),
                    ),
                },
                group_id: t.group_id.clone(),
                expiry_timestamp: t.expiry_timestamp.clone(),
            })
            .collect()
    }

    pub fn extract_positions_for_risk_greek_request(
        &self,
        current_spot: f64,
        spot_bump: f64,
        bump_times: i32,
    ) -> PositionsGreeksRequest {
        let positions = self
            .data
            .iter()
            .map(|t| PositionGreekRequest {
                side: t.side.clone(),
                option_kind: t.option_kind.clone().unwrap_or_default(),
                amount: t.amount.unwrap_or_default(),
                strike: t.strike,
                ttm: time_to_expiry(t.expiry_timestamp.clone().as_str()),
                inception_price: t.px_in_quote_ccy.unwrap_or_default(),
                spot: t.spot,
                r2: t.r2,
                r1: t.r1,
                iv: t.iv,
                expiry: Some(t.expiry_timestamp.clone()),
                req_id: Some(t.id.to_string()),
            })
            .collect();
        PositionsGreeksRequest {
            positions,
            current_spot,
            spot_bump,
            bump_times,
        }
    }
}

/// Response Struct of TradeHistory Request, return a vector of Trade Struct.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TradeHistory {
    pub data: Vec<Trade>,
}

impl TradeHistory {
    /// Extract Trade Struct into a Vector that can be shown in the data Table.
    pub fn extract(&self) -> Vec<ExtractedTrade> {
        self.data
            .iter()
            .map(|t| {
                ExtractedTrade {
                    id: t.id,
                    market: t.venue_instrument_name.clone(),
                    side: t.side.clone(),
                    kind: t.option_kind.clone().map_or_else(
                        || String::from("- -"),
                        |value| {
                            if value.len() > 0 {
                                value
                            } else {
                                String::from("- -")
                            }
                        },
                    ),
                    trans_type: t.instrument_kind.clone(),
                    // size: format_currency(t.amount.unwrap_or(0.0), t.base_currency_id.display_scale),
                    size: match t.activity.to_uppercase() == "OPEN" {
                        true => format_currency(
                            t.amount.unwrap_or(0.0),
                            t.base_currency_id.display_scale,
                        ),
                        false => format_currency(
                            t.amount.unwrap_or(0.0).abs(),
                            t.base_currency_id.display_scale,
                        ),
                    },
                    price: match t.activity.to_uppercase() == "OPEN" {
                        true => match t.payout_ccy.clone() {
                            Some(p) => {
                                if p == "base" {
                                    format_currency(
                                        t.px_in_base_ccy.unwrap_or(0.0),
                                        t.base_currency_id.display_scale,
                                    )
                                } else {
                                    format_currency(
                                        t.px_in_quote_ccy.unwrap_or(0.0),
                                        t.quote_currency_id.display_scale,
                                    )
                                }
                            }
                            None => format_currency(
                                t.px_in_quote_ccy.unwrap_or(0.0),
                                t.quote_currency_id.display_scale,
                            ),
                        },
                        // If the instrument kind is equals to Spot, then show even if it was closed
                        false => match t.instrument_kind.to_uppercase() == "SPOT" {
                            true => format_currency(
                                t.px_in_quote_ccy.unwrap_or(0.0),
                                t.quote_currency_id.display_scale,
                            ),
                            false => String::from("- -"),
                        },
                    },
                    // price: match t.payout_ccy.clone()
                    //     {
                    //         Some(p) => {
                    //             if p == "base" {
                    //                 format_currency(t.px_in_base_ccy.unwrap_or(0.0), t.base_currency_id.display_scale)
                    //             } else {
                    //                 format_currency(t.px_in_quote_ccy.unwrap_or(0.0), t.quote_currency_id.display_scale)
                    //             }
                    //         },
                    //         None => {
                    //             format_currency(t.px_in_quote_ccy.unwrap_or(0.0), t.quote_currency_id.display_scale)
                    //     },
                    // },
                    // live_pnl: match t.pnl_snapshot_ccy.clone() {
                    //     Some(p) => {
                    //         if p == "base" {
                    //             format_currency(
                    //                 t.pnl_snapshot.clone().unwrap_or(0.0),
                    //                 t.base_currency_id.display_scale,
                    //             )
                    //         } else {
                    //             format_currency(
                    //                 t.pnl_snapshot.clone().unwrap_or(0.0),
                    //                 t.quote_currency_id.display_scale,
                    //             )
                    //         }
                    //     }
                    //     None => {
                    //         String::from("- -")
                    //         // format_currency(t.pnl_snapshot.clone().unwrap_or(0.0), t.quote_currency_id.display_scale)
                    //     }
                    // },
                    live_pnl: t.pnl.clone().unwrap_or(0.0).to_string(),
                    time: extract_date(t.expiry_timestamp.clone()),
                    currency: t.base_currency_id.ticker.clone(),
                    date_created: format_utc_str_to_local_str(
                        t.date_created.clone().unwrap_or(String::from("")),
                    ),
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
                    live_pnl_ccy: {
                        match t.pnl_snapshot_ccy.clone() {
                            Some(p) => {
                                if p == "base" {
                                    t.pair_id.name.split("/").collect::<Vec<&str>>()[0].to_string()
                                } else {
                                    t.pair_id.name.split("/").collect::<Vec<&str>>()[1].to_string()
                                }
                            }
                            None => String::from(""),
                        }
                    },
                    realized_pnl: match t.pnl_ccy.clone() {
                        Some(p) => {
                            if p == t.base_currency_id.ticker {
                                format_currency(
                                    t.pnl.clone().unwrap_or(0.0),
                                    t.base_currency_id.display_scale,
                                )
                            } else {
                                format_currency(
                                    t.pnl.clone().unwrap_or(0.0),
                                    t.quote_currency_id.display_scale,
                                )
                            }
                        }
                        None => {
                            String::from("- -")
                            // format_currency(t.pnl_snapshot.clone().unwrap_or(0.0), t.quote_currency_id.display_scale)
                        }
                    },
                    realized_pnl_ccy: t.pnl_ccy.clone().unwrap_or_default(),
                    trade_type: t.trade_type.clone(),
                    trade_status: match t.trade_status.clone() {
                        Some(value) => value,
                        None => String::from("- -"),
                    },
                    activity: t.activity.clone(),
                    side_status: format!("{} {}", t.activity.clone(), t.side.clone()),
                    index_price: t.index_price.clone().unwrap_or(0.0).to_string(),
                    party_a: t.party_a.name.clone(),
                    party_b: t.party_b.name.clone(),
                    pnl_percentage: t.live_pnl_percentage.clone().unwrap_or(0.0).to_string(),
                    last_updated: match &t.date_updated {
                        Some(d) => format_utc_str_to_local_str(d.clone()),
                        None => format_utc_str_to_local_str(
                            t.date_created.clone().unwrap_or_default(),
                        ),
                    },
                }
            })
            .collect()
    }
    pub fn extract_group(&self) -> HashMap<String, HashMap<String, Vec<ExtractedTrade>>> {
        let collected_data = self.extract();
        let mut result: HashMap<String, HashMap<String, Vec<ExtractedTrade>>> = HashMap::new();
        for col in collected_data {
            // Group by name
            let entry = result
                .entry(col.currency.clone())
                .or_insert_with(HashMap::new);
            // Group by date
            let date_entry = entry.entry(col.time.clone()).or_insert_with(Vec::new);
            // Add MyStruct to the date entry
            date_entry.push(col);
        }
        result
    }

    pub fn extract_by_type(&self, instrument_kind: String) -> Vec<ExtractedTrade> {
        self.data
            .iter()
            .filter(|t| t.instrument_kind.to_uppercase() == instrument_kind.to_uppercase())
            .map(|t| {
                ExtractedTrade {
                    id: t.id,
                    market: t.venue_instrument_name.clone(),
                    side: t.side.clone(),
                    kind: t.option_kind.clone().map_or_else(
                        || String::from("- -"),
                        |value| {
                            if value.len() > 0 {
                                value
                            } else {
                                String::from("- -")
                            }
                        },
                    ),
                    trans_type: t.instrument_kind.clone(),
                    // size: format_currency(t.amount.unwrap_or(0.0), t.base_currency_id.display_scale),
                    size: match t.activity.to_uppercase() == "OPEN" {
                        true => format_currency(
                            t.amount.unwrap_or(0.0),
                            t.base_currency_id.display_scale,
                        ),
                        false => format_currency(
                            t.amount.unwrap_or(0.0).abs(),
                            t.base_currency_id.display_scale,
                        ),
                    },
                    price: match t.activity.to_uppercase() == "OPEN" {
                        true => match t.payout_ccy.clone() {
                            Some(p) => {
                                if p == "base" {
                                    format_currency(
                                        t.px_in_base_ccy.unwrap_or(0.0),
                                        t.base_currency_id.display_scale,
                                    )
                                } else {
                                    format_currency(
                                        t.px_in_quote_ccy.unwrap_or(0.0),
                                        t.quote_currency_id.display_scale,
                                    )
                                }
                            }
                            None => format_currency(
                                t.px_in_quote_ccy.unwrap_or(0.0),
                                t.quote_currency_id.display_scale,
                            ),
                        },
                        // If the instrument kind is equals to Spot, then show even if it was closed
                        false => match t.instrument_kind.to_uppercase() == "SPOT" {
                            true => format_currency(
                                t.px_in_quote_ccy.unwrap_or(0.0),
                                t.quote_currency_id.display_scale,
                            ),
                            false => String::from("- -"),
                        },
                    },
                    // price: match t.payout_ccy.clone()
                    //     {
                    //         Some(p) => {
                    //             if p == "base" {
                    //                 format_currency(t.px_in_base_ccy.unwrap_or(0.0), t.base_currency_id.display_scale)
                    //             } else {
                    //                 format_currency(t.px_in_quote_ccy.unwrap_or(0.0), t.quote_currency_id.display_scale)
                    //             }
                    //         },
                    //         None => {
                    //             format_currency(t.px_in_quote_ccy.unwrap_or(0.0), t.quote_currency_id.display_scale)
                    //     },
                    // },
                    // live_pnl: match t.pnl_snapshot_ccy.clone() {
                    //     Some(p) => {
                    //         if p == "base" {
                    //             format_currency(
                    //                 t.pnl_snapshot.clone().unwrap_or(0.0),
                    //                 t.base_currency_id.display_scale,
                    //             )
                    //         } else {
                    //             format_currency(
                    //                 t.pnl_snapshot.clone().unwrap_or(0.0),
                    //                 t.quote_currency_id.display_scale,
                    //             )
                    //         }
                    //     }
                    //     None => {
                    //         String::from("- -")
                    //         // format_currency(t.pnl_snapshot.clone().unwrap_or(0.0), t.quote_currency_id.display_scale)
                    //     }
                    // },
                    live_pnl: t.pnl.clone().unwrap_or(0.0).to_string(),
                    time: extract_date(t.expiry_timestamp.clone()),
                    currency: t.base_currency_id.ticker.clone(),
                    date_created: format_utc_str_to_local_str(
                        t.date_created.clone().unwrap_or(String::from("")),
                    ),
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
                    live_pnl_ccy: {
                        match t.pnl_snapshot_ccy.clone() {
                            Some(p) => {
                                if p == "base" {
                                    t.pair_id.name.split("/").collect::<Vec<&str>>()[0].to_string()
                                } else {
                                    t.pair_id.name.split("/").collect::<Vec<&str>>()[1].to_string()
                                }
                            }
                            None => String::from(""),
                        }
                    },
                    realized_pnl: match t.pnl_ccy.clone() {
                        Some(p) => {
                            if p == t.base_currency_id.ticker {
                                format_currency(
                                    t.pnl.clone().unwrap_or(0.0),
                                    t.base_currency_id.display_scale,
                                )
                            } else {
                                format_currency(
                                    t.pnl.clone().unwrap_or(0.0),
                                    t.quote_currency_id.display_scale,
                                )
                            }
                        }
                        None => {
                            String::from("- -")
                            // format_currency(t.pnl_snapshot.clone().unwrap_or(0.0), t.quote_currency_id.display_scale)
                        }
                    },
                    realized_pnl_ccy: t.pnl_ccy.clone().unwrap_or_default(),
                    trade_type: t.trade_type.clone(),
                    trade_status: match t.trade_status.clone() {
                        Some(value) => value,
                        None => String::from("- -"),
                    },
                    activity: t.activity.clone(),
                    side_status: format!("{} {}", t.activity.clone(), t.side.clone()),
                    index_price: t.index_price.clone().unwrap_or(0.0).to_string(),
                    party_a: t.party_a.name.clone(),
                    party_b: t.party_b.name.clone(),
                    pnl_percentage: t.live_pnl_percentage.clone().unwrap_or(0.0).to_string(),
                    last_updated: match &t.date_updated {
                        Some(d) => format_utc_str_to_local_str(d.clone()),
                        None => format_utc_str_to_local_str(
                            t.date_created.clone().unwrap_or_default(),
                        ),
                    },
                }
            })
            .collect()
    }
    pub fn extract_group_by_type(
        &self,
        instrument_kind: String,
    ) -> HashMap<String, HashMap<String, Vec<ExtractedTrade>>> {
        let collected_data = self.extract_by_type(instrument_kind);
        let mut result: HashMap<String, HashMap<String, Vec<ExtractedTrade>>> = HashMap::new();
        for col in collected_data {
            // Group by name
            let entry = result
                .entry(col.currency.clone())
                .or_insert_with(HashMap::new);
            // Group by date
            let date_entry = entry.entry(col.time.clone()).or_insert_with(Vec::new);
            // Add MyStruct to the date entry
            date_entry.push(col);
        }
        result
    }

    pub fn extract_and_group_by_date(&self) -> HashMap<String, Vec<ExtractedTrade>> {
        let collected_data = self.extract();
        let mut result: HashMap<String, Vec<ExtractedTrade>> = HashMap::new();
        for col in collected_data {
            // Group by date
            let date_entry = result.entry(col.time.clone()).or_insert_with(Vec::new);
            // Add MyStruct to the date entry
            date_entry.push(col);
        }
        result
    }

    pub fn extract_by_instrument_kind_and_group_by_date(
        &self,
        instrument_kind: String,
    ) -> HashMap<String, Vec<ExtractedTrade>> {
        let collected_data = self.extract_by_type(instrument_kind);
        let mut result: HashMap<String, Vec<ExtractedTrade>> = HashMap::new();
        for col in collected_data {
            // Group by date
            let date_entry = result.entry(col.time.clone()).or_insert_with(Vec::new);
            // Add MyStruct to the date entry
            date_entry.push(col);
        }
        result
    }

    pub fn extract_by_trade_status(&self, trade_status: String, reverse: bool) -> Vec<ExtractedTrade> {

        self.data
            .iter()
            .filter(|t| if reverse 
                {t.trade_status.clone().unwrap_or_default().to_uppercase() != trade_status.to_uppercase()} 
                else 
                {t.trade_status.clone().unwrap_or_default().to_uppercase() == trade_status.to_uppercase()}
            )
            .map(|t| {
                ExtractedTrade {
                    id: t.id,
                    market: t.venue_instrument_name.clone(),
                    side: t.side.clone(),
                    kind: t.option_kind.clone().map_or_else(
                        || String::from("- -"),
                        |value| {
                            if value.len() > 0 {
                                value
                            } else {
                                String::from("- -")
                            }
                        },
                    ),
                    trans_type: t.instrument_kind.clone(),
                    // size: format_currency(t.amount.unwrap_or(0.0), t.base_currency_id.display_scale),
                    size: match t.activity.to_uppercase() == "OPEN" {
                        true => format_currency(
                            t.amount.unwrap_or(0.0),
                            t.base_currency_id.display_scale,
                        ),
                        false => format_currency(
                            t.amount.unwrap_or(0.0).abs(),
                            t.base_currency_id.display_scale,
                        ),
                    },
                    price: match t.activity.to_uppercase() == "OPEN" {
                        true => match t.payout_ccy.clone() {
                            Some(p) => {
                                if p == "base" {
                                    format_currency(
                                        t.px_in_base_ccy.unwrap_or(0.0),
                                        t.base_currency_id.display_scale,
                                    )
                                } else {
                                    format_currency(
                                        t.px_in_quote_ccy.unwrap_or(0.0),
                                        t.quote_currency_id.display_scale,
                                    )
                                }
                            }
                            None => format_currency(
                                t.px_in_quote_ccy.unwrap_or(0.0),
                                t.quote_currency_id.display_scale,
                            ),
                        },
                        // If the instrument kind is equals to Spot, then show even if it was closed
                        false => match t.instrument_kind.to_uppercase() == "SPOT" {
                            true => format_currency(
                                t.px_in_quote_ccy.unwrap_or(0.0),
                                t.quote_currency_id.display_scale,
                            ),
                            false => String::from("- -"),
                        },
                    },
                    live_pnl: t.pnl.clone().unwrap_or(0.0).to_string(),
                    time: extract_date(t.expiry_timestamp.clone()),
                    currency: t.base_currency_id.ticker.clone(),
                    date_created: format_utc_str_to_local_str(
                        t.date_created.clone().unwrap_or(String::from("")),
                    ),
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
                    live_pnl_ccy: {
                        match t.pnl_snapshot_ccy.clone() {
                            Some(p) => {
                                if p == "base" {
                                    t.pair_id.name.split("/").collect::<Vec<&str>>()[0].to_string()
                                } else {
                                    t.pair_id.name.split("/").collect::<Vec<&str>>()[1].to_string()
                                }
                            }
                            None => String::from(""),
                        }
                    },
                    realized_pnl: match t.pnl_ccy.clone() {
                        Some(p) => {
                            if p == t.base_currency_id.ticker {
                                format_currency(
                                    t.pnl.clone().unwrap_or(0.0),
                                    t.base_currency_id.display_scale,
                                )
                            } else {
                                format_currency(
                                    t.pnl.clone().unwrap_or(0.0),
                                    t.quote_currency_id.display_scale,
                                )
                            }
                        }
                        None => {
                            String::from("- -")
                            // format_currency(t.pnl_snapshot.clone().unwrap_or(0.0), t.quote_currency_id.display_scale)
                        }
                    },
                    realized_pnl_ccy: t.pnl_ccy.clone().unwrap_or_default(),
                    trade_type: t.trade_type.clone(),
                    trade_status: match t.trade_status.clone() {
                        Some(value) => value,
                        None => String::from("- -"),
                    },
                    activity: t.activity.clone(),
                    side_status: format!("{} {}", t.activity.clone(), t.side.clone()),
                    index_price: t.index_price.clone().unwrap_or(0.0).to_string(),
                    party_a: t.party_a.name.clone(),
                    party_b: t.party_b.name.clone(),
                    pnl_percentage: t.live_pnl_percentage.clone().unwrap_or(0.0).to_string(),
                    last_updated: match &t.date_updated {
                        Some(d) => format_utc_str_to_local_str(d.to_string()),
                        None => format_utc_str_to_local_str(t.date_created.clone().unwrap_or_default()),
                    },
                }
            })
            .collect()
    }

    pub fn extract_csv_by_trade_status(
        &self,
        trade_status: Option<String>,
        reverse: bool,
    ) -> String {
        let data = match trade_status {
            Some(value) => self.extract(),
            None => self.extract(),
        };
        let mut content = String::new();
        content.push_str("Date Created, Client, Market,Side, Type, Trade Type, Kind, Size, Price, Price Currency, Index Price, Realized Pnl, Realized Pnl Currency, Status\n");
        for i in data {
            let line = format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                i.date_created,
                i.party_b,
                i.market,
                i.side,
                i.trans_type,
                i.trade_type,
                i.kind,
                i.size,
                i.price,
                i.premium_ccy,
                i.index_price,
                i.realized_pnl,
                i.realized_pnl_ccy,
                i.trade_status
            );
            content.push_str(&line.as_str());
        }
        content
    }
    pub fn extract_csv_by_kind(&self, kind: Option<String>) -> String {
        let data = match kind {
            Some(value) => self.extract_by_type(value),
            None => self.extract(),
        };
        let mut content = String::new();
        content.push_str("Date Created, Client, Market,Side, Type, Trade Type, Kind, Size, Price, Price Currency, Index Price, Realized Pnl, Realized Pnl Currency, Status\n");
        for i in data {
            let line = format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                i.date_created,
                i.party_b,
                i.market,
                i.side,
                i.trans_type,
                i.trade_type,
                i.kind,
                i.size,
                i.price,
                i.premium_ccy,
                i.index_price,
                i.realized_pnl,
                i.realized_pnl_ccy,
                i.trade_status
            );
            content.push_str(&line.as_str());
        }
        content
    }
}

/// Struct for the data that can be shown in the data table.
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct ExtractedTrade {
    pub id: u32,
    pub market: String,
    pub side: String,
    pub kind: String,
    pub trans_type: String,
    pub size: String,
    pub price: String,
    pub live_pnl: String,
    pub time: String,
    pub currency: String,
    pub date_created: String,
    pub premium_ccy: String,
    pub live_pnl_ccy: String,
    pub realized_pnl: String,
    pub realized_pnl_ccy: String,
    pub trade_status: String,
    pub trade_type: String,
    pub activity: String,
    pub side_status: String,
    pub index_price: String,
    pub party_a: String,
    pub party_b: String,
    pub pnl_percentage: String,
    pub last_updated: String,
}

/// Struct for the Trade data.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub id: u32,
    pub date_created: Option<String>,
    pub expiry_timestamp: String,
    pub venue_instrument_name: String,
    pub instrument_kind: String,
    pub side: String,
    pub group_id: String,
    /// Time to maturity.
    /// This can be optional since the closing record of a trade does not have this field.
    pub ttm: Option<f64>,
    /// Premium price in base currency.
    /// This can be optional since the closing record of a trade does not have this field.
    pub px_in_base_ccy: Option<f64>,
    /// Premium price in quote currency.
    /// This can be optional since the closing record of a trade does not have this field.
    pub px_in_quote_ccy: Option<f64>,
    pub payout_ccy: Option<String>,
    pub strike: f64,
    ///Price or a notional amount of the trade.
    /// This can be optional since the closing record of a trade does not have this field.
    pub amount: Option<f64>,
    pub option_kind: Option<String>,
    /// Spot price.
    /// This can be optional since the closing record of a trade does not have this field.
    pub spot: Option<f64>,
    /// Risk free rate 1.
    /// This can be optional since the closing record of a trade does not have this field.
    pub r1: Option<f64>,
    /// Risk free rate 2.
    /// This can be optional since the closing record of a trade does not have this field.
    pub r2: Option<f64>,
    ///Implied Volatility.
    /// This can be optional since the closing record of a trade does not have this field.
    pub iv: Option<f64>,
    pub base_currency_id: Currency,
    pub quote_currency_id: Currency,
    pub ccy_id: Currency,
    pub pair_id: CurrencyPair,
    pub counterparty_id: CounterParty,
    pub party_a: CounterParty,
    pub party_b: CounterParty,
    pub user_created: Option<User>,
    pub pnl_snapshot_ccy: Option<String>,
    pub pnl_snapshot: Option<f64>,
    pub pnl_ccy: Option<String>,
    pub pnl: Option<f64>,
    pub trade_status: Option<String>,
    pub trade_type: String,
    pub activity: String,
    pub index_price: Option<f64>,
    pub live_pnl_percentage: Option<f64>,
    pub date_updated: Option<String>,
}

impl Trade {
    pub fn get_query() -> String {
        format!(
            "id, date_created, date_updated, expiry_timestamp, venue_instrument_name, instrument_kind, side, group_id, ttm, px_in_base_ccy, px_in_quote_ccy, payout_ccy, strike, amount, option_kind, spot, r1, r2, iv, pnl_snapshot_ccy, pnl_snapshot, pnl_ccy, pnl, trade_status, trade_type, activity, index_price, {}, {}, {}, {}, {}, {}, {}, {}",
            Currency::get_query("base_currency_id"),
            Currency::get_query("quote_currency_id"),
            Currency::get_query("ccy_id"),
            CurrencyPair::get_query("pair_id"),
            CounterParty::get_query("counterparty_id"),
            CounterParty::get_query("party_a"),
            CounterParty::get_query("party_b"),
            User::get_query("user_created")
        )
    }
}

/// Function that sorts the data table.

pub fn sort(
    mut data_table: Vec<ExtractedTrade>,
    sort_type: bool,
    sort_by: String,
) -> Vec<ExtractedTrade> {
    match sort_by.to_uppercase().as_str() {
        "ID" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.id.cmp(&b.id));
            }
            false => {
                data_table.sort_by(|a, b| b.id.cmp(&a.id));
            }
        },
        "EXPIRY DATE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.time.cmp(&b.time));
            }
            false => {
                data_table.sort_by(|a, b| b.time.cmp(&a.time));
            }
        },
        "MARKET" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.market.cmp(&b.market));
            }
            false => {
                data_table.sort_by(|a, b| b.market.cmp(&a.market));
            }
        },
        "PRICE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.price.cmp(&b.price));
            }
            false => {
                data_table.sort_by(|a, b| b.price.cmp(&a.price));
            }
        },
        "KIND" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.kind.cmp(&b.kind));
            }
            false => {
                data_table.sort_by(|a, b| b.kind.cmp(&a.kind));
            }
        },
        "TYPE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.trans_type.cmp(&b.trans_type));
            }
            false => {
                data_table.sort_by(|a, b| b.trans_type.cmp(&a.trans_type));
            }
        },
        "DATE CREATED" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.date_created.cmp(&b.date_created));
            }
            false => {
                data_table.sort_by(|a, b| b.date_created.cmp(&a.date_created));
            }
        },
        "SIDE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.side.cmp(&b.side));
            }
            false => {
                data_table.sort_by(|a, b| b.side.cmp(&a.side));
            }
        },
        "SIZE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.size.cmp(&b.size));
            }
            false => {
                data_table.sort_by(|a, b| b.size.cmp(&a.size));
            }
        },
        "LIVE PNL" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.live_pnl.partial_cmp(&b.live_pnl).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.live_pnl.partial_cmp(&a.live_pnl).unwrap());
            }
        },
        "REALIZED PNL" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.realized_pnl.partial_cmp(&b.realized_pnl).unwrap());
            }
            false => {
                data_table.sort_by(|a, b| b.realized_pnl.partial_cmp(&a.realized_pnl).unwrap());
            }
        },
        "STATUS" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.activity.cmp(&b.activity));
            }
            false => {
                data_table.sort_by(|a, b| b.activity.cmp(&a.activity));
            }
        },
        "IS EXERCISED" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.trade_status.cmp(&b.trade_status));
            }
            false => {
                data_table.sort_by(|a, b| b.trade_status.cmp(&a.trade_status));
            }
        },
        "TRADE TYPE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.trade_type.cmp(&b.trade_type));
            }
            false => {
                data_table.sort_by(|a, b| b.trade_type.cmp(&a.trade_type));
            }
        },
        "INDEX PRICE" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.index_price.cmp(&b.index_price));
            }
            false => {
                data_table.sort_by(|a, b| b.index_price.cmp(&a.index_price));
            }
        },
        "PARTY A" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.party_a.cmp(&b.party_a));
            }
            false => {
                data_table.sort_by(|a, b| b.party_a.cmp(&a.party_a));
            }
        },
        "PARTY B" => match sort_type {
            true => {
                data_table.sort_by(|a, b| a.party_b.cmp(&b.party_b));
            }
            false => {
                data_table.sort_by(|a, b| b.party_b.cmp(&a.party_b));
            }
        },
        _ => (),
    }
    data_table
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TradeVenueOrderId {
    pub venue_order_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TradeVenueOrderIdResponse {
    pub data: Vec<TradeVenueOrderId>,
}

impl TradeVenueOrderIdResponse {
    pub fn concatenate_venue_order_ids(&self) -> String {
        let mut concatenated_ids = String::new();
        for (index, trade_venue_order_id) in self.data.iter().enumerate() {
            concatenated_ids.push_str(&trade_venue_order_id.venue_order_id);
            if index < self.data.len() - 1 {
                concatenated_ids.push(',');
            }
        }
        concatenated_ids
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateIVRequest {
    pub query: ApproveTradeQuoteRequestQuery,
    pub data: UpdateIVRequestData,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct UpdateIVRequestData {
    pub iv: f64,
}

impl UpdateIVRequest {
    pub fn new(group_id: Vec<String>, iv: f64) -> Self {
        Self {
            query: ApproveTradeQuoteRequestQuery {
                filter: ApproveTradeQuoteRequestQueryFilter {
                    group_id: FilterGroupId { _in: group_id },
                },
            },
            data: UpdateIVRequestData { iv },
        }
    }

    pub fn deserialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
/***************Trade Quotes Structs ***************/
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApproveTradeQuoteRequest {
    pub query: ApproveTradeQuoteRequestQuery,
    pub data: ApproveTradeQuoteRequestData,
}

/// Struct for the query of [`ApproveTradeQuoteRequest`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApproveTradeQuoteRequestQuery {
    pub filter: ApproveTradeQuoteRequestQueryFilter,
}

/// Struct for the filter of [`ApproveTradeQuoteRequestQuery`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApproveTradeQuoteRequestQueryFilter {
    pub group_id: FilterGroupId,
}

/// Struct for the group id of [`ApproveTradeQuoteRequestQueryFilter`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct FilterGroupId {
    pub _in: Vec<String>,
}

/// Struct for the data of [`ApproveTradeQuoteRequest`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ApproveTradeQuoteRequestData {
    pub quote_status: String,
}

impl ApproveTradeQuoteRequest {
    pub fn new(group_id: Vec<String>, quote_status: String) -> Self {
        Self {
            query: ApproveTradeQuoteRequestQuery {
                filter: ApproveTradeQuoteRequestQueryFilter {
                    group_id: FilterGroupId { _in: group_id },
                },
            },
            data: ApproveTradeQuoteRequestData { quote_status },
        }
    }

    pub fn deserialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PositionGreekResponse {
    pub px_in_base_ccy: f64,
    pub px_in_quote_ccy: f64,
    pub greeks: Greeks,
    pub pnl: f64,
    pub pnl_percentage: f64,
    pub req_id: Option<String>,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PositionGreekRequest {
    pub side: String,
    pub option_kind: String,
    pub amount: f64,
    pub strike: f64,
    pub ttm: f64,
    pub inception_price: f64,
    pub spot: Option<f64>,
    pub r2: Option<f64>,
    pub r1: Option<f64>,
    pub iv: Option<f64>,
    pub expiry: Option<String>,
    pub req_id: Option<String>,
}
impl PositionGreekRequest {
    pub fn new(
        side: String,
        option_kind: String,
        amount: f64,
        strike: f64,
        ttm: f64,
        inception_price: f64,
        spot: Option<f64>,
        r2: Option<f64>,
        r1: Option<f64>,
        iv: Option<f64>,
        expiry: Option<String>,
        req_id: Option<String>,
    ) -> Self {
        Self {
            side,
            option_kind,
            amount,
            strike,
            ttm,
            inception_price,
            spot,
            r2,
            r1,
            iv,
            expiry,
            req_id,
        }
    }
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PositionsGreeksRequest {
    pub positions: Vec<PositionGreekRequest>,
    pub current_spot: f64,
    pub spot_bump: f64,
    pub bump_times: i32,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct AtmRiskSlide {
    pub spot: f64,
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub pnl: f64,
}
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PositionsGreeksResponse {
    pub data: PositionsGreeksResponseData,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PositionsGreeksResponseData {
    pub atm_risk_slide: AtmRiskSlide,
    pub positions: Vec<PositionGreekResponse>,
    pub agg_bumped_greeks: Vec<BumpedGreek>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BumpedGreek {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PositionsGreeksResponseExtraData {
    pub atm_risk_slide: AtmRiskSlide,
    pub positions: Vec<RiskSlideTrade>,
    pub agg_bumped_greeks: Vec<BumpedGreek>,
}

impl PositionsGreeksResponseExtraData {
    pub fn new(
        atm_risk_slide: AtmRiskSlide,
        positions: Vec<RiskSlideTrade>,
        agg_bumped_greeks: Vec<BumpedGreek>,
    ) -> Self {
        Self {
            atm_risk_slide,
            positions,
            agg_bumped_greeks,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeribitPositions {
    pub estimated_liquidation_price: Option<f64>,
    pub size_currency: Option<f64>,
    pub total_profit_loss: Option<f64>,
    pub realized_profit_loss: Option<f64>,
    pub floating_profit_loss: Option<f64>,
    pub leverage: Option<f64>,
    pub average_price: Option<f64>,
    pub delta: Option<f64>,
    pub open_orders_margin: Option<f64>,
    pub initial_margin: Option<f64>,
    pub maintenance_margin: Option<f64>,
    pub settlement_price: Option<f64>,
    pub instrument_name: Option<String>,
    pub mark_price: Option<f64>,
    pub index_price: Option<f64>,
    pub direction: Option<String>,
    pub kind: Option<String>,
    pub size: Option<f64>,
    pub floating_profit_loss_usd: Option<f64>,
    pub average_price_usd:  Option<f64>,
    pub theta:  Option<f64>,
    pub vega:  Option<f64>,
    pub gamma:  Option<f64>,
    pub realized_funding:  Option<f64>,
    pub interest_value:  Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeribitRiskSlide {
    pub delta: f64,
    pub gamma: f64,
    pub theta: f64,
    pub vega: f64,
    pub pnl: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeribitPositionsRequest {
    pub currency: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeribitPositionsResponseData {
    pub data: DeribitPositionsResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeribitPositionsResponse {
    pub positions: Vec<DeribitPositions>,
    pub deribit_risk_slide: DeribitRiskSlide,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ITMOTMPositionsRequest {
    pub currency: String,
    pub counterparty: String,
    pub current_spot: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ITMOTMPositions {
    pub id: i32,
    pub instrument_name: String,
    pub amount: f64,
    pub side: String,
    pub index_price:f64,
    pub pnl: f64,
    pub pnl_ccy: String,
    pub date_created: String,
    pub expiry_timestamp: String,
    pub trade_status: String,
    pub counterparty_id: CounterPartyId,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CounterPartyId {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ITMOTMRiskSlide {
    pub pnl_in_base_ccy: f64,
    pub pnl_in_mark_price: f64,
    pub delta: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ITMOTMPositionsResponseData {
    pub data: ITMOTMPositionsResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ITMOTMPositionsResponse {
    pub positions: Vec<ITMOTMPositions>,
    pub positions_itm_otm_risk_slide: ITMOTMRiskSlide,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CollateralRequest {
    pub currency: String,
    pub counterparty: String,
    pub current_spot: f64,
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Collateral {
    pub total_initial_usd: f64,
    pub total_current_usd: f64,
    pub total_notional: f64,
    pub pnl: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CollateralData {
    pub exchange_name: String,
    pub initial_usd: f64,
    pub current_usd: f64,
    pub notional: f64,
    pub pnl: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CollateralResponseData {
    pub data: CollateralResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CollateralResponse {
    pub unwind_risk_slide: Collateral,
    pub exchanges_unwind: Vec<CollateralData>,
}