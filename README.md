# eodhd-rs 💹 
## [![Latest Version]][crates.io]  [![Docs.rs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)][docs.rs]

[crates.io]: https://crates.io/crates/eodhd_rs
[docs.rs]: https://docs.rs/eodhd_rs

The community rust wrapper around the eodhd API!
You're building the next Warren Buffet as an AI
or consulting the central bank on their next step?
Well if you love rust, then you're in luck, because
the necessary data is obtainable via [eodhd](https://eodhistoricaldata.com/)
and this repository has some convenient wrapper structs/functions.

## Features
Currently we support the following API operations

- end of period data
- history intraday
- realtime quote/trades/forex/crypto 
- realtime delayed
- news sentiment
- economic events

all features are provided asynchronously, so you need
either the [tokio runtime](https://tokio.rs/) or leverage `std:future` etc.

## Usage
Install via `cargo add eodhd_rs`
or add to your `Cargo.toml`


```toml
eodhd_rs = "0.2.0"
```


For authentication purposes set the environment
variable `EODHD_TOKEN` to your token.
Furthermore the following dependencies will make your live easier
```toml
env_logger = "0.9"
chrono = "0.4.22"
tokio = { version = "1", features = ["full"] }
```

## Examples

### End of Period, HistoricIntraday

```rust
use eodhd_rs::datetime::{EODHDDate, EODHDInterval};
use eodhd_rs::end_of_period::{EODHDEndOfPeriodFilter, EODHDPeriod};
use eodhd_rs::historic_intraday::HistoricIntradayOptions;


#[tokio::main]
async fn main() {
    let options = HistoricIntradayOptions {
        from: None,
        to: None,
        interval: EODHDInterval::Minute
    };

    match eodhd_rs::historic_intraday::get_historic_intraday("AAPL", options).await {
        Ok(o) => {
            // o is Vec<EODHDHistoricIntraday>
            println!("{:#?}", o);
        },
        Err(e) => {
            // e is Box<dyn Error> (mostly serde, reqwest errors)
            println!("{:#?}", e);
        }
    }

    // EODHDDate (year, month, day) is utility type for easy construction of chrono's NaiveDates
    let from = chrono::naive::NaiveDate::from(EODHDDate(2022, 9, 12));

    let filter = EODHDEndOfPeriodFilter {
        from: Some(from),
        to: None, // defaults of eodhd are used instead
        period: Some(EODHDPeriod::Daily)
    };
    match eodhd_rs::end_of_period::get_end_of_period("AAPL", Some(filter)).await {
        Ok(o) => {
            println!("{:#?}", o);
        },
        Err(e) => {
            println!("{:#?}", e);
        }
    }
}
```

### Realtime 

```rust

use futures::{Stream, StreamExt};
use eodhd_rs::realtime::{socket::{
    EODHDSocketKind, 
    subscribe_rt, 
    unsubscribe_rt,
    create_socket_channel
}, us::EODHDUSQuote, forex::{EODHDForexRT, EODHDCryptoRT}};

#[tokio::main]
async fn main() {
    // depending on the log level
    // you can get insight what is happening
    // behind the scenes (all messages received
    // instead of only the ticks)
    env_logger::init();

    let mut channel = create_socket_channel::<EODHDCryptoRT>(
        2, // size of the buffer for received ticks
        EODHDSocketKind::Crypto // must fit the generic parameter
    ).await.expect("Failed to create channel");
    subscribe_rt("BTC-USD", &mut channel).await.expect("Failed to subscribe to ticker");
    let mut counter = 0;
    while let Some(tick) = channel.tick_channel.recv().await {
        println!("Got a forex tick {:#?}", tick);
        if counter > 10 {
            break;
        }
        counter += 1;
    }
    unsubscribe_rt("EURUSD", &mut channel);

    let mut channel = create_socket_channel::<EODHDUSQuote>(2, EODHDSocketKind::Quote).await.expect("Failed to create channel");
    subscribe_rt("AAPL", &mut channel).await.expect("Failed to subscribe to ticker");
    let mut counter = 0;
    while let Some(tick) = channel.tick_channel.recv().await {
        println!("Got a quote {:#?}", tick);
        if counter > 10 {
            break;
        }
        counter += 1;
    }

    unsubscribe_rt("AAPL", &mut channel);

}
```

**Author**: Niklas Jona Lohmann
