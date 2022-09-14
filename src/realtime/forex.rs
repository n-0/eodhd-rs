use crate::eodhd_string_float;
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use super::socket::get_n_pips;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EODHDForexRT {
    // symbol
    pub s: String,
    // ask price
    pub a: f64,
    // bid price
    pub b: f64,
    // daily change percentage (sometimes delivered as string from eodhd)
    #[serde(with = "eodhd_string_float")]
    pub dc: f64,
    //daily difference price (sometimes delivered as string from eodhd)
    #[serde(with = "eodhd_string_float")]
    pub dd: f64,
    // timestamp in milliseconds
    pub t: i64,
}

pub async fn get_n_forex_pips(
    n: u64,
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Vec<EODHDForexRT> {
    get_n_pips(n, socket).await
}
