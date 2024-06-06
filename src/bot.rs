//! This module defines the `TradingBot` struct and its associated methods for executing market making strategies.
//!
//! The `TradingBot` struct is responsible for:
//! - Fetching market data using the `CubeApi`.
//! - Calculating bid and ask prices based on the mid price, spread, and maker fee.
//! - Simulating trade execution due to current API issues.
//! - Recording and displaying trade performance metrics via the `Dashboard`.
//!
//! The bot operates asynchronously and is designed to handle multiple trading pairs concurrently.

use crate::api::CubeApi;
use crate::constants;
use crate::dashboard::{Dashboard, Trade, TradeType};
use log::warn;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

pub struct TradingBot {
    api: CubeApi,
    base_currency: String,
    profit_margin: f64,
    amount: f64,
    dashboard: Arc<Mutex<Dashboard>>,
}

impl TradingBot {
    pub fn new(base_currency: &str, amount: f64, dashboard: Arc<Mutex<Dashboard>>) -> Self {
        Self {
            api: CubeApi::new(),
            base_currency: base_currency.to_string(),
            profit_margin: constants::PROFIT_MARGIN,
            amount,
            dashboard,
        }
    }

    pub async fn run(&mut self) {
        let mut trade_interval = time::interval(Duration::from_secs(10));
        loop {
            tokio::select! {
                _ = trade_interval.tick() => {
                    match self
                        .api
                        .get_bid_ask_prices_by_base_currency(&self.base_currency)
                        .await
                    {
                        Ok(Some(ticker)) => {
                            let mid_price = (ticker.bid + ticker.ask) / 2.0;

                            // Adjust the spread to cover the maker fee and profit margin
                            let adjusted_spread_percentange = self.profit_margin + constants::MAKER_FEE;

                            // Set buy and sell prices
                            let buy_price = mid_price * (1.0 - adjusted_spread_percentange);
                            let sell_price = mid_price * (1.0 + adjusted_spread_percentange);

                            // Simulate trade for demonstration purposes
                            // Delete this after implementing the actual trading logic
                            {
                                let mut dashboard = self.dashboard.lock().await;
                                dashboard.record_trade(Trade {
                                    trade_type: TradeType::Buy,
                                    price: buy_price,
                                    volume: self.amount,
                                    timestamp: chrono::Utc::now(),
                                });
                                dashboard.record_trade(Trade {
                                    trade_type: TradeType::Sell,
                                    price: sell_price,
                                    volume: self.amount,
                                    timestamp: chrono::Utc::now(),
                                });

                                // Display trade performance
                                println!("\n[{}]:", self.base_currency);
                                println!("Buy Price: {:.6}, Sell Price: {:.6}", buy_price, sell_price);
                                dashboard.display_trade_performance();
                            }
                        }

                        Ok(None) => {
                            warn!(
                                "No ticker data found for base currency: {}",
                                self.base_currency
                            );
                        }
                        Err(e) => {
                            warn!("Error fetching market data: {}", e);
                        }
                    }
                }
            }
        }
    }
}
