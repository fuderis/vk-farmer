#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]  // DO NOT REMOVE!!
use app::{ prelude::*, Manager, Config, Profile };
use tauri::State;

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
            let log = fmt!("[{}] {}", record.level(), record.args());

            // printing to terminal:
            println!("{log}");

            // preparing log message:
            let mut msg = log;
            for (k, v) in [
                ("<", "&lt;"),
                (">", "&gt;"),
            ] {
                msg = msg.replace(k, v);
            }

            let log = app::templates::gen_log_line(vec![
                ("__LOG__".to_owned(), msg),
            ]);
            
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
    let config = config.lock().await;
    
    // reading profiles:
    let profiles = config.profiles.iter()
        .map(|(id, profile)| {
            let block = app::templates::gen_bot_profile(vec![
                ("__ID__".to_owned(), id.clone()),

                ("__NAME__".to_owned(), profile.name.clone()),
                ("__VK_ID__".to_owned(), profile.vk_id.clone()),

                ("__FARM_LIKES__".to_owned(), if profile.farm_likes { "checked".to_owned() }else{ "".to_owned() }),
                ("__LIKES_LIMIT__".to_owned(), profile.likes_limit.to_string()),

                ("__FARM_FRIENDS__".to_owned(), if profile.farm_friends { "checked".to_owned() }else{ "".to_owned() }),
                ("__FRIENDS_LIMIT__".to_owned(), profile.friends_limit.to_string()),

                ("__FARM_SUBSCRIBES__".to_owned(), if profile.farm_subscribes { "checked".to_owned() }else{ "".to_owned() }),
                ("__SUBSCRIBES_LIMIT__".to_owned(), profile.subscribes_limit.to_string()),

                ("__LIMITS__".to_owned(), (profile.likes_limit + profile.friends_limit + profile.subscribes_limit).to_string()),
            ]);

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
    let mut config = config.lock().await;

    let id: String = app::uniq_id();
    let profile = Profile {
        name: id.clone(),
        vk_id: "".into(),

        farm_likes: config.settings.defaults.farm_likes,
        likes_limit: config.settings.defaults.likes_limit,

        farm_friends: config.settings.defaults.farm_friends,
        friends_limit: config.settings.defaults.friends_limit,

        farm_subscribes: config.settings.defaults.farm_subscribes,
        subscribes_limit: config.settings.defaults.subscribes_limit,
    };

    let block = app::templates::gen_bot_profile(vec![
        ("__ID__".to_owned(), id.clone()),

        ("__NAME__".to_owned(), profile.name.clone()),
        ("__VK_ID__".to_owned(), profile.vk_id.clone()),

        ("__FARM_LIKES__".to_owned(), if profile.farm_likes { "checked".to_owned() }else{ "".to_owned() }),
        ("__LIKES_LIMIT__".to_owned(), profile.likes_limit.to_string()),

        ("__FARM_FRIENDS__".to_owned(), if profile.farm_friends { "checked".to_owned() }else{ "".to_owned() }),
        ("__FRIENDS_LIMIT__".to_owned(), profile.friends_limit.to_string()),

        ("__FARM_SUBSCRIBES__".to_owned(), if profile.farm_subscribes { "checked".to_owned() }else{ "".to_owned() }),
        ("__SUBSCRIBES_LIMIT__".to_owned(), profile.subscribes_limit.to_string()),

        ("__LIMITS__".to_owned(), (profile.likes_limit + profile.friends_limit + profile.subscribes_limit).to_string()),
    ]);

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
