use std::error::Error;

use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    Ok(())
}
