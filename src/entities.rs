use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum MarketState {
    #[serde(alias = "REGULAR", alias = "CLOSED")]
    Regular,
    #[serde(alias = "PRE", alias = "PREPRE")]
    Pre,
    #[serde(alias = "POST", alias = "POSTPOST")]
    Post,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    pub symbol: String,
    pub market_state: MarketState,
    pub regular_market_price: Option<f64>,
    pub regular_market_change: Option<f64>,
    pub regular_market_change_percent: Option<f64>,
    pub pre_market_price: Option<f64>,
    pub pre_market_change: Option<f64>,
    pub pre_market_change_percent: Option<f64>,
    pub post_market_price: Option<f64>,
    pub post_market_change: Option<f64>,
    pub post_market_change_percent: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct QuoteResponse {
    pub result: Vec<Ticker>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub quote_response: QuoteResponse,
}

pub fn get_ticker_data(ticker: &Ticker) -> (&str, MarketState, f64, f64, f64) {
    let pre_market_change = ticker.pre_market_change.unwrap_or_default();
    let post_market_change = ticker.post_market_change.unwrap_or_default();

    return if ticker.market_state == MarketState::Pre && pre_market_change != 0.0 {
        (
            &ticker.symbol,
            MarketState::Pre,
            ticker.pre_market_price.unwrap_or_default(),
            pre_market_change,
            ticker.pre_market_change_percent.unwrap_or_default(),
        )
    } else if ticker.market_state != MarketState::Regular && post_market_change != 0.0 {
        (
            &ticker.symbol,
            MarketState::Post,
            ticker.post_market_price.unwrap_or_default(),
            post_market_change,
            ticker.post_market_change_percent.unwrap_or_default(),
        )
    } else {
        (
            &ticker.symbol,
            MarketState::Regular,
            ticker.regular_market_price.unwrap_or_default(),
            ticker.regular_market_change.unwrap_or_default(),
            ticker.regular_market_change_percent.unwrap_or_default(),
        )
    };
}
