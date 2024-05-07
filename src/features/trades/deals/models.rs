use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct BlankRequest;

/// Struct for Deals data.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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

impl Default for Deals {
    fn default() -> Self {
        Self {
            base_currency: "".to_string(),
            qoute_currency: "".to_string(),
            expiry_in_days: 0.0,
            currency: "".to_string(),
            ccy1_amount: 0.0,
            ccy2_amount: 0.0,
            strike: 0.0,
            amount: 0.0,
            option_kind: "".to_string(),
            spot: 0.0,
            r1: 0.0,
            r2: 0.0,
            iv_t1: 0.0,
            px_in_base_ccy: 0.0,
            px_in_quote_ccy: 0.0,
            jabra_side: "".to_string(),
            expiry_timestamp: String::from(""),
        }
    }
}

/// Struct for Deals data with group id.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DealsWithGroupId {
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
    pub group_id: String,
}

impl Default for DealsWithGroupId {
    fn default() -> Self {
        Self {
            base_currency: "".to_string(),
            qoute_currency: "".to_string(),
            expiry_in_days: 0.0,
            currency: "".to_string(),
            ccy1_amount: 0.0,
            ccy2_amount: 0.0,
            strike: 0.0,
            amount: 0.0,
            option_kind: "".to_string(),
            spot: 0.0,
            r1: 0.0,
            r2: 0.0,
            iv_t1: 0.0,
            px_in_base_ccy: 0.0,
            px_in_quote_ccy: 0.0,
            group_id: "".to_string(),
        }
    }
}

/// Struct for CounterParty Response.
/// Has a vector of [`CounterParty`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CounterPartyResponse {
    pub data: Vec<CounterParty>,
}

impl Default for CounterPartyResponse {
    fn default() -> Self {
        Self {
            data: vec![CounterParty::default()],
        }
    }
}

/// Struct for CounterParty data.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CounterParty {
    pub ticker: String,
    pub name: String,
    pub short_name: Option<String>,
    pub is_exchange: bool,
}

impl Default for CounterParty {
    fn default() -> Self {
        Self {
            ticker: "".to_string(),
            name: "".to_string(),
            short_name: None,
            is_exchange: false,
        }
    }
}

/// Struct for Settlement Option Data.

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementOption {
    pub id: i32,
    pub settlement_description: String,
    pub collateral_exchange_settlement: String,
    pub is_static_value: bool,
    pub settlement_condition: String,
    pub settlement_value: String,
    pub option_kind: String,
    pub if_exercised: bool,
    pub deposit_ccy: String,
}
impl SettlementOption {
    pub fn new(
        id: i32,
        settlement_description: String,
        collateral_exchange_settlement: String,
        is_static_value: bool,
        settlement_condition: String,
        settlement_value: String,
        option_kind: String,
        if_exercised: bool,
        deposit_ccy: String,
    ) -> Self {
        Self {
            id,
            settlement_description,
            collateral_exchange_settlement,
            is_static_value,
            settlement_condition,
            settlement_value,
            option_kind,
            if_exercised,
            deposit_ccy,
        }
    }
}
impl Default for SettlementOption {
    fn default() -> Self {
        Self {
            id: 0,
            settlement_description: "".to_string(),
            collateral_exchange_settlement: "".to_string(),
            is_static_value: false,
            settlement_condition: "".to_string(),
            settlement_value: "".to_string(),
            option_kind: "".to_string(),
            if_exercised: false,
            deposit_ccy: "".to_string(),
        }
    }
}

/// Struct for Settlement Option Response.
/// Has a vector of [`SettlementOption`].

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementOptionResponse {
    pub data: Vec<SettlementOption>,
}

impl Default for SettlementOptionResponse {
    fn default() -> Self {
        Self {
            data: Vec::<SettlementOption>::default(),
        }
    }
}

impl SettlementOptionResponse {
    pub fn to_dcl_payment_list(&self) -> Vec<DclPaymentList> {
        self.data
            .iter()
            .map(|settlement_option| DclPaymentList {
                id: settlement_option.id,
                settlement_condition: settlement_option.settlement_condition.clone(),
                settlement_value: settlement_option.settlement_value.clone(),
            })
            .collect()
    }
}

/// Struct for Settlement Option Request.

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct SettlementOptionRequest {
    pub spot_t1: f64,
    pub strike: f64,
    pub deposit: f64,
    pub ccy2_premium: f64,
    pub counterparty_name: String,
    pub pair_name: String,
    pub base_ccy: String,
    pub term_ccy: String,
    pub deposit_ccy: String,                    //ccy1 or ccy2
    pub call_or_put: String,                    //call or put
    pub collateral_exchange_settlement: String, //cash or delivery
    pub jabra_side: String,                     //buy or sell
}

/// Struct for Submit New Term Sheet Request.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitNewTermSheetRequest {
    pub counterparty_name: String,
    pub pair_name: String,
    pub base_ccy: String,
    pub term_ccy: String,
    pub instrument_type: String,
    pub deal_date: String,
    pub expiry_date: String,
    pub deposit_amount: f64,
    pub deposit_ccy: String,
    pub spot_t1: f64,
    pub strike: f64,
    pub r2: f64,
    pub r1: f64,
    pub iv_t1: f64,
    pub term_sheet: String,
    pub dcl_payment: Vec<DclPaymentList>,
    pub collateral_setting_method: String,
    pub collateral_exchange_settlement: String,
    pub exchange_rate_determining_agent: String,
    pub stop_loss_level: f64,
    pub px_in_base_ccy: f64,
    pub px_in_quote_ccy: f64,
    pub conditional_loss_limit_event: String,
    pub settlement_ccy: String,
}

/// Struct for Submit New Term Sheet Request with Group Id.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitNewTermSheetRequestWithGroupId {
    pub counterparty_name: String,
    pub pair_name: String,
    pub base_ccy: String,
    pub term_ccy: String,
    pub instrument_type: String,
    pub deal_date: String,
    pub expiry_date: String,
    pub deposit_amount: f64,
    pub deposit_ccy: String,
    pub spot_t1: f64,
    pub strike: f64,
    pub r2: f64,
    pub r1: f64,
    pub iv_t1: f64,
    pub term_sheet: String,
    pub dcl_payment: Vec<DclPaymentList>,
    pub collateral_setting_method: String,
    pub collateral_exchange_settlement: String,
    pub exchange_rate_determining_agent: String,
    pub stop_loss_level: f64,
    pub px_in_base_ccy: f64,
    pub px_in_quote_ccy: f64,
    pub conditional_loss_limit_event: String,
    pub group_id: String,
    pub settlement_ccy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DclPaymentList {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "settlement_condition")]
    pub settlement_condition: String,
    #[serde(rename = "settlement_value")]
    pub settlement_value: String,
}

/// Struct for Submit New Term Sheet Response.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubmitNewTermSheetResponse {
    pub status: i32,
    pub message: String,
    pub refid: String,
}
