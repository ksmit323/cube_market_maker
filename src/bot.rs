use crate::api::CubeApi;
use crate::constants;
use crate::dashboard::{Dashboard, Trade, TradeType};
use log::warn;
use std::sync::Arc;
use tokio::time::{self, Duration};

pub struct TradingBot {
    api: Arc<CubeApi>,
    base_currency: String,
    profit_margin: f64,
    amount: f64,
    pub dashboard: Dashboard,
}

impl TradingBot {
    pub fn new(api: Arc<CubeApi>, base_currency: &str, profit_margin: f64, amount: f64) -> Self {
        Self {
            api,
            base_currency: base_currency.to_string(),
            profit_margin,
            amount,
            dashboard: Dashboard::new(base_currency),
        }
    }

    pub async fn run(&mut self) {
        let mut trade_interval = time::interval(Duration::from_secs(10));
        let mut performance_interval = time::interval(Duration::from_secs(20));
        loop {
            tokio::select! {
                _ = trade_interval.tick() => {        
                    match self
                        .api
                        .get_bidask_prices_by_base_currency(&self.base_currency)
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
                            self.dashboard.record_trade(Trade {
                                trade_type: TradeType::Buy,
                                price: buy_price,
                                volume: self.amount,
                                timestamp: chrono::Utc::now(),
                            });
                            self.dashboard.record_trade(Trade {
                                trade_type: TradeType::Sell,
                                price: sell_price,
                                volume: self.amount,
                                timestamp: chrono::Utc::now(),
                            });
                            
                            // Display trade performance
                            println!("\n[{}]:", self.base_currency);
                            println!("Buy Price: {:.6}, Sell Price: {:.6}", buy_price, sell_price);
                            self.dashboard.display_trade_performance();
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
                _ = performance_interval.tick() => {
                    // Display the general performance every 60 seconds
                    self.dashboard.display_general_performance();
                }
            }
        }
    }
}
