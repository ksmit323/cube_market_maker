mod api;
mod bot;
mod constants;
mod dashboard;

use api::CubeApi;
use bot::TradingBot;
use env_logger;
use log::info;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Create shared API instance
    let api = Arc::new(CubeApi::new());

    // Create ETH trading bot
    let mut eth_trading_bot = TradingBot::new(
        Arc::clone(&api),
        constants::ETH,
        constants::PROFIT_MARGIN,
        constants::ETH_ORDER_SIZE,
    );

    // Create SOL trading bot
    let mut sol_trading_bot = TradingBot::new(
        Arc::clone(&api),
        constants::SOL,
        constants::PROFIT_MARGIN,
        constants::SOL_ORDER_SIZE,
    );

    // Run trading bots asynchronously
    info!("Starting trading bots...");
    tokio::spawn(async move {
        eth_trading_bot.run().await;
    });
    tokio::spawn(async move {
        sol_trading_bot.run().await;
    });

    // Keep the main function running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
