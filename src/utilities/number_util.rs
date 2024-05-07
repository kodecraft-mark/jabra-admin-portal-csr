use currency_rs::{Currency, CurrencyOpts};
use num_format::{Locale, ToFormattedString};

pub fn format_currency(number: f64, scale: u8) -> String {
    format!("{:.*}", usize::from(scale), number)
}

pub fn format_currency_with_scale(number: f64, scale: u8, separator: &str) -> String {
    let value = format_currency(number, scale);
    format_money(value, separator, scale)
}

pub fn format_money(value: String, separator: &str, precision: u8) -> String {
    let opts: CurrencyOpts = CurrencyOpts::new()
        .set_separator(separator)
        .set_symbol("")
        .set_decimal(".");
    let opts = opts.set_precision(precision as i64);
    let currency = Currency::new_string(value.as_str(), Some(opts)).unwrap();
    currency.format()
}
pub fn format_number_en(number_str: String, precision: usize) -> String {
    let test_number = number_str.parse::<f64>();
    if test_number.is_err() {
        return number_str;
    }
    let parts = number_str.split('.').collect::<Vec<&str>>();
    if parts.len() < 2 {
        let f = parts[0].to_string().parse::<i64>().unwrap();
        let formatted_f = f.to_formatted_string(&Locale::en);
        let decimal_places = format!("{:.*}", precision, "000000000");
        let concat = format!("{}.{}", formatted_f, decimal_places);
        concat
    } else {
        let f = parts[0].to_string().parse::<i64>().unwrap();
        let formatted_f = f.to_formatted_string(&Locale::en);
        let decimal_places = format!("{:.*}", precision, parts[1].to_string());
        let concat = format!("{}.{}", formatted_f, decimal_places);
        concat
    }
}