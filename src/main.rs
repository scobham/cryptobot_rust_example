use std::env;

mod coinstruct;
use coinstruct::CoinsMarketChart;

mod utils;
use utils::{type_of, convert_date, convert_vec_date, coins_market_chart};

mod plotting;
use plotting::{line_and_scatter_plot, series_line_and_scatter_plot};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {  

    let iota_prices = coins_market_chart("iota").await?;
    let cardano_prices = coins_market_chart("cardano").await?;
    let algorand_prices = coins_market_chart("algorand").await?;

    // line_and_scatter_plot(&dates, &y, "iota");
    // line_and_scatter_plot(&iota_prices.0, &iota_prices.1, "iota");
    // line_and_scatter_plot(&cardano_prices.0, &cardano_prices.1, "cardano");
    // line_and_scatter_plot(&algorand_prices.0, &algorand_prices.1, "algorand");

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    series_line_and_scatter_plot(args).await?;


    Ok(())
}
