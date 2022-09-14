use serde::{Deserialize, Serialize};

use crate::{env_eodhd_token, BASE_URL};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EODHDDelayed {
    pub code: String,
    pub timestamp: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    #[serde(rename = "previousClose")]
    pub previous_close: f64,
    pub change: f64,
    pub change_p: f64,
}

pub async fn get_delayed(ticker: &str) -> EODHDDelayed {
    let token = env_eodhd_token();
    let url = format!(
        "{base_url}/real-time/{symbol}?fmt=json&api_token={token}",
        base_url = BASE_URL,
        symbol = ticker,
        token = token
    );

    println!("{:?}", url);
    let request = reqwest::get(url.clone()).await;
    if request.is_err() {
        log::error!(
            "REQUEST TO EODHD DELAYED FAILED \n{:?}\n with {:?}",
            url,
            request.err()
        );
        panic!();
    }
    let parsed_delayed = request.unwrap().json::<EODHDDelayed>().await;
    if parsed_delayed.is_err() {
        log::error!("PARSING DELAYED EODHD FAILED {:?}", parsed_delayed.err());
        panic!();
    }
    parsed_delayed.unwrap()
}
