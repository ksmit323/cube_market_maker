mod api;
mod bot;
mod constants;
mod dashboard;
mod input;

use bot::TradingBot;
use env_logger;
use input::handle_user_input;
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::init();

    // Create ETH trading bot
    let eth_dashboard = Arc::new(Mutex::new(dashboard::Dashboard::new(constants::ETH)));
    let mut eth_trading_bot = TradingBot::new(
        constants::ETH,
        constants::ETH_ORDER_SIZE,
        Arc::clone(&eth_dashboard),
    );

    // Create SOL trading bot
    let sol_dashboard = Arc::new(Mutex::new(dashboard::Dashboard::new(constants::SOL)));
    let mut sol_trading_bot = TradingBot::new(
        constants::SOL,
        constants::SOL_ORDER_SIZE,
        Arc::clone(&sol_dashboard),
    );

    // Run trading bots asynchronously
    info!("Starting trading bots...");
    tokio::spawn(async move {
        eth_trading_bot.run().await;
    });
    tokio::spawn(async move {
        sol_trading_bot.run().await;
    });

    // Spawn a task to handle user input
    tokio::spawn(handle_user_input(eth_dashboard, sol_dashboard));

    // Keep the main function running
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
