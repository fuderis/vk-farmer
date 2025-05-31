extern crate vk_farmer_bot;  use vk_farmer_bot::{ prelude::*, Farmer, Config };

use macron::*;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = tokio::join!(
        farm("Synap", "54477"),
        farm("Bot1", "54478")
    );

    Ok(())
}

// Start farming
async fn farm(profile: &str, port: &str) {
    if let Err(e) = farm_handler(profile, port).await {
        eprintln!("{e}");
    }
}
async fn farm_handler(profile: &str, port: &str) -> Result<()> {
    let mut farmer = Farmer::login(Config {
        profile: str!(profile),
        port: str!(port),
        ..Default::default()
    }).await?;

    farmer.farm().await?;

    Ok(())
}
