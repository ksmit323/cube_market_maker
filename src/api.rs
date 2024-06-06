/*
Endpoints, authentication required
Endpoints in this section require REST Authentication headers. Note that only API keys with access-level WRITE are able to access any of these endpoints.

REST Authentication Headers
The REST API uses the following HTTP headers for authentication:

x-api-key:
The API Key ID, as specified on the API settings page.

Each API key has an associated access level, which is determined at the time of key creation.

Read access allows only read HTTP methods (GET, HEAD, etc.).

Write access permits all HTTP methods.

x-api-signature:
The API signature string authenticating this request.

The payload to be signed is a concatenation of the byte string cube.xyz and the current Unix epoch timestamp in seconds, converted into an 8-byte little-endian array. The signature is the HMAC-SHA256 digest of the payload, using the secret key associated with the specified API key.

Implementation notes:

The signature is base-64 encoded with the 'standard' alphabet and padding: ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/.

The timestamp should be encoded as an 8-byte little-endian array of bytes.

The secret key should be decoded from a hex string into a 32-byte array of bytes.

x-api-timestamp:
The timestamp used for signature generation.

Test net API URL:
https://staging.cube.exchange/ir/v0/
*/

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[allow(unused)]
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

impl CubeApi {
    pub fn new(api_key: &str, api_secret: &str) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.to_string(),
            api_secret: api_secret.to_string(),
        }
    }

    pub async fn get_market_data(&self) -> Result<String, Box<dyn std::error::Error>> {
        let url = "https://api.cube.exchange/md/v0/parsed/tickers";
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

    pub async fn get_bidask_prices_by_base_currency(
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
        let url = "https://staging.cube.exchange/ir/v0/markets";
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
