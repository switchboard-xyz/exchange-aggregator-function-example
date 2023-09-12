use chrono::{Utc, Duration};
use serde::Deserialize;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CoinbaseCandle {
    time: i64,
    low: f64,
    high: f64,
    open: f64,
    close: f64,
    volume: f64,
}

pub fn coinbase_close_average(candles: &Vec<CoinbaseCandle>) -> Decimal {
    if candles.len() == 0 {
        return Decimal::ZERO;
    }

    let sum: Decimal = candles.iter().map(|data| Decimal::from_f64(data.close).unwrap()).sum();

    let twap = sum / Decimal::from_usize(candles.len()).unwrap();
    twap
}
