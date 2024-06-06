use crate::constants;
use dotenv::dotenv;
use reqwest::{self, Client, Error};
use serde::{Deserialize, Serialize};
use std::env;
use base64;
use hex;
use sha2::Sha256;
use hmac::{Hmac, Mac};
use std::time::{SystemTime, UNIX_EPOCH};

// Alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

struct EnvVars {
    api_key: String,
    api_secret: String,
    subaccount_id: u64,
}

pub struct CubeApi {
    client: Client,
    api_key: String,
    api_secret: String,
    subaccount_id: u64,
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

// TODO: Refactor the redundant Ticker structs
#[derive(Serialize, Deserialize, Debug)]
pub struct Ticker {
    pub base_currency: String,
    pub bid: f64,
    pub ask: f64,
    pub volume: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    #[serde(rename = "clientOrderId")]
    client_order_id: u64,
    #[serde(rename = "requestId")]
    request_id: u64,
    #[serde(rename = "marketId")]
    market_id: u64,
    price: Option<u64>,
    quantity: u64,
    side: i32,
    #[serde(rename = "timeInForce")]
    time_in_force: i32,
    #[serde(rename = "orderType")]
    order_type: i32,
    #[serde(rename = "subaccountId")]
    subaccount_id: u64,
    #[serde(rename = "selfTradePrevention")]
    self_trade_prevention: Option<i32>,
    #[serde(rename = "postOnly")]
    post_only: i32,
    #[serde(rename = "cancelOnDisconnect")]
    cancel_on_disconnect: bool,
}

impl EnvVars {
    fn new() -> Self {
        dotenv().ok();
        Self {
            api_key: env::var("CUBE_API_KEY").expect("Please set your API key in .env file"),
            api_secret: env::var("CUBE_API_SECRET")
                .expect("Please set your API secret in .env file"),
            subaccount_id: env::var("CUBE_SUBACCOUNT_ID")
                .expect("Please set your subaccount id in .env file")
                .parse::<u64>()
                .expect("Subaccount id must be a valid u64"),
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
            subaccount_id: env_vars.subaccount_id,
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

    #[allow(unused)]
    async fn place_order(
        &self,
        client: &reqwest::Client,
        client_order_id: u64,
        request_id: u64,
        market_id: u64,
        price: Option<u64>,
        quantity: u64,
        side: i32,
        order_type: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {

        // Order parameters
        let order = Order {
            client_order_id,
            request_id,
            market_id,
            price,
            quantity,
            side,
            time_in_force: 1, // Example timeInForce value
            order_type,
            subaccount_id: self.subaccount_id,
            self_trade_prevention: Some(1), // Example selfTradePrevention value
            post_only: 1,                   // Example postOnly value
            cancel_on_disconnect: true,
        };

        let timestamp = get_timestamp();
        let api_signature = generate_api_signature(&self.api_secret, timestamp);

        let json_body= serde_json::to_string(&order)?;
    
        let response = client
            .post(constants::URL_MAINNET.to_string() + "os/v0/order") 
            .header("x-api-key", &self.api_key)
            .header("x-api-signature", api_signature)
            .header("x-api-timestamp", timestamp.to_string())
            .header("Content-Type", "application/json")
            .body(json_body)
            .send()
            .await?;

        let text = response.text().await?;
        Ok(text)
    }

    #[allow(unused)]
    pub async fn get_staging_market_data(&self) -> Result<String, Error> {
        let url = constants::URL_STAGING.to_string() + "md/v0/parsed/tickers";
        let response = reqwest::get(url).await?;
        let text = response.text().await?;
        Ok(text)
    }

    #[allow(unused)]
    pub fn print_api_response_text(
        &self,
        response_text: Result<String, Error>,
    ) {
        if let Ok(response_text) = response_text {
            println!("Response Data: {}", response_text);
        } else {
            println!("Error fetching API  data");
        }
    }
}

fn get_timestamp() -> u64 {
    // Get the current Unix epoch timestamp in seconds
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp = since_the_epoch.as_secs() as u64;
    timestamp
}

fn generate_api_signature(api_secret: &str, timestamp: u64) -> String {
    // Convert the timestamp to an 8-byte little-endian array
    let timestamp_bytes = timestamp.to_le_bytes();

    // Create the payload
    let payload = b"cube.xyz".iter().chain(&timestamp_bytes).cloned().collect::<Vec<u8>>();

    // Decode the secret key from hex
    let secret_key = hex::decode(api_secret).expect("Invalid hex string");

    // Create HMAC-SHA256 instance and sign the payload
    let mut mac = HmacSha256::new_from_slice(&secret_key).expect("Invalid key length");
    mac.update(&payload);
    let result = mac.finalize();
    let signature = result.into_bytes();

    // Base64 encode the signature
    base64::encode(&signature)
}