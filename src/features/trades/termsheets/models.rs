use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlankRequest;

/// Struct for the Download Term Sheet Request.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadTermSheetRequest {
    pub fileid: String,
}

/// Struct for the Get New Term Sheet Response.
/// Has a vector of [`GetNewTermSheetData`].

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNewTermSheetResponse {
    pub data: Vec<GetNewTermSheetData>,
}

/// Struct for the Get New Term Sheet Data.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNewTermSheetData {
    pub id: i64,
    #[serde(rename = "reference_id")]
    pub reference_id: String,
    #[serde(rename = "deal_date")]
    pub deal_date: String,
    #[serde(rename = "expiry_date")]
    pub expiry_date: String,
    #[serde(rename = "deposit_amount")]
    pub deposit_amount: f64,
    #[serde(rename = "spot_t1")]
    pub spot_t1: f64,
    pub strike: f64,
    pub r2: f64,
    pub r1: f64,
    #[serde(rename = "iv_t1")]
    pub iv_t1: f64,
    #[serde(rename = "collateral_setting_method")]
    pub collateral_setting_method: String,
    #[serde(rename = "collateral_exchange_settlement")]
    pub collateral_exchange_settlement: String,
    #[serde(rename = "exchange_rate_determining_agent")]
    pub exchange_rate_determining_agent: String,
    #[serde(rename = "term_sheet")]
    pub term_sheet: Option<String>,
    #[serde(rename = "term_sheet_status")]
    pub term_sheet_status: String,
    #[serde(rename = "instrument_type")]
    pub instrument_type: String,
    #[serde(rename = "stop_loss_level")]
    pub stop_loss_level: f64,
    #[serde(rename = "px_in_base_ccy")]
    pub px_in_base_ccy: f64,
    #[serde(rename = "px_in_quote_ccy")]
    pub px_in_quote_ccy: f64,
    #[serde(rename = "counterparty_id")]
    pub counterparty_id: CounterpartyId,
    #[serde(rename = "pair_id")]
    pub pair_id: PairId,
    #[serde(rename = "base_ccy_id")]
    pub base_ccy_id: BaseCcyId,
    #[serde(rename = "term_ccy_id")]
    pub term_ccy_id: TermCcyId,
    #[serde(rename = "deposit_ccy_id")]
    pub deposit_ccy_id: DepositCcyId,
    #[serde(rename = "dcl_settlement_details")]
    pub dcl_settlement_details: Vec<DclSettlementDetail>,
    #[serde(rename = "conditional_loss_limit_event")]
    pub conditional_loss_limit_event: Option<String>,
}

/// Struct for the Counterparty Id.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CounterpartyId {
    pub name: String,
}

/// Struct for the Pair Id.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PairId {
    pub name: String,
}

/// Struct for the Base Ccy Id.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseCcyId {
    pub ticker: String,
}

/// Struct for the Term Ccy Id.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TermCcyId {
    pub ticker: String,
}

/// Struct for the Deposit Ccy Id.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositCcyId {
    pub ticker: String,
}

/// Struct for the Dcl Settlement Detail.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DclSettlementDetail {
    #[serde(rename = "settlement_template_id")]
    pub settlement_template_id: Option<i64>,
    #[serde(rename = "settlement_condition")]
    pub settlement_condition: String,
    #[serde(rename = "settlement_value")]
    pub settlement_value: String,
}

/// Struct for Approve or Rejeect Term Sheet Request.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApproveRejectTermSheetRequest {
    pub id: i64,
    pub status: String,
}

/// Struct for Term Sheet Approval Status.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TermSheetApprovalStatus {
    pub term_sheet_status: String,
}

/// Struct for the Approve or Reject Term Sheet Response.
/// Used in modals.

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApproveRejectTermSheetResponse {
    pub success: bool,
    pub message: String,
}

impl ApproveRejectTermSheetResponse {
    pub fn new(success: bool, message: String) -> Self {
        Self { success, message }
    }
}

