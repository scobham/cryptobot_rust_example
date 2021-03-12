// Plotting is where all plotter related functions reside

use plotly::{ Plot, Layout, Scatter};
use plotly::common::{Mode, Title};
use plotly::layout::{Axis, RangeSlider, RangeSelector, SelectorButton, SelectorStep, StepMode};

pub fn line_and_scatter_plot(x: &Vec<f64>, y: &Vec<f64>, name: &str) {
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
