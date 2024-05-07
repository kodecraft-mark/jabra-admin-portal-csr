use crate::{commons::models::login::{DirectusLoginRequest, DirectusLoginResponse}, utilities::{cookies::{set_jabra_cookie, JabraCookie}, errors::JabraError, http_wrapper::{call_and_parse, HttpMethod}}};
use leptos::*;

pub async fn directus_login(userid: String, password: String) -> Result<bool, ServerFnError> {
    let url = option_env!("DIRECTUSURL");
    // let url = if let Ok(var) = std::env::var("DIRECTUSURL") {
    //     var
    // } else {
    //     "".to_string()
    // };
    let path = format!("{}/auth/login", url.unwrap_or_default());
    let email = userid.clone();
    let login_request = DirectusLoginRequest::new(userid.into(), password.into());
    let response = call_and_parse::<DirectusLoginRequest, DirectusLoginResponse>(
        Some(login_request),
        path,
        reqwest::header::HeaderMap::new(),
        HttpMethod::POST,
    )
    .await;

    match response {
        Ok(res) => {
            // Calculate expiration time in millis, subract 2 minute to be safe
            // Why 10 minutes? There are other api resource that are automatically when users navigate to a certain page
            // Only those API calls in action will have the refresh token
            // Which means during the manual submit, the refresh token is used
            // 10 minutes will act as a buffer for those action

            let expiration_time =
                chrono::Utc::now().timestamp_millis() + res.data.expires - 600_000;

            let jabra_cookie = JabraCookie::new(
                email,
                res.data.access_token.clone(),
                res.data.refresh_token,
                expiration_time,
            );
            set_jabra_cookie(jabra_cookie, "admin_portal_csr".to_string()).await;

            Ok(true)
        }
        Err(e) => {
            log::info!("Login Error: {}", e.to_string());
            Err(ServerFnError::ServerError(
                JabraError::LoginError.to_string(),
            ))
        }
    }
}