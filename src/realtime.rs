/// Setting up the socket and subscribing
/// to ticks.
pub mod socket;
/// Realtime data that is not sent
/// over a socket [eodhd
/// docs](https://eodhistoricaldata.com/financial-apis/live-realtime-stocks-api/)
pub mod delayed;
/// Also includes tick types for crypto
pub mod forex;
/// Tick types for quote & trades 
pub mod us;

const BASE_URL_SOCKET: &str = "wss://ws.eodhistoricaldata.com/ws";
