#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]  // DO NOT REMOVE!!
use app::{ prelude::*, Manager, Profile };
use tauri::Manager as TauriManager;

/// Get bot profiles
#[tauri::command]
async fn get_bots() -> StdResult<Vec<String>, String> {
    let config = CONFIG.lock().await;
    
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

/// Creates a new bot
#[tauri::command]
async fn create_bot() -> StdResult<String, String> {
    let mut config = CONFIG.lock().await;

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
async fn delete_bot(id: String) -> StdResult<String, String> {
    let mut config = CONFIG.lock().await;
    let _profile = config.profiles.get(&id).ok_or(Error::InvalidBotNameID.to_string())?;

    config.profiles.remove(&id);
    config.save().map_err(|e| e.to_string())?;

    Ok(String::new())
}

/// Updates bot data
#[tauri::command]
async fn update_bot(id: String, data: String) -> StdResult<usize, String> {
    let mut config = CONFIG.lock().await;
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
async fn start_bot(id: String) -> StdResult<String, String> {
    let config = CONFIG.lock().await;
    let profile = config.profiles.get(&id).ok_or(Error::InvalidBotNameID.to_string())?;
    
    BOTS_MANAGER.lock().await.start_bot(id, profile.clone(), config.settings.clone()).await.map_err(|e| e.to_string())?;

    Ok(String::new())
}

/// Stop bot handler
#[tauri::command]
async fn stop_bot(id: String) -> StdResult<String, String> {
    BOTS_MANAGER.lock().await.stop_bot(&id).await.map_err(|e| e.to_string())?;

    Ok(String::new())
}


pub static BOTS_MANAGER: Lazy<Arc<Mutex<Manager>>> = Lazy::new(|| Manager::new());

#[tokio::main]
async fn main() -> Result<()> {
    // init logger:
    LOGGER.init()?;

    // allowing shortcuts:
    let prevent = tauri_plugin_prevent_default::Builder::new()
        .with_flags(tauri_plugin_prevent_default::Flags::empty())
        .build();

    // run ui:
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_bots,

            create_bot,
            delete_bot,
            update_bot,

            start_bot,
            stop_bot,
        ])
        .setup(|app| {
            let app_handle = app.app_handle().clone();
            let window = app_handle.get_webview_window("main").unwrap();

            // init app handler:
            *APP_HANDLE.lock().unwrap() = Some(app_handle.clone());
            
            // init tray-icon:
            *SYSTEM_TRAY.lock().unwrap() = Some(Tray::new());
            
            // window events:
            window.on_window_event(move |event| {
                let window = app_handle.get_webview_window("main").unwrap();
                
                match event {
                    // if window closes:
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        
                        // closing bot sessions:
                        tokio::task::block_in_place(|| {
                            tauri::async_runtime::block_on(async {
                                let mut manager = BOTS_MANAGER.lock().await;
                                manager.stop_all_bots().await.unwrap();

                                loop {
                                    if manager.all_bots_is_stoped().await {
                                        break;
                                    }
                                }
                            });
                        });

                        // saving logs:
                        LOGGER.save().unwrap();

                        // removing tray:
                        if let Some(tray) = SYSTEM_TRAY.lock().unwrap().take() {
                            tray.remove();
                        }

                        // closing program:
                        app_handle.exit(0);
                    }

                    // if window minimized:
                    tauri::WindowEvent::Resized(_) => {
                        if window.is_minimized().unwrap_or(false) {
                            window.hide().unwrap();
                        }
                    }

                    _ => {}
                }
            });
            
            Ok(())
        })
        .plugin(prevent)
        .run(tauri::generate_context!())?;

    Ok(())
}
