pub async fn get_environment() -> String {
    let env = option_env!("PROJECT_ENVIRONMENT");
    return env.unwrap_or_default().to_string().to_uppercase();
}
