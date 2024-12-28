// Import Duration from the standard time module
use std::time::Duration;

// Import the Handler struct and Storage from clara module
use clara::{handler::Handler, storage::Storage};
// Import sleep function from tokio's time module
use tokio::time::sleep;

// File path for persistent storage
const STORAGE_FILE: &'static str = "storage.json";

// Main async function using tokio runtime
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    // Initialize the environment logger
    env_logger::init();

    // Load processed tweets from storage file
    let storage = Storage::load_from_file(STORAGE_FILE)?;

    // Create a new instance of Handler with storage
    let mut handler = Handler::new(storage).await?;

    // Infinite loop to continuously process tweets
    loop {
        // Print status message for each iteration
        println!("Starting a new iteration...");
        // Process tweets using the handler
        handler.process_tweets().await?;
        // Sleep for 2 minutes before next iteration
        sleep(Duration::from_secs(2 * 60)).await;
    }
}
