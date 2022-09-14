use crate::{
    datetime::{eodhd_serde_date, EODHDDate},
    env_eodhd_token, BASE_URL,
};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EODHDSentiment {
    #[serde(with = "eodhd_serde_date")]
    pub date: NaiveDate,
    pub count: u64,
    pub normalized: f64,
}

pub async fn get_sentiment(ticker: &str, from: EODHDDate, to: EODHDDate) -> Vec<EODHDSentiment> {
    let token = env_eodhd_token();
    let url = format!(
        "{base_url}/sentiments?api_token={token}&s={ticker}&from={from}&to={to}",
        base_url = BASE_URL,
        token = token,
        ticker = ticker,
        from = NaiveDate::from(from),
        to = NaiveDate::from(to),
    );
    println!("{:?}", url);
    let request = reqwest::get(url.to_owned()).await;
    if request.is_err() {
        log::error!(
            "REQUEST TO EODHD FAILED \n{:?}\n with {:?}",
            url,
            request.err()
        );
        panic!();
    }
    let response = request.unwrap().text().await;
    if response.is_err() {
        log::error!(
            "CONVERTING EODHD RESPONSE TO TEXT FAILED with {:?}",
            response.err()
        );
        panic!();
    }
    let jsoned: Result<serde_json::Value, serde_json::error::Error> =
        serde_json::from_str(response.unwrap().as_str());
    if jsoned.is_err() {
        log::error!(
            "{:?} CONVERTING EODHD RESPONSE TO JSON FAILED with {:?}",
            url,
            jsoned.err()
        );
        panic!();
    }
    let v = jsoned.unwrap();
    let array = v[ticker.to_owned()].to_string();
    if array == "undefined" {
        log::error!("TICKER NOT FOUND IN EODHD RESPONSE {:?}", array);
        panic!();
    }
    let parse_sentiment: Result<Vec<EODHDSentiment>, serde_json::error::Error> =
        serde_json::from_str(array.as_str());
    if parse_sentiment.is_err() {
        log::error!(
            "CONVERTING EODHD RESPONSE TO JSON FAILED with {:?}",
            parse_sentiment.err()
        );
        panic!();
    }
    parse_sentiment.unwrap()
}
