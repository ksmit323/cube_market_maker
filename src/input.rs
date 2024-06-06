use crate::dashboard;
use std::sync::Arc;
use tokio::io::{self, AsyncBufReadExt};
use tokio::sync::Mutex;

pub async fn handle_user_input(
    eth_dashboard: Arc<Mutex<dashboard::Dashboard>>,
    sol_dashboard: Arc<Mutex<dashboard::Dashboard>>,
) {
    // Create reader for input
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin).lines();

    // Continuously read user input
    while let Ok(Some(line)) = reader.next_line().await {
        match line.trim() {
            // If user types "eth", display the ETH dashboard
            "eth" => {
                let eth_dashboard = eth_dashboard.lock().await;
                eth_dashboard.display_general_performance();
            }
            // If user types "sol", display the SOL dashboard
            "sol" => {
                let sol_dashboard = sol_dashboard.lock().await;
                sol_dashboard.display_general_performance();
            }
            // For any other input, display an unknown command message
            _ => {
                println!("Unknown command. Use 'eth' to show ETH dashboard, 'sol' to show SOL dashboard.");
            }
        }
    }
}
