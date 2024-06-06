# Cube Market Maker

Welcome to the Cube Market Maker project! This project was developed as part of the Vietnam Rust Hackathon. It focuses on creating a market maker bot for the Cube Exchange, utilizing the Rust programming language. The goal of the bot is to continuously places buy and sell orders to provide liquidity and capture spreads on the Cube Exchange.
<p align="center">
  <img src="assets/crypto_cat.png" alt="Crypto Cat" width="400"/>
</p>

## Features

- **Asynchronous Trading:** Utilizes Rust's async capabilities to perform concurrent trading operations.
- **Configurable Spread and Order Sizes:** Allows users to define the spread and order sizes for market making.
- **Real-Time Market Data:** Fetches real-time market data from Cube Exchange.
- **Interactive Console:** Provides an interactive console to view trading performance and account details.
- **Error Handling:** Robust error handling and logging using `tokio` and `env_logger`.

## What's a Market Maker bot?
Imagine walking into a bustling marketplace, full of people shouting prices for their goods. Somewhere in the middle, there's this super sharp, ultra-fast trader who’s always ready to buy and sell. This trader is a market maker bot! But instead of haggling over apples and oranges, it’s trading cryptocurrencies on an exchange.

So, what does this slick operator do? Here’s the deal:

- **Liquidity Provider:** A market maker bot ensures there's always someone to buy or sell an asset. It places buy orders (bids) and sell orders (asks) continuously.
  
- **Spread Capturer:** It profits by capturing the "spread" – the tiny difference between the buy price and the sell price. Like a savvy shopkeeper who buys low and sells high, it earns from the price difference.

- **Non-stop Action:** This bot doesn’t sleep. It’s caffeinated and ready 24/7, making trades faster than you can say “blockchain”.

In essence, a market maker bot is the cool, unflappable trader that keeps the market moving, ensuring that trades happen smoothly, prices stay fair, and everyone gets their fill of the trading action.

## Getting Started
To start working with the project, clone this repository and build the project.

```sh
# Clone the repository
git clone https://github.com/ksmit323/cube_market_maker.git

# Change to the project directory
cd cube_market_maker

# Build the project
cargo build --release
```

## Usage

1. **Set up Environment Variables**
  - Create a .env file in the root directory and add your Cube Exchange API key and secret:
    ```env
    CUBE_API_KEY = "YOUR_KEY_HERE"
    CUBE_API_SECRET = "YOU_API_SECRET_HERE"
    ```
2. **Run the Bot**
    ```sh
    cargo run --releaese
    ```
3. **Interactive Console**
  - Type in the base currency symbol into the console to print the dashboard and display account details.
  - For example, type 'eth' to display the ETH dashboard

4. **Adding More Bots:**
The bot is designed to be dynamic, allowing you to add more trading bots for different cryptocurrencies easily. You are not limited to SOL or ETH; those are just examples. To add more bots:
- Modify the main() function to include more bots
- Example:
  ```rust
     // Create a BTC trading bot
    let btc_dashboard = Arc::new(Mutex::new(dashboard::Dashboard::new("BTC")));
    let mut BTC_trading_bot = TradingBot::new(
        "BTC",
        order_size_here,
        Arc::clone(&btc_dashboard),
      );
    // Run concurrently
    tokio::spawn(async move {
      sol_trading_bot.run().await;
    });
  ```


## Configuration

The bot can be configured using the "constants.rs" file. You can set parameters like:
- **Order Size**: Define the size of each order.
- **Profit Margin**: Set the desired profit margin over the maker fee.
- **Currencies**: Specify the currencies to be traded.

## Project Structure
```bash
.
├── assets             # Contains the images for the README.md        
├── src                # Source code for the project
│   ├── api.rs         # Connect to Cube API
│   ├── bot.rs         # Logic for trading bot
│   ├── constants.rs   # Configure constants
│   └── dashboard.rs   # Displays trading and account details
│   └── input.rs       # Interacts with console
│   └── main.rs        # Main entry point into project
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md         
```

## Improvements

Here are some potential improvements that could be made to the system:

- **Enhanced Error Handling:** Implement more granular error handling to cover specific cases, such as network timeouts or API rate limits, and provide more informative error messages.

- **Backtesting Framework:** Develop a backtesting framework to test the trading bot's strategies against historical data, allowing for better strategy optimization and validation before deploying in a live environment.
  
- **Advanced Order Types:** Introduce support for advanced order types such as stop-loss, take-profit, and trailing stop orders to enhance the bot's trading capabilities and risk management.

- **Dynamic Spread Adjustment:** Implement a mechanism to dynamically adjust the spread based on market conditions, such as volatility or order book depth, to optimize the bot's performance.
  
- **Performance Metrics Dashboard:** Develop a more comprehensive performance metrics dashboard that includes visualizations (e.g., charts and graphs) to provide better insights into the bot's trading performance over time.
  
- **Machine Learning Integration:** Integrate machine learning models to predict market trends and inform trading decisions, potentially improving the profitability of the bot.
  
- **High Availability and Scalability:** Refactor the bot to support high availability and scalability, such as deploying multiple instances across different servers and implementing failover mechanisms.

- **Automated Deployment:** Create scripts or use tools like Docker to automate the deployment of the trading bot, ensuring consistent and repeatable setups across different environments.

- **User Authentication and Authorization:** Implement a user authentication and authorization system to secure access to the bot and its configuration, especially if it is deployed as a service.

- **Real-Time Notifications:** Integrate real-time notification systems (e.g., email, SMS, or push notifications) to alert users of significant events, such as large trades, errors, or achieving profit targets.

If you have any suggestions or ideas for improvements, feel free to open an issue or submit a pull request. Contributions from the community are highly welcomed and appreciated.

## Acknowledgements

I would like to thank the organizers, ZKP, for all of their work and the **Cube Exchange** for providing this opportunity to explore the fascinating world of crypto markets and an easy to use API for easy integration.
