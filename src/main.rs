mod coinstruct;
use coinstruct::CoinsMarketChart;

mod utils;
use utils::type_of;

use std::fmt;
use std::collections::HashMap;
// use std::any::type_name;
use futures::TryFutureExt;
use serde::{Deserialize, Deserializer, Serialize};


// Testing Plotly.rs
use plotly::{ Plot, Layout, Scatter};
use plotly::common::{Mode, Title};
use plotly::layout::{Axis, RangeSlider, RangeSelector, SelectorButton, SelectorStep, StepMode};

fn line_and_scatter_plot(x: &Vec<f64>, y: &Vec<f64>, name: &str) {
    let _x = x.to_vec();
    let _y = y.to_vec();
    // let date = data.iter().map(|d| d.date.clone()).collect();
    // let high = data.iter().map(|d| d.high).collect();

    let trace1 = Scatter::new(_x, _y)
        .name("trace1")
        .mode(Mode::LinesMarkers)
        .name(&name.to_uppercase());
    // let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
    //     .name("trace2")
    //     .mode(Mode::Lines);
    // let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12]).name("trace3");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    // plot.add_trace(trace2);
    // plot.add_trace(trace3);

    let layout = Layout::new()
        .title(Title::new("Coin Price History"))
        .x_axis(Axis::new().range_slider(RangeSlider::new().visible(true))
        .range_selector(RangeSelector::new().buttons(vec![
            SelectorButton::new()
                .count(1)
                .label("1m")
                .step(SelectorStep::Month)
                .step_mode(StepMode::Backward),
            SelectorButton::new()
                .count(6)
                .label("6m")
                .step(SelectorStep::Month)
                .step_mode(StepMode::Backward),
            SelectorButton::new()
                .count(1)
                .label("YTD")
                .step(SelectorStep::Year)
                .step_mode(StepMode::ToDate),
            SelectorButton::new()
                .count(1)
                .label("1y")
                .step(SelectorStep::Year)
                .step_mode(StepMode::Backward),
            SelectorButton::new().step(SelectorStep::All),
        ])),
        );
    plot.set_layout(layout);

    plot.show();
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build the client using the builder pattern
    let client = reqwest::Client::builder()
        .build()?;

    // Perform the network request
    let res = client
        .get("https://api.coingecko.com/api/v3/coins/iota/market_chart?vs_currency=usd&days=30&interval=daily")
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

    // let mut x: Vec<_> = &coin_prices.prices.iter()[0];
    // println!("X\n{:?}", x);
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    for item in &coin_prices.prices{
        // println!("==========\n{:?}", item[0]);
        x.push(item[0]);
        y.push(item[1]);
    }

    println!("{:?}", &x);
    println!("type of x {}", type_of(&x));
    println!("{:?}", &y);
    println!("type of y {}", type_of(&y));

    line_and_scatter_plot(&x, &y, "iota");

    Ok(())
}
