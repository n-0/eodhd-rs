use crate::{
    datetime::{eodhd_serde_datetime, EODHDDate},
    env_eodhd_token, BASE_URL,
};
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum EODHDEconomicEventComparison {
    #[serde(rename = "mom")]
    MOM,
    #[serde(rename = "qoq")]
    QOQ,
    #[serde(rename = "yoy")]
    YOY,
}

impl ToString for EODHDEconomicEventComparison {
    fn to_string(&self) -> String {
        (match self {
            EODHDEconomicEventComparison::MOM => "mom",
            EODHDEconomicEventComparison::QOQ => "qoq",
            EODHDEconomicEventComparison::YOY => "yoy",
        })
        .to_string()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EODHDEconomicEvent {
    #[serde(rename = "type")]
    pub type_event: String,
    pub comparison: Option<EODHDEconomicEventComparison>,
    pub country: String,
    #[serde(with = "eodhd_serde_datetime")]
    pub date: NaiveDateTime,
    pub actual: f64,
    pub previous: f64,
    pub estimate: f64,
    pub change: f64,
    pub change_percentage: f64,
}

pub async fn get_economic_events(from: EODHDDate, to: EODHDDate) -> Vec<EODHDEconomicEvent> {
    let token = env_eodhd_token();
    let url = format!(
        "{base_url}/economic-events?api_token={token}&from={from}&to={to}",
        base_url = BASE_URL,
        token = token,
        from = NaiveDate::from(from),
        to = NaiveDate::from(to),
    );
    let request = reqwest::get(url.to_owned()).await;
    if request.is_err() {
        log::error!(
            "REQUEST TO EODHD FAILED \n{:?}\n with {:?}",
            url,
            request.err()
        );
        panic!();
    }
    let parsed_events = request.unwrap().json::<Vec<EODHDEconomicEvent>>().await;
    if parsed_events.is_err() {
        log::error!("CONVERTING EODHD RESPONSE TO TEXT FAILED");
        panic!("{:?}", parsed_events.err());
    }
    parsed_events.unwrap()
}
