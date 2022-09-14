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
