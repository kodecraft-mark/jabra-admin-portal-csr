pub fn get_conditional_loss(
    option_kind: String,
    stop_loss_level: f64,
    ccy1_amount: f64,
    ccy2_premium: f64,
    strike: f64,
    counterparty_name: String,
    currency: String,
) -> String {
    if option_kind == "Call".to_string() {
        format! {
            "If the value of the {:.2} CE exceeds ${:.2} for {:.2} {} notional, JABRA TRADING LLC will execute a market order and terminate the contract early. {} will owe the difference between the closeout price and ${:.2} to JABRA TRADING LLC.",
            strike, stop_loss_level, ccy1_amount, currency, counterparty_name, ccy2_premium
        }
    } else {
        format! {
            "If the value of the {:.2} PE exceeds ${:.2} for {:.2} {} notional, JABRA TRADING LLC will execute a market order and terminate the contract early. {} will owe the difference between the closeout price and ${:.2} to JABRA TRADING LLC.",
            strike, stop_loss_level, ccy1_amount, currency, counterparty_name, ccy2_premium
        }
    }
}