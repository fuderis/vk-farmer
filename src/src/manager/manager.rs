use crate::{ prelude::*, Farmer, Profile, Settings };
use super::Task;

/// The bots manager
#[derive(Debug)]
pub struct Manager {
    port: usize,
    bots: Arc<Mutex<HashMap<String, (Arc<Mutex<Farmer>>, Arc<Mutex<Task>>)>>>,
}

impl Manager {
    /// Creates a new instance of bots manager
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
            Self {
                port: 54477,
                bots: Arc::new(Mutex::new(HashMap::new()))
            }
        ))
    }

    /// Start bot session
    pub async fn start_bot<S: Into<String>>(&mut self, id: S, profile: Profile, settings: Settings) -> Result<()> {        
        // check name for unique:
        if self.bots.lock().await.contains_key(&profile.name) {
            return Err(Error::BotNameIDIsBusy.into());
        }

        // convert id:
        let id = id.into();
        
        // generate new port:
        let port = self.port.to_string();
        self.port += 1;
        
        // spawn task:
        tokio::spawn(Self::start_bot_handler(self.bots.clone(), port, id, profile, settings));
        
        Ok(())
    }

    /// Start bot session handler
    async fn start_bot_handler(bots: Arc<Mutex<HashMap<String, (Arc<Mutex<Farmer>>, Arc<Mutex<Task>>)>>>, port: String, id: String, profile: Profile, settings: Settings) {
        let task = Task::new(
            if profile.farm_likes { profile.likes_limit }else{ 0 }
            + if profile.farm_friends { profile.friends_limit }else{ 0 }
            + if profile.farm_subscribes { profile.subscribes_limit }else{ 0 }
        );
        
        // init & start bot:
        let bot = Farmer::login(task.clone(), port, profile, settings).await.unwrap();
        let _ = bots.lock().await.insert(id, (bot.clone(), task));

        tokio::spawn(async move {
            bot.lock().await.farm().await.unwrap();
        });
    }

    /// Stop bot session
    pub async fn stop_bot(&mut self, id: &str) -> Result<()> {
        if let Some((bot, task)) = self.bots.lock().await.remove(id) {
            task.lock().await.close();

            let mut bot_lock = bot.lock().await;
            bot_lock.close().await?;

            drop(bot_lock);
        } else {
            return Err(Error::InvalidBotNameID.into())
        }
        
        Ok(())
    }

    /// Get bot limits percentage
    pub async fn get_bot_limits_percentage(&self, id: &str) -> Result<usize> {
        if let Some((_, task)) = self.bots.lock().await.get(id) {
            Ok(task.lock().await.calc_limits_percentage())
        } else {
            Err(Error::InvalidBotNameID.into())
        }
    }
}
