# eodhd-rs 💹

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
- realtime stocks and forex
- realtime delayed
- sentiment
- economic events

all features are provided asynchronously, so you need
either the [tokio runtime](https://tokio.rs/) or leverage `std:future` etc.

## Usage
Install via


```cargo add eodhd-rs```


or add to your `Cargo.toml`


```"eodhd-rs" = "0.1.0"```


For authentication purposes set the environment
variable `EODHD_TOKEN` to your token.

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
use eodhd_rs::realtime::socket::{EODHDSocketKind, subscribe_rt, unsubscribe_rt};
use eodhd_rs::realtime::us::{get_n_us_quote, get_n_us_trade};

#[tokio::main]
async fn main() {

    // Warning!!!
    // the realtime API is still "panicky"
    let mut channel = subscribe_rt("AAPL", EODHDSocketKind::Quote).await;
    // takes n quotes from the channel
    let quotes = get_n_us_quote(20, &mut channel).await;
    println!("{:#?}", quotes);
    // yes you should clean up this yourself
    unsubscribe_rt("AAPL", channel);

    /**
     * channel is a WebSocketStream<MaybeTlsStream<TcpStream>>
     * from tokio_tungstenite, we recommend to use the traits of
     * that package, if you want more fine grained control over reading from the channel.
    */

    let mut channel = subscribe_rt("AAPL", EODHDSocketKind::Trade).await;
    let trades = get_n_us_trade(10, &mut channel).await;
    println!("{:#?}", trades);
    // yes you should clean up this yourself
    unsubscribe_rt("AAPL", channel);
}
```

**Author**: Niklas Jona Lohmann
