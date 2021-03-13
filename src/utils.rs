// Contains commonly used utility functions for testing this application

pub use std::any::type_name;
pub use std::time::{SystemTime, UNIX_EPOCH};
pub use chrono::{DateTime, TimeZone, Utc};

use crate::coinstruct::CoinsMarketChart;

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

// Get coins_market_chart data
pub async fn coins_market_chart(coin_name: &str) -> Result<(Vec<std::string::String>, Vec<f64>), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    // Perform the network request
    let res = client
        .get(format!("https://api.coingecko.com/api/v3/coins/{}/market_chart?vs_currency=usd&days=max&interval=daily", coin_name))
        .send()
        .await?;

    // Parse the response body as Json with CoinsMarketChart struct
    let coin_prices = res
        .json::<CoinsMarketChart>()
        .await?;

    //TEST
    // println!("{:#?}", coin_prices.prices);
    // println!("type of coin_prices.prices{}", type_of(&coin_prices.prices));

    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    for item in &coin_prices.prices{
        x.push(item[0]);
        y.push(item[1]);
    }

    let dates = convert_vec_date(&x);


    Ok((dates, y))
}
