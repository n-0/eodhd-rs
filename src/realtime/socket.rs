use futures_util::{SinkExt, StreamExt};
use log::{error, info, debug};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, Message},
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
    Crypto,
    Quote,
    Trade,
}

impl ToString for EODHDSocketKind {
    fn to_string(&self) -> String {
        match self {
            EODHDSocketKind::Forex => "forex".to_string(),
            EODHDSocketKind::Crypto => "crypto".to_string(),
            EODHDSocketKind::Quote => "us-quote".to_string(),
            EODHDSocketKind::Trade => "us".to_string(),
        }
    }
}

/// A struct owning the necessary
/// parts to handle a websocket 
/// realtime connection
#[derive(Debug)]
pub struct EODHDRTChannels<T: std::fmt::Debug + Clone + Send + Sync + DeserializeOwned> {
    /// tokio_tunsgtenites messages can be send to the socket via this channel
    /// you don't have to do this unless, there are new features in the API
    /// that are not supported by this library yet.
    pub message_channel: tokio::sync::mpsc::Sender<tokio_tungstenite::tungstenite::Message>,
    /// receives the actual ticks of the asset from the socket 
    pub tick_channel: tokio::sync::mpsc::Receiver<T>
}

pub async fn create_socket_channel<T: 
    std::fmt::Debug + 
    Clone 
    + Send 
    + Sync 
    + DeserializeOwned 
    + 'static
>(
    capacity: usize,
    kind: EODHDSocketKind
) -> Result<EODHDRTChannels<T>, Box<dyn std::error::Error + Send + Sync>> {

    let (tick_tx, tick_tr) = tokio::sync::mpsc::channel(capacity);
    let (message_tx, mut message_tr) = tokio::sync::mpsc::channel::<tokio_tungstenite::tungstenite::Message>(capacity);

    let token = env_eodhd_token();
    let url_string = format!(
        "{base_url}/{kind}?api_token={api_token}",
        base_url = BASE_URL_SOCKET,
        kind = kind.to_string(),
        api_token = token
    );

    let url = url::Url::parse(&url_string).unwrap();
    match connect_async(url).await {
        Ok((socket, _response)) => {
                    let (mut socket_tx, mut socket_tr) = socket.split();

                    let message_tx_clone = message_tx.clone();
                    // task handles new (un)subscriptions and other messages
                    tokio::spawn(async move {
                            while let Some(message) = message_tr.recv().await {
                                match socket_tx.send(message).await {
                                    Ok(o) => {
                                        info!("Send socket a message {:?}", o);
                                    },
                                    Err(e) => {
                                        error!("There was an error while reading from the socket {:#?}", e);
                                    }
                                }
                            }
                    });
                    // saves new ticks and takes care 
                    // of messages the user doesn't need
                    tokio::spawn(async move {
                        while let Some(res) = socket_tr.next().await {
                            match res {
                                Err(e) => {
                                    error!("There was an error while reading from the socket {:#?}", e);
                                },
                                Ok(message) => {
                                    debug!(
                                        "Socket got a message {:?}",
                                        message 
                                    );
                                        match message {
                                            tungstenite::Message::Ping(_) => {
                                                let pong = message_tx_clone.send(Message::Pong(vec![])).await;
                                                if let Err(e) = pong {
                                                    error!("There was an error while reading from the socket {:#?}", e);
                                                }
                                            }
                                            tungstenite::Message::Text(pip_string) => {
                                                if pip_string.contains("status_code") {
                                                    let status_message_parsed = serde_json::from_str::<StatusMessage>(&pip_string);
                                                    match status_message_parsed {
                                                        Ok(status_message) => {
                                                            info!(
                                                                "Socket status {:?} with message {:?}",
                                                                status_message.status_code, status_message.message
                                                            );
                                                        },
                                                        Err(e) => {
                                                            error!("There was an error while reading from the socket {:#?}", e);
                                                        }
                                                    }
                                                    continue;
                                                }
                                                let parsed_pip = serde_json::from_str::<T>(pip_string.as_str());
                                                match parsed_pip {
                                                    Ok(parsed_pip) => {
                                                        match tick_tx.send(parsed_pip.clone()).await {
                                                            Ok(_) => debug!("Send a pip over tick channel {:#?}", parsed_pip),
                                                            Err(e) => {
                                                                error!("error while sending pip {:#?}", e);
                                                            },
                                                        }

                                                    },
                                                    Err(e) => {
                                                        error!("Unable to decode message from socket {:#?}", e);
                                                    }
                                                }
                                            },
                                            unknown => {
                                                error!("There was an unknown type in the socket {:#?}", unknown);
                                            }
                                        }
                                    }
                                
                                }
                            }
                    });
        },
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    Ok(EODHDRTChannels {
        message_channel: message_tx,
        tick_channel: tick_tr,
    })
}

pub async fn subscribe_rt<T: std::fmt::Debug + Clone + Send + Sync + DeserializeOwned>(
    ticker: &str,
    channel: &mut EODHDRTChannels<T>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscribe_msg = format!(
        "{{\"action\": \"subscribe\", \"symbols\": \"{symbol}\"}}",
        symbol = ticker
    );
    let message = Message::text(subscribe_msg.to_string());
    let response = channel.message_channel.send(message).await;
    if let Err(e) = response {
        error!("Unable to subscribe for symbol {:?}", ticker);
        return Err(Box::new(e))
    }
    Ok(())
}

pub async fn unsubscribe_rt<T: 
    std::fmt::Debug + 
    Clone 
    + Send 
    + Sync 
    + DeserializeOwned 
    + 'static
>(
    ticker: &str,
    channel: &mut EODHDRTChannels<T>
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let unsubscribe_msg = format!(
        "{{\"action\": \"unsubscribe\", \"symbols\": \"{symbol}\"}}",
        symbol = ticker
    );
    let message = Message::text(unsubscribe_msg.to_string());
    let unsubscribe_res = channel.message_channel.send(message).await;
    if let Err(e) = unsubscribe_res {
        error!("Unsubscribe for ticker {} failed", ticker);
        return Err(Box::new(e));
    }
    Ok(())
}
