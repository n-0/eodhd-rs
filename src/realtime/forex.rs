use crate::eodhd_string_float;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EODHDForexRT {
    /// symbol
    pub s: String,
    /// ask price
    pub a: f64,
    /// bid price
    pub b: f64,
    /// daily change percentage (sometimes delivered as string from eodhd)
    #[serde(with = "eodhd_string_float")]
    pub dc: f64,
    ///daily difference price (sometimes delivered as string from eodhd)
    #[serde(with = "eodhd_string_float")]
    pub dd: f64,
    /// timestamp in milliseconds
    pub t: i64,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EODHDCryptoRT {
    /// symbol
    pub s: Option<String>,
    /// last price
    #[serde(with = "eodhd_string_float")]
    pub p: f64,
    /// quantity of the trade 
    #[serde(with = "eodhd_string_float")]
    pub q: f64,
    /// daily change percentage (sometimes delivered as string from eodhd)
    #[serde(with = "eodhd_string_float")]
    pub dc: f64,
    ///daily difference price (sometimes delivered as string from eodhd)
    #[serde(with = "eodhd_string_float")]
    pub dd: f64,
    /// timestamp in milliseconds
    pub t: i64,
}
