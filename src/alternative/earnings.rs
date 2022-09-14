use chrono::NaiveDate;
use log::{debug, warn};
use serde::{Deserialize, Serialize};

use crate::{datetime::eodhd_serde_date, env_eodhd_token, EODHDError};

const BASE_URL: &str = "https://eodhistoricaldata.com/api/calendar/earnings";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EODHDBeforeAfterMarket {
    BeforeMarket,
    AfterMarket,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EODHDEarnings {
    pub code: String,
    pub currency: Option<String>,
    #[serde(with = "eodhd_serde_date")]
    pub report_date: NaiveDate,
    pub before_after_market: Option<EODHDBeforeAfterMarket>,
    pub actual: Option<f64>,
    pub estimate: Option<f64>,
    pub difference: Option<f64>,
    pub percent: Option<f64>,
}

pub struct EODHDEarningsFilter {
    /// The start date for earnings data, if not provided, today will be used.
    from: Option<chrono::NaiveDate>,
    /// The end date for earnings data, if not provided, today + 7 days will be used.
    to: Option<chrono::NaiveDate>,
    /// or tickers
    symbols: Option<Vec<String>>,
}

pub async fn get_earnings(
    filter: Option<EODHDEarningsFilter>,
) -> Result<Vec<EODHDEarnings>, EODHDError> {
    let mut url = format!(
        "{base_url}?api_token={token}&fmt=json",
        base_url = BASE_URL,
        token = env_eodhd_token()
    );
    if filter.is_some() {
        let unwrapped_filter = filter.unwrap();
        if unwrapped_filter.from.is_some() {
            url = format!(
                "{url}&from={from}",
                url = url,
                from = unwrapped_filter.from.unwrap().format("%Y-%m-%d")
            );
        }
        if unwrapped_filter.to.is_some() {
            url = format!(
                "{url}&to={to}",
                url = url,
                to = unwrapped_filter.to.unwrap().format("%Y-%m-%d")
            );
        }
        if unwrapped_filter.symbols.is_some() {
            let stringed_symbols = unwrapped_filter
                .symbols
                .unwrap()
                .iter()
                .fold("".to_string(), |acc, sym| format!("{},{}", acc, sym));
            url = format!(
                "{url}&symbols={symbols}",
                url = url,
                symbols = stringed_symbols
            );
        }
    }

    debug!("{}", url);
    let request = reqwest::get(url).await;

    if request.is_err() {
        let description: &str = "miner::eodhd::alternative::earnings request failed";
        warn!("{}", description);
        return Err(EODHDError {
            description: description.to_string(),
            inner_error: Box::new(request.err().unwrap()),
        });
    }
    let text = request.unwrap().text().await;
    if text.is_err() {
        let description: &str = "miner::eodhd::alternative::earnings reading response failed";
        warn!("{}", description);
        return Err(EODHDError {
            description: description.to_string(),
            inner_error: Box::new(text.err().unwrap()),
        });
    }
    let jsoned = serde_json::from_str::<serde_json::Value>(text.unwrap().as_str());
    if jsoned.is_err() {
        let description: &str =
            "miner::eodhd::alternative::earnings parsing json from response failed";
        warn!("{}", description);
        return Err(EODHDError {
            description: description.to_string(),
            inner_error: Box::new(jsoned.err().unwrap()),
        });
    }
    let mut eodhd_earnings: Vec<EODHDEarnings> = vec![];
    let jsoned_unwraped = jsoned.unwrap();
    let jsoned_earnings = jsoned_unwraped.get("earnings");
    if jsoned_earnings.is_none() {
        let description: &str = "miner::eodhd::alternative::earnings found no earnings";
        warn!("{}", description);
        return Ok(eodhd_earnings);
    }
    for jsoned_earnings in jsoned_earnings.unwrap().as_array().unwrap() {
        let earning = EODHDEarnings::deserialize(jsoned_earnings);
        if earning.is_err() {
            let description: &str =
                "miner::eodhd::alternative::earnings failed to deserialize earnings";
            warn!("{}\n{}", description, earning.err().unwrap());
            return Ok(eodhd_earnings);
        }
        eodhd_earnings.push(earning.unwrap());
    }
    Ok(eodhd_earnings)
}
