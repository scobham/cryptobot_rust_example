mod coinstruct;
use coinstruct::CoinsMarketChart;

mod utils;
use utils::{type_of, convert_date, convert_vec_date};

mod plotting;
use plotting::{line_and_scatter_plot};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    // Perform the network request
    let res = client
        .get("https://api.coingecko.com/api/v3/coins/iota/market_chart?vs_currency=usd&days=max&interval=daily")
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

    //TEST
    // println!("{:?}", &x);
    // println!("type of x {}", type_of(&x));
    // println!("{:?}", &y);
    // println!("type of y {}", type_of(&y));
    // convert_date();

    let dates = convert_vec_date(&x);
    // println!("{:?}", &dates);

    line_and_scatter_plot(&dates, &y, "iota");

    Ok(())
}
