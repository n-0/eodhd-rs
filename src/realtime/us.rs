use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EODHDUSTrade {
    // ticker code
    pub s: String,
    // price
    pub p: f64,
    // timestamp in milliseconds
    pub t: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EODHDUSQuote {
    // ticker code
    pub s: String,
    // ask price
    pub ap: f64,
    // ask size
    #[serde(rename = "as")]
    pub av: f64,
    // bid price
    pub bp: f64,
    // bid size
    #[serde(rename = "bs")]
    pub bv: f64,
    // timestamp in milliseconds
    pub t: i64,
}
