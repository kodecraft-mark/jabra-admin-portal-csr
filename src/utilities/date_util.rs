use chrono::{
    DateTime, Duration, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone,
    Timelike, Utc,
};
use num_format::{Locale, ToFormattedString};

pub fn format_date(date_time: DateTime<Utc>) -> String {
    let formatted_time = date_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    formatted_time
}
pub fn calculate_time_difference(
    date_created: Option<String>,
    expiry: String,
    gtc: bool,
) -> String {
    if gtc {
        return "Good Till Canceled".to_string();
    }
    let start = match date_created {
        Some(s) => parse_timestamp(&s),
        None => Some(Utc::now().naive_utc()),
    };
    let end = parse_timestamp(&expiry);

    match (start, end) {
        (Some(s), Some(e)) => {
            let duration = e.signed_duration_since(s);

            if duration.num_seconds() < 0 {
                return "Expired".to_string();
            }

            let hours = duration.num_hours();
            let minutes = (duration.num_minutes() - (hours * 60)).abs();
            let seconds = (duration.num_seconds() - (minutes * 60)).abs();

            format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
        }
        _ => "Expired".to_string(),
    }
}
pub fn convert_utc_to_local(timestamp: &str) -> String {
    let date_time = parse_timestamp(timestamp);
    match date_time {
        Some(dt) => {
            // let utc = Utc.offset_from_utc_datetime(&dt);
            let local = Local.from_utc_datetime(&dt);
            let formatted_datetime = local.format("%Y-%m-%d %H:%M:%S").to_string();
            formatted_datetime
        }
        None => "".to_string(),
    }
}
pub fn parse_str_to_utc_datetime_str(input: &str) -> String {
    // let date_time = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S");
    // match date_time {
    //     Ok(dt) => {
    //         let utc = Utc.from_utc_datetime(&dt);
    //         let formatted_datetime = utc.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    //         formatted_datetime
    //     }
    //     _ => "".to_string(),
    // }
    let formats = [
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S", // First format
        "%Y-%m-%dT%H:%M",    // Second format without seconds
    ];

    // Attempt to parse the timestamp using each format
    for format in &formats {
        if let Ok(parsed_dt) = NaiveDateTime::parse_from_str(input, format) {
            let utc: NaiveDateTime = Local.from_local_datetime(&parsed_dt).unwrap().naive_utc();
            // let utc = Utc.from_utc_datetime(&parsed_dt);
            let formatted_datetime = utc.format("%Y-%m-%dT%H:%M:%SZ").to_string();
            return formatted_datetime;
        }
    }

    String::from("")
}
pub fn parse_timestamp(timestamp: &str) -> Option<NaiveDateTime> {
    // Define the expected timestamp formats
    let formats = [
        "%Y-%m-%dT%H:%M:%S%.3fZ", // First format with milliseconds
        "%Y-%m-%dT%H:%M:%S",
        "%Y-%m-%d %H:%M:%S", // First format
        "%Y-%m-%dT%H:%M",    // Second format without milliseconds
    ];

    // Attempt to parse the timestamp using each format
    for format in &formats {
        if let Ok(parsed_dt) = NaiveDateTime::parse_from_str(timestamp, format) {
            return Some(parsed_dt);
        }
    }

    None // Return None if parsing failed with all formats
}
pub fn extract_date(date_str: String) -> String {
    // Parse the input date string
    let parsed_date = NaiveDateTime::parse_from_str(date_str.as_str(), "%Y-%m-%dT%H:%M:%S%.3fZ")
        .expect("Invalid date format");
    // Format the date as "Mon Day Year"
    let formatted_date = parsed_date.format("%b %d %Y").to_string();
    formatted_date
}
pub fn time_to_expiry(date_expiry: &str) -> f64 {
    let utc = Utc::now();
    let difference = format_date_datetime(date_expiry.to_string()) - utc;
    let temp_ttm = difference.num_seconds() as f64 / 86400.0;
    (temp_ttm * 100.0).round() / 100.0
}
pub fn format_date_datetime(date_str: String) -> DateTime<chrono::Utc> {
    match NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S.%3fZ") {
        Ok(dt) => {
            let datetime_utc = Utc.from_utc_datetime(&dt);
            datetime_utc
        }
        Err(e) => {
            println!("Error parsing date: {:?}", e);
            Utc::now()
        }
    }
}

