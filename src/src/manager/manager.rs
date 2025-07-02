use crate::{ prelude::*, Farmer, Profile, Settings };
use super::Task;

use std::net::TcpListener;

/// The bots manager
#[derive(Debug)]
pub struct Manager {
    bots: Arc<Mutex<HashMap<String, (Arc<Mutex<Option<Farmer>>>, Arc<Mutex<Task>>)>>>,
}

impl Manager {
    /// Creates a new instance of bots manager
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
            Self {
                bots: Arc::new(Mutex::new(HashMap::new()))
            }
        ))
    }

    /// Start bot session
    pub async fn start_bot<S: Into<String>>(&mut self, bot_id: S, profile: Profile, settings: Settings) -> Result<()> {        
        // check name for unique:
        if self.bots.lock().await.contains_key(&profile.name) {
            return Err(Error::BotNameIDIsBusy.into());
        }

        // convert id:
        let bot_id = bot_id.into();
        
        // get free port:
        let port = TcpListener::bind("127.0.0.1:0")?.local_addr()?.port().to_string();
        
        // create a task controller:
        let task = Task::new(
            &bot_id,
            if profile.farm_likes { profile.likes_limit }else{ 0 }
            + if profile.farm_friends { profile.friends_limit }else{ 0 }
            + if profile.farm_subscribes { profile.subscribes_limit }else{ 0 }
        );

        let bot = Arc::new(Mutex::new(None));
        self.bots.lock().await.insert(bot_id.clone(), (bot.clone(), task.clone()));
        
        // spawn handler:
        tokio::spawn(Self::start_bot_handler(bot_id, bot, task, port, profile, settings));
        
        Ok(())
    }

    /// Start bot session handler
    async fn start_bot_handler(bot_id: String, bot: Arc<Mutex<Option<Farmer>>>, task: Arc<Mutex<Task>>, port: String, profile: Profile, settings: Settings) {
        // starting farm:
        tokio::spawn(async move {
            // init & start bot:
            match Farmer::login(bot_id, task.clone(), port, profile, settings).await {
                Ok(farmer) => {
                    let _ = bot.lock().await.insert(farmer);
                    
                    if let Some(mut bot) = bot.lock().await.take() {
                        bot.farm().await.unwrap()
                    }
                }

                Err(_e) => {}
            }
        });
    }

    /// Stop bot session
    pub async fn stop_bot(&mut self, id: &str) -> Result<()> {
        if let Some((bot, task)) = self.bots.lock().await.remove(id) {
            task.lock().await.close();
            
            if let Some(mut bot) = bot.lock().await.take() {
                bot.close().await?;
            }
        } else {
            return Err(Error::InvalidBotNameID.into())
        }
        
        Ok(())
    }

    /// Stop all bot sessions
    pub async fn stop_all_bots(&mut self) -> Result<()> {
        warn!("Closing bot sessions before closing program, please wait..");
        
        for (_, (_, task)) in self.bots.lock().await.iter() {
            task.lock().await.close();
        }

        Ok(())
    }

    /// Checks all bot sessions for closed
    pub async fn all_bots_is_stoped(&self) -> bool {
        for (_, (_, task)) in self.bots.lock().await.iter() {
            if !task.lock().await.is_closed() {
                return false;
            }
        }

        true
    }
}
