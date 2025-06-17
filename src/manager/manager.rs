use crate::{ prelude::*, Farmer, Profile, Settings };
use super::Task;

/// The bots manager
#[derive(Debug)]
pub struct Manager {
    port: usize,
    bots: HashMap<String, (Arc<Mutex<Farmer>>, Arc<Mutex<Task>>)>,
}

impl Manager {
    /// Creates a new instance of bots manager
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
            Self {
                port: 54477,
                bots: HashMap::new()
            }
        ))
    }

    /// Start bot session
    pub async fn start_bot<S: Into<String>>(&mut self, id: S, profile: Profile, settings: Settings) -> Result<()> {
        // check name for unique:
        if self.bots.contains_key(&profile.name) {
            return Err(Error::BotNameIDIsBusy.into());
        }
        
        let port = self.port.to_string();
        self.port += 1;

        let task = Task::new(profile.likes_limit + profile.friends_limit + profile.subscribes_limit);
        
        // init & start bot:
        let bot = Farmer::login(task.clone(), port, profile, settings).await?;
        let _ = self.bots.insert(id.into(), (bot.clone(), task));

        tokio::spawn(async move {
            bot.lock().await.farm().await.unwrap();
        });

        Ok(())
    }

    /// Stop bot session
    pub async fn stop_bot(&mut self, id: &str) -> Result<()> {
        if let Some((bot, task)) = self.bots.remove(id) {
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
        if let Some((_, task)) = self.bots.get(id) {
            Ok(task.lock().await.calc_limits_percentage())
        } else {
            Err(Error::InvalidBotNameID.into())
        }
    }
}
