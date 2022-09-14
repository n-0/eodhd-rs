use futures_util::SinkExt;
use log::{error, info};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, Message},
    MaybeTlsStream, WebSocketStream,
};

use crate::env_eodhd_token;

use super::BASE_URL_SOCKET;

/// Messages like
//Ok(Text("{\"status_code\":200,\"message\":\"Authorized\"}"))
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusMessage {
    pub status_code: u16,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EODHDSocketKind {
    Forex,
    Quote,
    Trade,
}

impl ToString for EODHDSocketKind {
    fn to_string(&self) -> String {
        match self {
            EODHDSocketKind::Forex => "forex".to_string(),
            EODHDSocketKind::Quote => "us-quote".to_string(),
            EODHDSocketKind::Trade => "us".to_string(),
        }
    }
}

pub async fn subscribe_rt(
    ticker: &str,
    kind: EODHDSocketKind,
) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
    let token = env_eodhd_token();
    let url_string = format!(
        "{base_url}/{kind}?api_token={api_token}",
        base_url = BASE_URL_SOCKET,
        kind = kind.to_string(),
        api_token = token
    );
    let url = url::Url::parse(&url_string).unwrap();
    let subscribe_msg = format!(
        "{{\"action\": \"subscribe\", \"symbols\": \"{symbol}\"}}",
        symbol = ticker
    );
    let message = Message::text(subscribe_msg.to_string());
    let (mut socket, _response) = connect_async(url).await.expect("Failed to connect");
    let socket_init = socket.send(message).await;
    if socket_init.is_err() {
        error!("Unable to subscribe for symbol {:?}", ticker);
        panic!("{:?}", socket_init.err());
    }
    socket
}

pub async fn unsubscribe_rt(
    ticker: &str,
    mut socket: WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Result<(), tungstenite::error::Error> {
    let unsubscribe_msg = format!(
        "{{\"action\": \"unsubscribe\", \"symbols\": \"{symbol}\"}}",
        symbol = ticker
    );
    let message = Message::text(unsubscribe_msg.to_string());
    let unsubscribe_res = socket.send(message).await;
    if unsubscribe_res.is_err() {
        error!("Unsubscribe for ticker {} failed", ticker);
        return Err(unsubscribe_res.err().unwrap());
    }
    let close_res = socket.close(None).await;
    if close_res.is_err() {
        error!("Closing socket for ticker {} failed", ticker);
        return Err(close_res.err().unwrap());
    }
    Ok(())
}

pub async fn get_n_pips<T>(
    n: u64,
    socket: &mut WebSocketStream<MaybeTlsStream<TcpStream>>,
) -> Vec<T>
where
    T: DeserializeOwned,
{
    let mut counter = 0;
    let mut pips: Vec<T> = vec![];
    while let Some(res) = socket.next().await {
        if res.is_err() {
            error!("There was an error while reading forex");
        }
        let message = res.unwrap();
        match message {
            tungstenite::Message::Ping(_) => {
                let pong = socket.send(Message::Pong(vec![])).await;
                if pong.is_err() {
                    error!("There was an error while reading forex");
                    panic!()
                }
                continue;
            }
            tungstenite::Message::Text(pip_string) => {
                if pip_string.contains("status_code") {
                    let status_message_parsed = serde_json::from_str::<StatusMessage>(&pip_string);
                    if status_message_parsed.is_ok() {
                        let status_message = status_message_parsed.unwrap();
                        info!(
                            "Forex status {:?} with message {:?}",
                            status_message.status_code, status_message.message
                        );
                    } else {
                        error!("There was an error while reading forex");
                        panic!()
                    }
                    continue;
                }
                let parsed_pip = serde_json::from_str::<T>(pip_string.as_str());
                if parsed_pip.is_err() {
                    error!("Unable to decode forex");
                    panic!("{:?}", parsed_pip.err())
                }
                pips.push(parsed_pip.unwrap());
                counter += 1;
            }
            _ => {
                error!("There was an error while reading forex");
                panic!()
            }
        }

        if counter == n {
            break;
        }
    }
    pips
}
