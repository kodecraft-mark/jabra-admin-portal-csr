use serde::{Deserialize, Serialize};
/// Account Overview Response
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PortfolioOverviewResponse {
    /// Total equity of the account
    pub total_equity: f64,
    /// Total realized profit and loss of the account
    pub total_realized_pnl: f64,
    /// Total live profit and loss of the account
    pub total_live_pnl: f64,
    /// Total available balance of the account
    pub total_available_balance: f64,
    /// Vector of PortfolioCurrency
    pub currencies: Vec<PortfolioCurrency>,
}

/// Represent a certain currency
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PortfolioCurrency {
    /// The name of the currency
    pub currency: String,
    /// The balance of the account with this currency
    pub balance: f64,
    /// The realized profit and loss of the account with this currency
    pub exercised_balances: f64,
    /// The live profit and loss of the account with this currency
    pub live_pnl: f64,
    /// The available balance of the account with this currency
    pub available_balance: f64,
    /// The equity of the account with this currency
    pub equity: f64,
    /// The equity of the account in USD
    pub equity_usd: f64,
    /// The accrued interest of the account with this currency
    pub interest_payments: f64,
}

impl PortfolioCurrency {
    /// Create a new PortfolioCurrency
    ///
    /// # Arguments
    ///
    /// * `currency` - The name of the currency
    /// * `balance` - The balance of the account with this currency
    /// * `exercised_balances` - The realized profit and loss of the account with this currency
    /// * `live_pnl` - The live profit and loss of the account with this currency
    /// * `available_balance` - The available balance of the account with this currency
    /// * `equity` - The equity of the account with this currency
    /// * `equity_usd` - The equity of the account in USD
    /// * `interest_payments` - The accrued interest of the account with this currency
    ///
    /// # Returns
    ///
    /// * `PortfolioCurrency` - The new PortfolioCurrency
    pub fn new(
        currency: String,
        balance: f64,
        exercised_balances: f64,
        live_pnl: f64,
        available_balance: f64,
        equity: f64,
        equity_usd: f64,
        interest_payments: f64,
    ) -> PortfolioCurrency {
        PortfolioCurrency {
            currency,
            balance,
            exercised_balances,
            live_pnl,
            available_balance,
            equity,
            equity_usd,
            interest_payments,
        }
    }
}