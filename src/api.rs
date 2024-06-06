use crate::constants;
use dotenv::dotenv;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

struct EnvVars {
    api_key: String,
    api_secret: String,
}

pub struct CubeApi {
    client: Client,
    api_key: String,
    api_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    result: Vec<TickerResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TickerResponse {
    pub base_currency: String,
    pub bid: Option<f64>,
    pub ask: Option<f64>,
    pub base_volume: Option<f64>,
}

// TODO: Refactor and eliminate the redundant structs
#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    pub base_currency: String,
    pub bid: f64,
    pub ask: f64,
    pub volume: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub symbol: String,
    pub price: f64,
    pub amount: f64,
    pub side: String,
}

impl EnvVars {
    fn new() -> Self {
        dotenv().ok();
        Self {
            api_key: env::var("CUBE_API_KEY").expect("Please set your API key in .env file"),
            api_secret: env::var("CUBE_API_SECRET")
                .expect("Please set your API secret in .env file"),
        }
    }
}

impl CubeApi {
    pub fn new() -> Self {
        let env_vars = EnvVars::new();
        Self {
            client: Client::new(),
            api_key: env_vars.api_key,
            api_secret: env_vars.api_secret,
        }
    }

    pub async fn get_market_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = constants::URL_MAINNET.to_string() + "md/v0/parsed/tickers";
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        Ok(text)
    }

    pub async fn extract_bid_ask_prices(&self) -> Result<Vec<Ticker>, Box<dyn std::error::Error>> {
        let market_data = self.get_market_data().await?;
        let api_response: ApiResponse = serde_json::from_str(&market_data)?;
        let ticker_data = api_response
            .result
            .into_iter()
            .filter_map(|ticker| {
                if let (Some(bid), Some(ask), Some(volume)) =
                    (ticker.bid, ticker.ask, ticker.base_volume)
                {
                    Some(Ticker {
                        base_currency: ticker.base_currency,
                        bid,
                        ask,
                        volume,
                    })
                } else {
                    None
                }
            })
            .collect();
        Ok(ticker_data)
    }

    pub async fn get_bid_ask_prices_by_base_currency(
        &self,
        base_currency: &str,
    ) -> Result<Option<Ticker>, Box<dyn std::error::Error>> {
        let tickers = self.extract_bid_ask_prices().await?;
        let ticker = tickers
            .into_iter()
            .find(|t| t.base_currency == base_currency);
        Ok(ticker)
    }

    // TODO: Fix ordering system
    #[allow(unused)]
    pub async fn place_order(
        &self,
        symbol: &str,
        price: f64,
        amount: f64,
        side: &str,
    ) -> Result<Order, reqwest::Error> {
        let url = "https://api.cube.exchange/os/v0/order";
        let order = serde_json::json!({
            "symbol": symbol,
            "price": price,
            "amount": amount,
            "side": side,
            "apiKey": self.api_key,
            "apiSecret": self.api_secret
        });

        let resp = self
            .client
            .post(url)
            .json(&order)
            .send()
            .await?
            .json::<Order>()
            .await?;
        Ok(resp)
    }

    #[allow(unused)]
    pub async fn get_staging_market_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://staging.cube.exchange/md/v0/parsed/tickers";
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        Ok(text)
    }

    #[allow(unused)]
    pub fn print_api_response_text(
        &self,
        response_text: Result<String, Box<dyn std::error::Error>>,
    ) {
        if let Ok(response_text) = response_text {
            println!("Market Data: {}", response_text);
        } else {
            println!("Error fetching staging market data");
        }
    }
}
