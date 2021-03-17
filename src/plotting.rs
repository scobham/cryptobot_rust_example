
use std::borrow::Borrow;

use crate::utils::{coins_market_chart};
use chrono::{Utc};


// Plotting is where all plotter related functions reside

use plotly::{ Plot, Layout, Scatter, Bar};
use plotly::common::{Mode, Title};
use plotly::layout::{Axis, RangeSlider, RangeSelector, SelectorButton, SelectorStep, StepMode, LayoutGrid, GridPattern, BarMode};

// pub fn line_and_scatter_plot(x: &Vec<f64>, y: &Vec<f64>, name: &str) {
pub fn line_and_scatter_plot(x: &Vec<String>, y: &Vec<f64>, coin_name: &str) {
    let _x = x.to_vec();
    let _y = y.to_vec();
    // let date = data.iter().map(|d| d.date.clone()).collect();
    // let high = data.iter().map(|d| d.high).collect();

    let trace1 = Scatter::new(_x, _y)
        .name("trace1")
        .mode(Mode::LinesMarkers)
        .name(&coin_name.to_uppercase())
        .show_legend(true);
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
                .count(3)
                .label("3m")
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

pub async fn series_line_and_scatter_plot(coins: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // let mut prices = Vec::new();
    let mut plot_price = Plot::new();
    let mut plot_volume = Plot::new();
    for coin in &coins[1..] {
        let (dates, prices , volumes) = coins_market_chart(coin).await?;
        let date = dates.clone();

        let trace = Scatter::new(dates, prices)
        // .name("trace1")
        .mode(Mode::LinesMarkers)
        .name(&coin.to_uppercase())
        // .x_axis(format!("x-trace-{}", &coin).as_str())
        // .y_axis(format!("y-trace-{}", &coin).as_str())
        .x_axis("x-trace")
        .y_axis("y-trace")
        .show_legend(true);

        let bar = Bar::new(date, volumes)
        .name(format!("{} Volume", &coin.to_uppercase()).as_str())
        // .x_axis(format!("x-bar-{}", &coin).as_str())
        // .y_axis(format!("y-bar-{}", &coin).as_str())
        .x_axis("x-bar")
        .y_axis("y-bar")
        ;


        plot_price.add_trace(trace);
        plot_volume.add_trace(bar);
        // plot.add_traces(vec!(trace, bar));

    }

    // TEST
    // println!("{:#?}", &prices);
    
    let layout_price = Layout::new()
    // .grid(
    //     LayoutGrid::new()
    //     .rows(1)
    //     .columns(2)
    //     .pattern(GridPattern::Independent),
    // )
    .title(Title::new("Coin Price History"))
    

    .x_axis(Axis::new()
        .domain(&[0., 1.])
        .range_slider(RangeSlider::new().visible(true))
        .range_selector(RangeSelector::new().buttons(vec![
            SelectorButton::new()
                .count(1)
                .label("1m")
                .step(SelectorStep::Month)
                .step_mode(StepMode::Backward),
            SelectorButton::new()
                .count(3)
                .label("3m")
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
    )
    // .y_axis(Axis::new().domain(&[0.5, 1.]).anchor("y-trace"))

    // .x_axis2(Axis::new().domain(&[0., 1.]).anchor("x-trace"))
    // .y_axis2(Axis::new().domain(&[0.3, 0.45]).anchor("y-bar"))

    // .x_axis3(Axis::new().domain(&[0., 1.]).anchor("x-bar"))
    // .y_axis3(Axis::new().domain(&[0., 0.25]).anchor("y-bar"))
    // .y_axis(Axis::new().position(0.2))
    // .y_axis2(Axis::new().domain(&[0.2, 0.45]).anchor("free").overlaying("y-bar"))
    // .y_axis2(Axis::new().position(0.2).anchor("free").overlaying("y-trace"))
    // .y_axis3(Axis::new().domain(&[0.5, 1.]).anchor("free").overlaying("y-trace"))
    // .y_axis3(Axis::new().position(0.6).anchor("free").overlaying("y-trace"))
    ;

    let layout_volume = Layout::new()
    .title(Title::new("Coin Market Volume History"))
    .bar_mode(BarMode::Group)
    .x_axis(Axis::new()
    .domain(&[0., 1.])
    .range_slider(RangeSlider::new().visible(true))
    .range_selector(RangeSelector::new().buttons(vec![
        SelectorButton::new()
            .count(1)
            .label("1m")
            .step(SelectorStep::Month)
            .step_mode(StepMode::Backward),
        SelectorButton::new()
            .count(3)
            .label("3m")
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
    )
    ;

    plot_price.set_layout(layout_price);
    plot_volume.set_layout(layout_volume);

    plot_price.show();
    plot_volume.show();

    Ok(())
}
