// Contains commonly used utility functions for testing this application

pub use std::any::type_name;
// use std::convert::FloatToInt;
pub use std::time::{SystemTime, UNIX_EPOCH};
pub use chrono::{DateTime, TimeZone, Utc};

// Find out the type of an object
pub fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

// Get a datetime from the Unix Ephoch
pub fn convert_date(){
    let dt = Utc.timestamp(1615507200000/1000, 0).format("%Y-%m-%d").to_string();
    println!("{:?}", dt);
}

// Convert Coingecko dates from UNIX_EPOCH milliseconds to UTC date
pub fn convert_vec_date(dates: &Vec<f64>) -> Vec<String> {
    let dates_conv_ms_to_sec = dates.iter().map(|f| f/1000.0).collect::<Vec<f64>>();
    let date_list = dates_conv_ms_to_sec.iter()
    .map(|d| Utc.timestamp(d.round() as i64, 0).format("%Y-%m-%d").to_string())
    .collect();
    date_list
}
