#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]  // DO NOT REMOVE!!
use app::{ prelude::*, Manager, Config, Profile };
use tauri::State;

use std::{ fs, time::{SystemTime, UNIX_EPOCH} };

/// Generates an unique ID
fn uniq_id() -> String {
    let millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let random: u16 = rand::random();
    format!("{}{:04x}", millis, random)
}

/// Get bot profiles
#[tauri::command]
async fn get_bots(config: State<'_, Arc<Mutex<Config>>>) -> StdResult<Vec<String>, String> {
    let template = fs::read_to_string("ui/components/profile.html").map_err(|e| e.to_string())?;
    let config = config.lock().await;
    
    // reading profiles:
    let profiles = config.profiles.iter()
        .map(|(id, profile)| {
            let mut block = template.clone();

            // replacing bot data:
            for (from, to) in [
                ("__ID__", &id[..]),

                ("__NAME__", &profile.name[..]),
                ("__VK_ID__", &profile.vk_id[..]),

                ("__FARM_LIKES__", if profile.farm_likes { "checked" }else{ "" }),
                ("__LIKES_LIMIT__", &profile.likes_limit.to_string()[..]),

                ("__FARM_FRIENDS__", if profile.farm_friends { "checked" }else{ "" }),
                ("__FRIENDS_LIMIT__", &profile.friends_limit.to_string()[..]),

                ("__FARM_SUBSCRIBES__", if profile.farm_subscribes { "checked" }else{ "" }),
                ("__SUBSCRIBES_LIMIT__", &profile.subscribes_limit.to_string()[..]),
            ] {
                block = block.replace(from, to);
            }

            block
        })
        .collect::<Vec<_>>();

    Ok(profiles)
}

/// Creates a new bot
#[tauri::command]
async fn create_bot(config: State<'_, Arc<Mutex<Config>>>) -> StdResult<String, String> {
    let mut block = fs::read_to_string("ui/components/profile.html").map_err(|e| e.to_string())?;
    let mut config = config.lock().await;

    let id: String = uniq_id();
    let profile = Profile {
        name: id.clone(),
        ..Profile::default()
    };

    // replacing bot data:
    for (from, to) in [
        ("__ID__", &id[..]),

        ("__NAME__", &profile.name[..]),
        ("__VK_ID__", &profile.vk_id[..]),

        ("__FARM_LIKES__", if profile.farm_likes { "checked" }else{ "" }),
        ("__LIKES_LIMIT__", &profile.likes_limit.to_string()[..]),

        ("__FARM_FRIENDS__", if profile.farm_friends { "checked" }else{ "" }),
        ("__FRIENDS_LIMIT__", &profile.friends_limit.to_string()[..]),

        ("__FARM_SUBSCRIBES__", if profile.farm_subscribes { "checked" }else{ "" }),
        ("__SUBSCRIBES_LIMIT__", &profile.subscribes_limit.to_string()[..]),
    ] {
        block = block.replace(from, to);
    }

    config.profiles.insert(id, profile);
    config.save().map_err(|e| e.to_string())?;
    
    Ok(block)
}

/// Removes a bot
#[tauri::command]
async fn delete_bot(id: String, config: State<'_, Arc<Mutex<Config>>>) -> StdResult<String, String> {
    let mut config = config.lock().await;
    let _profile = config.profiles.get(&id).ok_or(Error::InvalidBotNameID.to_string())?;

    config.profiles.remove(&id);
    config.save().map_err(|e| e.to_string())?;

    Ok(String::new())
}

/// Updates bot data
#[tauri::command]
async fn update_bot(id: String, data: Profile) -> StdResult<String, String> {
    // TODO: ..
    
    Ok(String::new())
}

/// Start bot handler
#[tauri::command]
async fn start_bot(id: String, manager: State<'_, Arc<Mutex<Manager>>>, config: State<'_, Arc<Mutex<Config>>>) -> StdResult<String, String> {
    let config = config.lock().await;
    let profile = config.profiles.get(&id).ok_or(Error::InvalidBotNameID.to_string())?;
    
    manager.lock().await.start_bot(profile.clone(), config.settings.clone()).await.unwrap();

    Ok(String::new())
}

/// Stop bot handler
#[tauri::command]
async fn stop_bot(id: String, manager: State<'_, Arc<Mutex<Manager>>>) -> StdResult<String, String> {
    manager.lock().await.stop_bot(&id).await.unwrap();

    Ok(String::new())
}


#[tokio::main]
async fn main() -> Result<()> {
    // read/write config:
    let config = Arc::new(Mutex::new(Config::read_or_write("config.json")?));
    // init bots manager:
    let manager = Manager::new();

    // run ui:
    tauri::Builder::default()
        .manage(config)
        .manage(manager)
        .invoke_handler(tauri::generate_handler![
            get_bots,

            create_bot,
            delete_bot,

            start_bot,
            stop_bot
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}