pub fn format_utc_str_to_local_str(utc_date_str: String) -> String {
    let utc_str_to_naive_dt =
        NaiveDateTime::parse_from_str(utc_date_str.as_str(), "%Y-%m-%dT%H:%M:%S%.3fZ");

    // log::info!("utc_date_str : {:?}", utc_date_str);

    // log::info!("utc_str_to_naive_dt : {:?}", utc_str_to_naive_dt);

    if let Ok(d) = utc_str_to_naive_dt {
        let utc = d.and_utc();
        let utc_local = utc.with_timezone(&Local);
        let formatted_local = utc_local.format("%Y-%m-%d %H:%M:%S").to_string();

        return formatted_local;
    }

    String::from("")
}
pub fn get_expiry(expiry_in_minutes: u16) -> String {
    // if expiry_in_minutes == 0 {
    //     return "".to_string();
    // }
    let current_time = Utc::now();
    let expiry_time = current_time + Duration::minutes(i64::from(expiry_in_minutes));
    let formatted_time = expiry_time.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    formatted_time
}
pub fn get_expiration_datetime_utc(
    start_date: Option<DateTime<Utc>>,
    expiration_in_day: f64,
) -> DateTime<Utc> {
    // Calculate the total seconds (1 day = 86400 seconds)
    let seconds_in_1_day: f64 = 86400.0;
    let total_seconds: i64 = (seconds_in_1_day * expiration_in_day) as i64;

    // Create a duration of the calculated seconds
    let expiration_duration: Duration = Duration::seconds(total_seconds);

    // Add the duration to the current date
    match start_date {
        Some(dt) => dt + expiration_duration,
        None => Utc::now() + expiration_duration,
    }
    // let expiration_date: DateTime<Utc> = start_date + expiration_duration;

    // expiration_date
}
pub fn convert_utc_to_edt(dt: DateTime<Utc>) -> DateTime<FixedOffset> {
    let edt_offset = FixedOffset::east_opt(-4 * 3600); // EDT offset is UTC-4
    dt.with_timezone(&edt_offset.unwrap())
}
pub fn update_edt_time(dt: DateTime<FixedOffset>, new_time: &str) -> DateTime<FixedOffset> {
    // Parse the new_time string
    let new_time_parts: Vec<&str> = new_time.split(':').collect();
    if new_time_parts.len() != 3 {
        panic!("Invalid time format");
    }

    let new_hour: u32 = new_time_parts[0].parse::<u32>().expect("Invalid hour");
    let new_minute: u32 = new_time_parts[1].parse::<u32>().expect("Invalid minute");
    let new_second: u32 = new_time_parts[2].parse::<u32>().expect("Invalid second");

    // Create a new DateTime with the updated time
    let result = dt
        .with_hour(new_hour)
        .expect("Invalid hour")
        .with_minute(new_minute)
        .expect("Invalid minute")
        .with_second(new_second)
        .expect("Invalid second");
    result
}

///Accepts Date time in UTC and convert to local time to get time
pub fn get_time_in_local_time(utc_date_time: String) -> String {
    let utc_date_time = parse_timestamp(&utc_date_time);
    match utc_date_time {
        Some(dt) => {
            let local = Local.from_utc_datetime(&dt);
            let formatted_datetime = local.format("%H:%M:%S").to_string();
            formatted_datetime
        }
        None => "".to_string(),
    }
}
///Accepts Date time in UTC and convert to local time to get date_time
pub fn get_datetime_in_local_time(utc_date_time: String) -> String {
    let utc_date_time = parse_timestamp(&utc_date_time);
    match utc_date_time {
        Some(dt) => {
            let local = Local.from_utc_datetime(&dt);
            let formatted_datetime = local.format("%Y-%m-%d %H:%M:%S").to_string();
            formatted_datetime
        }
        None => "".to_string(),
    }
}
///Accepts Date time in UTC and convert to local time to get date
pub fn get_date_in_local_time(utc_date_time: String) -> String {
    let utc_date_time = parse_timestamp(&utc_date_time);
    match utc_date_time {
        Some(dt) => {
            let local = Local.from_utc_datetime(&dt);
            let formatted_datetime = local.format("%Y-%m-%d").to_string();
            formatted_datetime
        }
        None => "".to_string(),
    }
}


pub fn get_current_local_time() -> String {

    let utc = Utc::now();
    let utc_local = utc.with_timezone(&Local);
    let formatted_local = utc_local.format("%Y-%m-%d %H:%M:%S").to_string();

    return formatted_local;
}