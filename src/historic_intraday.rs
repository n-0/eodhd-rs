use std::error::Error;
use serde::Deserialize;
use super::{datetime::EODHDInterval, env_eodhd_token, BASE_URL};

/// e.g.
/// Timestamp,Gmtoffset,Datetime,Open,High,Low,Close,Volume
/// 1647504300,0,"2022-03-17 08:05:00",16.86,16.868,16.775999,16.802,129473
#[derive(Clone, Debug, Deserialize)]
pub struct EODHDHistoricIntraday {
    #[serde(rename = "timestamp", default)]
    pub t: i64,
    #[serde(rename = "gmtoffset", default)]
    pub gmt: Option<u8>,
    #[serde(rename = "datetime", default)]
    pub dt: String,
    #[serde(rename = "open", default)]
    pub o: Option<f64>,
    #[serde(rename = "high", default)]
    pub h: Option<f64>,
    #[serde(rename = "low", default)]
    pub l: Option<f64>,
    #[serde(rename = "close", default)]
    pub c: Option<f64>,
    #[serde(rename = "volume", default)]
    pub v: Option<i64>,
}

impl Default for EODHDHistoricIntraday {
    fn default() -> Self {
        Self {
            t: 0,
            o: Some(0.0),
            h: Some(0.0),
            l: Some(0.0),
            c: Some(0.0),
            v: None,
            gmt: Some(0),
            dt: "".to_string(),
        }
    }
}


pub struct HistoricIntradayOptions {
    pub from: Option<i64>,
    pub to: Option<i64>,
    pub interval: EODHDInterval,
}

pub async fn get_historic_intraday(
    ticker: String,
    options: HistoricIntradayOptions,
) -> Result<Vec<EODHDHistoricIntraday>, Box<dyn Error>> {
    let token = env_eodhd_token();
    let mut url = format!(
        "{base_url}/intraday/{ticker}?api_token={token}&fmt=json&interval={interval}",
        base_url = BASE_URL,
        ticker = ticker,
        token = token,
        interval = options.interval.to_string()
    );
    if options.from.is_some() {
        url = format!("{url}&from={from}", url = url, from = options.from.unwrap());
    }
    if options.to.is_some() {
        url = format!("{url}&to={to}", url = url, to = options.to.unwrap());
    }

    let request = reqwest::get(url.clone()).await;
    if request.is_err() {
        log::error!(
            "REQUEST TO EODHD FAILED \n{:?}\n with {:?}",
            url,
            request.err()
        );
        panic!();
    }
    let pre_eodhd_ticks = request.unwrap().json::<Vec<EODHDHistoricIntraday>>().await;
    if pre_eodhd_ticks.is_err() {
        println!("{:#?}", url);
        log::error!(
            "UNABLE TO PARSE eodhd RESPONSE {:?}",
            pre_eodhd_ticks.unwrap_err()
        );
        panic!();
    }
    match pre_eodhd_ticks {
        Ok(ticks) => Ok(ticks),
        Err(e) => Err(Box::new(e))
    }
}
