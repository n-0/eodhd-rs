use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

use super::socket::get_n_pips;

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

pub async fn get_n_us_trade(
    n: u64,
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Vec<EODHDUSTrade> {
    get_n_pips(n, socket).await
}

pub async fn get_n_us_quote(
    n: u64,
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Vec<EODHDUSQuote> {
    get_n_pips(n, socket).await
}
