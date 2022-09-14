use chrono::{DateTime, NaiveDate};
use log::{debug, warn};
use serde::{Deserialize, Serialize};

use super::{env_eodhd_token, EODHDError};

const BASE_URL: &str = "https://eodhistoricaldata.com/api/eod";

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EODHDEndOfPeriod {
    pub date: NaiveDate,
    #[serde(rename = "open")]
    pub o: f64,
    #[serde(rename = "high")]
    pub h: f64,
    #[serde(rename = "low")]
    pub l: f64,
    #[serde(rename = "close")]
    pub unadjusted_close: f64,
    #[serde(rename = "adjusted_close")]
    pub c: f64,
    #[serde(rename = "volume")]
    pub v: i64,
}

pub enum EODHDPeriod {
    Daily,
    Weekly,
    Monthly,
}

impl ToString for EODHDPeriod {
    fn to_string(&self) -> String {
        match self {
            EODHDPeriod::Daily => "d".to_string(),
            EODHDPeriod::Weekly => "w".to_string(),
            EODHDPeriod::Monthly => "m".to_string(),
        }
    }
}

pub struct EODHDEndOfPeriodFilter {
    // The start date for end of period data, earliest default is 1966-07-05
    pub from: Option<chrono::NaiveDate>,
    // The end date for earnings data, if not provided, today will be used.
    pub to: Option<chrono::NaiveDate>,
    // The end date for earnings data, if not provided, daily will be used.
    pub period: Option<EODHDPeriod>,
}

pub async fn get_end_of_period(
    ticker: &str,
    filter: Option<EODHDEndOfPeriodFilter>,
) -> Result<Vec<EODHDEndOfPeriod>, EODHDError> {
    let mut url = format!(
        "{base_url}/{ticker}?api_token={token}&fmt=json",
        base_url = BASE_URL,
        ticker = ticker,
        token = env_eodhd_token()
    );
    if filter.is_some() {
        let unwraped_filter = filter.unwrap();
        if unwraped_filter.from.is_some() {
            url = format!(
                "{url}&from={from}",
                url = url,
                from = unwraped_filter.from.unwrap().format("%Y-%m-%d")
            );
        }
        if unwraped_filter.to.is_some() {
            url = format!(
                "{url}&to={to}",
                url = url,
                to = unwraped_filter.to.unwrap().format("%Y-%m-%d")
            );
        }
        if unwraped_filter.to.is_some() {
            url = format!(
                "{url}&to={to}",
                url = url,
                to = unwraped_filter.to.unwrap().format("%Y-%m-%d")
            );
        }
        if unwraped_filter.period.is_some() {
            url = format!(
                "{url}&period={period}",
                url = url,
                period = unwraped_filter.period.unwrap().to_string()
            );
        }
    }

    let request = reqwest::get(url).await;

    if request.is_err() {
        let description: &str = "request failed";
        warn!("{}", description);
        return Err(EODHDError {
            description: description.to_string(),
            inner_error: Box::new(request.err().unwrap()),
        });
    }

    let end_of_day = request.unwrap().json::<Vec<EODHDEndOfPeriod>>().await;
    if end_of_day.is_err() {
        let description: &str = "parsing end of day failed";
        warn!("{}", description);
        return Err(EODHDError {
            description: description.to_string(),
            inner_error: Box::new(end_of_day.err().unwrap()),
        });
    }
    Ok(end_of_day.unwrap())
}
