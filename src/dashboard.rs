/*
Include a simple performance analytics dashboard showing bot's trading performance
*/

use chrono::{DateTime, Utc};

pub struct Dashboard {
    base_currency: String,
    num_trades: usize,
    total_profit: f64,
    total_buy_volume: f64,
    total_sell_volume: f64,
    total_buy_amount: f64,
    total_sell_amount: f64,
    last_trade: Option<Trade>,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub trade_type: TradeType,
    pub price: f64,
    pub volume: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum TradeType {
    Buy,
    Sell,
}

impl Dashboard {
    pub fn new(base_currency: &str) -> Self {
        Self {
            base_currency: base_currency.to_string(),
            num_trades: 0,
            total_profit: 0.0,
            total_buy_volume: 0.0,  
            total_sell_volume: 0.0,
            total_buy_amount: 0.0,
            total_sell_amount: 0.0,
            last_trade: None,
        }
    }

    pub fn record_trade(&mut self, trade: Trade) {
        self.num_trades += 1;
        self.last_trade = Some(trade.clone());

        match trade.trade_type {
            TradeType::Buy => {
                self.total_buy_volume += trade.volume;
                self.total_buy_amount += trade.price * trade.volume;
            }
            TradeType::Sell => {
                self.total_sell_volume += trade.volume;
                self.total_sell_amount += trade.price * trade.volume;
                self.total_profit += trade.volume * (trade.price - self.average_buy_price());
            }
        }
    }

    fn average_buy_price(&self) -> f64 {
        if self.total_buy_volume > 0.0 {
            self.total_buy_amount / self.total_buy_volume
        } else {
            0.0
        }
    }

    fn average_sell_price(&self) -> f64 {
        if self.total_sell_volume > 0.0 {
            self.total_sell_amount / self.total_sell_volume
        } else {
            0.0
        }
    }

    pub fn display_trade_performance(&self) {
        if let Some(trade) = &self.last_trade {
            println!("Last trade: {:?}", trade);
        }
    }

    pub fn display_general_performance(&self) {
        println!("\n[{}] Performance Summary:", self.base_currency);
        println!("Number of trades: {}", self.num_trades);
        println!("Total P&L: {:.6}", self.total_profit);
        println!("Total buy volume: {:.6}", self.total_buy_volume);
        println!("Total sell volume: {:.6}", self.total_sell_volume);
        println!("Average buy price: {:.6}", self.average_buy_price());
        println!("Average sell price: {:.6}", self.average_sell_price());
    }
}
