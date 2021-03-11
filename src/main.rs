use std::collections::HashMap;
use std::any::type_name;
use futures::TryFutureExt;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CoinsMarketChart {
    prices: Vec<[f64; 2]>,
    market_caps: Vec<[f64; 2]>,
    total_volumes: Vec<[f64; 2]>
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    // Perform the network request
    let res = client
        .get("https://api.coingecko.com/api/v3/coins/iota/market_chart?vs_currency=usd&days=3&interval=daily")
        .send()
        .await?;

    // Parse the response body as Json with CoinsMarketChart struct
    let coin_prices = res
        .json::<CoinsMarketChart>()
        .await?;

    println!("{:#?}", coin_prices.prices);
    println!("type of coin_prices.prices{}", type_of(&coin_prices.prices));

    let mut map = HashMap::<f32, f32>::new();

    // for item in coin_prices.prices.iter() {
    //     println!("==============\n{:?}", &item);
    //     map.insert(&item[0], &item[1]);
    // }

    // println!("Map {:#?}", map);

    Ok(())
}
