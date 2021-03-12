use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct CoinsMarketChart {
    pub prices: Vec<[f64; 2]>,
    pub market_caps: Vec<[f64; 2]>,
    pub total_volumes: Vec<[f64; 2]>
}