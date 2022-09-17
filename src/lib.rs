use std::{error::Error, fmt::Display};

/// Based on [eodhd docs](
/// https://eodhistoricaldata.com/financial-apis/category/alternative-data-financial-api/)
pub mod alternative;
/// Helpful wrappers for serde to 
/// handle eodhd DateTime Formats
pub mod datetime;
pub mod end_of_period;
/// Based on [eodhd 
/// docs](https://eodhistoricaldata.com/financial-apis/stock-etfs-fundamental-data-feeds/)
pub mod fundamentals;
/// Based on [eodhd
/// docs](https://eodhistoricaldata.com/financial-apis/intraday-historical-data-api/)
pub mod historic_intraday;
/// Based on [eodhd
/// docs](https://eodhistoricaldata.com/financial-apis/live-realtime-stocks-api/)
pub mod realtime;

fn env_eodhd_token() -> String {
    if std::env::var("EODHD_TOKEN").is_err() {
        panic!("Please add EODHD_TOKEN for authentication to env");
    }
    std::env::var("EODHD_TOKEN").unwrap()
}

pub const BASE_URL: &str = "https://eodhistoricaldata.com/api";

#[derive(Debug)]
pub struct EODHDError {
    pub description: String,
    pub inner_error: Box<dyn Error + Send + Sync>,
}

impl Display for EODHDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.description.as_str())
    }
}
impl Error for EODHDError {}

/// EODHD sometimes encodes floats as strings
/// in json responses.
pub mod eodhd_string_float {
    use serde::{self, Deserialize, Deserializer, Serializer};
    pub fn serialize<S>(value: &f64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return serializer.serialize_str(value.to_string().as_str());
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let value: Result<f64, D::Error> = s.parse().map_err(serde::de::Error::custom);
        value
    }
}
