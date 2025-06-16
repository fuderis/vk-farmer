use crate::{ prelude::*, Farmer, Controller, Profile, Settings };

/// The bots manager
#[derive(Debug)]
pub struct Manager {
    port: usize,
    bots: HashMap<String, Arc<Mutex<Controller>>>,
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
    pub async fn start_bot(&mut self, profile: Profile, settings: Settings) -> Result<()> {
        // check name for unique:
        if self.bots.contains_key(&profile.name) {
            return Err(Error::BotNameIDIsBusy.into());
        }
        
        let name = profile.name.clone();
        let control = Controller::new();
        let port = self.port.to_string();
        self.port += 1;
        
        // init & start bot:
        let bot = Farmer::login(control.clone(), port, profile, settings).await?;
        tokio::spawn(bot.farm());

        let _bot_control = self.bots.insert(name.clone(), control);

        Ok(())
    }

    /// Stop bot session
    pub async fn stop_bot(&mut self, name: &str) -> Result<()> {
        if let Some(control) = self.bots.remove(name) {
            control.lock().await.close();
        } else {
            return Err(Error::InvalidBotNameID.into())
        }
        
        Ok(())
    }
}
