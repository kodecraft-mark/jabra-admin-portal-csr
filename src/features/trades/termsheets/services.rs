use leptos::ServerFnError;
use crate::commons::models::defaults::BlankRequest;
use crate::utilities::cookies::{get_jabra_cookie, JabraCookie,set_jabra_cookie,refresh_token};
use crate::utilities::http_wrapper::{call_and_parse,call, HttpMethod};

use super::models::{ApproveRejectTermSheetRequest, GetNewTermSheetResponse, TermSheetApprovalStatus};

pub async fn fetch_new_term_sheet_list() -> Result<GetNewTermSheetResponse, ServerFnError> {

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    if jwt_cookie.is_expired() {
        return Ok(GetNewTermSheetResponse::default());
    }
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/dcl?filter[term_sheet_status][_eq]=New&sort[]=-id&fields=id,reference_id,counterparty_id.name,pair_id.name,base_ccy_id.ticker,term_ccy_id.ticker,deposit_ccy_id.ticker,deal_date,expiry_date,deposit_amount,spot_t1,strike,r2,r1,iv_t1,collateral_setting_method,collateral_exchange_settlement,exchange_rate_determining_agent,term_sheet,term_sheet_status,instrument_type,conditional_loss_limit_event,stop_loss_level,px_in_base_ccy,px_in_quote_ccy,dcl_settlement_details.settlement_template_id,dcl_settlement_details.settlement_condition,dcl_settlement_details.settlement_value", url.unwrap_or_default());
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );

    let response = call_and_parse::<BlankRequest, GetNewTermSheetResponse>(
        Option::None,
        path,
        headers,
        HttpMethod::GET,
    )
    .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::info!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// Server function for approving or rejecting the term sheet.
/// Accepts [`ApproveRejectTermSheetRequest`] and returns [`bool`].

pub async fn post_approve_term_sheet(
    request: ApproveRejectTermSheetRequest,
) -> Result<bool, ServerFnError> {

    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    let mut bearer = format!("Bearer {}", jwt_cookie.access_token);

    // Check if token expires, this checking will be available only to actions and server action
    // Other resources will still work due to 10 minutes buffer time
    if jwt_cookie.is_expired() {
        let refresh =
            refresh_token(jwt_cookie.user_id, jwt_cookie.refresh_token).await;
        match refresh {
            Ok(r) => {
                bearer = format!("Bearer {}", r.access_token);
                set_jabra_cookie(r, "admin_portal_csr".to_string()).await;
            }
            Err(e) => {
                // directus_wrapper::set_jabra_cookie(None, "JabraOPv1_2023".to_string()).await;
                return Err(ServerFnError::new(e.to_string()));
            }
        }
    }
    let url = option_env!("DIRECTUSURL");
    let path = format!("{}/items/dcl/{}", url.unwrap_or_default(), request.id);
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );
    let termsheet_status = TermSheetApprovalStatus {
        term_sheet_status: request.status.to_string(),
    };

    let response =
        call::<TermSheetApprovalStatus>(Some(termsheet_status), path, headers, HttpMethod::PATCH)
            .await;
    match response {
        Ok(res) => Ok(res),
        Err(e) => {
            log::info!("error: {:?}", e);
            Err(ServerFnError::new(e.to_string()))
        }
    }
}

/// Server function for downloading the termsheet.
/// Accepts [`String`] and returns [`String`].

pub async fn download_termsheet(file_id: String) -> Result<String, ServerFnError> {
    let url = option_env!("JABRAAPIGATEWAYPUB");
    let path = format!("{}/{}/{}", url.unwrap_or_default(), "option_pricer/dl_termsheet", file_id);
    Ok(path)
}