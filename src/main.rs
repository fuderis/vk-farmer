#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]  // DO NOT REMOVE!!
use app::{ prelude::*, Manager, Config, Profile };
use tauri::State;

use std::{ fs, time::{SystemTime, UNIX_EPOCH} };

static LOGGER: Logger = Logger { logs: StdMutex::new(vec![]) };

/// The programm logger
struct Logger {
    pub logs: StdMutex<Vec<String>>,
}

impl Logger {
    /// Takes all logger logs
    pub fn take(&self) -> Vec<String> {
        let mut logs_lock = self.logs.lock().unwrap();
        std::mem::take(&mut *logs_lock)
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut log = fmt!("[{}] {}", record.level(), record.args());

            // printing to terminal:
            println!("{log}");

            // preparing log to HTML:
            for (k, v) in [
                ("<", "&lt;"),
                (">", "&gt;"),
            ] {
                log = log.replace(k, v);
            }
            let log = fmt!(r#"<div class="line">{log}</div>"#);
            
            self.logs.lock().unwrap().push(log);
        }
    }

    fn flush(&self) {}
}

/// Updates logger lines
#[tauri::command]
async fn update_logger() -> StdResult<Vec<String>, String> {
    Ok(LOGGER.take())
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

                ("__LIMITS__", &(profile.likes_limit + profile.friends_limit + profile.subscribes_limit).to_string()[..]),
            ] {
                block = block.replace(from, to);
            }

            block
        })
        .collect::<Vec<_>>();

    Ok(profiles)
}

/// Updates bot limits percentage
#[tauri::command]
async fn update_bot_limits(id: String, manager: State<'_, Arc<Mutex<Manager>>>) -> StdResult<usize, String> {
    let percent = manager.lock().await.get_bot_limits_percentage(&id).await.map_err(|e| e.to_string())?;
    Ok(percent)
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

        ("__LIMITS__", &(profile.likes_limit + profile.friends_limit + profile.subscribes_limit).to_string()[..]),
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
async fn update_bot(id: String, data: String, config: State<'_, Arc<Mutex<Config>>>) -> StdResult<usize, String> {
    let mut config = config.lock().await;
    let profile = config.profiles.get_mut(&id).ok_or(Error::InvalidBotNameID.to_string())?;

    let data: Profile = serde_json::from_str(&data).map_err(|e| e.to_string())?;

    profile.name = data.name.to_string();
    profile.vk_id = data.vk_id.to_string();

    profile.farm_likes = data.farm_likes;
    profile.likes_limit = data.likes_limit;

    profile.farm_friends = data.farm_friends;
    profile.friends_limit = data.friends_limit;

    profile.farm_subscribes = data.farm_subscribes;
    profile.subscribes_limit = data.subscribes_limit;

    let limits = profile.likes_limit + profile.friends_limit + profile.subscribes_limit;

    config.save().map_err(|e| e.to_string())?;
    
    Ok(limits)
}

/// Start bot handler
#[tauri::command]
async fn start_bot(id: String, manager: State<'_, Arc<Mutex<Manager>>>, config: State<'_, Arc<Mutex<Config>>>) -> StdResult<String, String> {
    let config = config.lock().await;
    let profile = config.profiles.get(&id).ok_or(Error::InvalidBotNameID.to_string())?;
    
    manager.lock().await.start_bot(id, profile.clone(), config.settings.clone()).await.map_err(|e| e.to_string())?;

    Ok(String::new())
}

/// Stop bot handler
#[tauri::command]
async fn stop_bot(id: String, manager: State<'_, Arc<Mutex<Manager>>>) -> StdResult<String, String> {
    manager.lock().await.stop_bot(&id).await.map_err(|e| e.to_string())?;

    Ok(String::new())
}


#[tokio::main]
async fn main() -> Result<()> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(log::LevelFilter::Info);

    // read/write config:
    let config = Arc::new(Mutex::new(Config::read_or_write("config.json")?));
    // init bots manager:
    let manager = Manager::new();

    // run ui:
    tauri::Builder::default()
        .manage(config)
        .manage(manager)
        .invoke_handler(tauri::generate_handler![
            update_logger,
            
            get_bots,
            update_bot_limits,

            create_bot,
            delete_bot,
            update_bot,

            start_bot,
            stop_bot,
        ])
        .run(tauri::generate_context!())?;

    Ok(())
}


/// Generates an unique ID
fn uniq_id() -> String {
    let millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let random: u16 = rand::random();
    format!("{}{:04x}", millis, random)
}
