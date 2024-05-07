use super::models::*;
use leptos::ServerFnError;
use crate::utilities::cookies::{get_jabra_cookie, JabraCookie};
use crate::utilities::http_wrapper::{call_and_parse, HttpMethod};

pub async fn post_settlement_option(
    request: SettlementOptionRequest,
) -> Result<SettlementOptionResponse, ServerFnError> {

    // let cookie = get_cookie_value("JabraOPv1_2023").await;
    // let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    // let mut bearer = format!("Bearer {}", jwt_cookie.access_token);

    // // Check if token expires, this checking will be available only to actions and server action
    // // Other resources will still work due to 10 minutes buffer time
    // if jwt_cookie.is_expired() {
    //     let refresh =
    //         server_wrapper::refresh_token(jwt_cookie.user_id, jwt_cookie.refresh_token).await;
    //     match refresh {
    //         Ok(r) => {
    //             bearer = format!("Bearer {}", r.access_token);
    //             directus_wrapper::set_jabra_cookie(Some(r), "JabraOPv1_2023".to_string()).await;
    //         }
    //         Err(e) => {
    //             // directus_wrapper::set_jabra_cookie(None, "JabraOPv1_2023".to_string()).await;
    //             return Err(ServerFnError::new(e.to_string()));
    //         }
    //     }
    // }
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    if jwt_cookie.is_expired() {
        return Ok(Default::default());
    }
    let bearer = format!("Bearer {}", jwt_cookie.access_token);
    let url = option_env!("JABRAAPIGATEWAY");
    let path = format!("{}/option_pricer/settlement_template", url.unwrap_or_default());

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );
    log::info!("request: {:?}", request);
    let response = call_and_parse::<SettlementOptionRequest, SettlementOptionResponse>(
        Some(request),
        path,
        headers,
        HttpMethod::POST,
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

pub async fn post_submit_new_term_sheet_with_id(
    request: SubmitNewTermSheetRequestWithGroupId,
) -> Result<SubmitNewTermSheetResponse, ServerFnError> {

    // let cookie = get_cookie_value("JabraOPv1_2023").await;
    // let jwt_cookie = JabraCookie::decrypt(cookie.unwrap()).unwrap_or_default();
    // let mut bearer = format!("Bearer {}", jwt_cookie.access_token);

    // // Check if token expires, this checking will be available only to actions and server action
    // // Other resources will still work due to 10 minutes buffer time
    // if jwt_cookie.is_expired() {
    //     let refresh =
    //         server_wrapper::refresh_token(jwt_cookie.user_id, jwt_cookie.refresh_token).await;
    //     match refresh {
    //         Ok(r) => {
    //             bearer = format!("Bearer {}", r.access_token);
    //             directus_wrapper::set_jabra_cookie(Some(r), "JabraOPv1_2023".to_string()).await;
    //         }
    //         Err(e) => {
    //             // directus_wrapper::set_jabra_cookie(None, "JabraOPv1_2023".to_string()).await;
    //             return Err(ServerFnError::new(e.to_string()));
    //         }
    //     }
    // }
    let cookie = get_jabra_cookie("admin_portal_csr".to_string()).await;
    let jwt_cookie = JabraCookie::decrypt(cookie).unwrap_or_default();
    if jwt_cookie.is_expired() {
        return Ok(Default::default());
    }
    let bearer = format!("Bearer {}", jwt_cookie.access_token);

    let url = option_env!("JABRAAPIGATEWAY");
    let path = format!("{}/rfq/submit_new_termsheet", url.unwrap_or_default());

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization",
        reqwest::header::HeaderValue::from_str(&bearer).unwrap(),
    );
    log::info!("request-rfq: {:?}", request);
    let response =
        call_and_parse::<SubmitNewTermSheetRequestWithGroupId, SubmitNewTermSheetResponse>(
            Some(request),
            path,
            headers,
            HttpMethod::POST,
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