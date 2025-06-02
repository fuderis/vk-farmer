extern crate vk_farmer_bot;  use vk_farmer_bot::{ prelude::*, Farmer, Config, Profile, Settings };

use futures::future;

#[tokio::main]
async fn main() -> Result<()> {
    // init config:
    let config = Config::read_or_save("config.json")?;
    
    // create futures:
    let mut futures = vec![];
    for profile in config.profiles {
        futures.push( farm(config.settings.clone(), profile) );
    }

    // run futures:
    future::join_all(futures).await;

    Ok(())
}

// Start farming
async fn farm(settings: Settings, config: Profile) {
    if let Err(e) = farm_handler(settings, config).await {
        eprintln!("{e}");
    }
}
async fn farm_handler(settings: Settings, config: Profile) -> Result<()> {
    let mut farmer = Farmer::login(settings, config).await?;

    farmer.farm().await?;

    Ok(())
}
