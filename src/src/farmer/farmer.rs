use crate::{ prelude::*, Profile, Settings, Task };
use super::{ VKontakte, FreeLikes, BigLike };

use chromedriver_api::Session;
use tokio::time::{ sleep, Duration };

/// Farmer bot
#[allow(dead_code)]
#[derive(Debug)]
pub struct Farmer {
    task: Arc<Mutex<Task>>,
    
    pub(crate) port: String,
    pub(crate) profile: Profile,
    pub(crate) settings: Settings,
    
    bot_id: String,
    session: Option<Session>,
    vkontakte: Arc<Mutex<VKontakte>>,
    freelikes: FreeLikes,
    biglike: BigLike
}

impl Farmer {
    /// Login to profile
    pub async fn login<S: Into<String>>(bot_id: S, task: Arc<Mutex<Task>>, port: S, profile: Profile, settings: Settings) -> Result<Self> {
        info!("({}) Starting bot session ..", &profile.name);

        let bot_id = bot_id.into();
        let path = fmt!("C:\\Users\\Synap\\AppData\\Local\\Google\\Chrome\\Profiles\\{}", &profile.name);
        let port = port.into();
        
        // run temp session (loging to social networks..):
        let mut session = Session::run(&port, Some("/bin/chromedriver/chromedriver.exe"), Some(&path), false).await?;
        let _ = VKontakte::login(&mut session, &profile, true).await?;
        session.close().await?;

        if task.lock().await.to_close() {
            return Err(Error::BotLoginCanceled.into())
        }

        // run session:
        let mut session = Session::run(&port, Some("/bin/chromedriver/chromedriver.exe"), Some(&path), true).await?;
        let vkontakte = Arc::new(Mutex::new(VKontakte::login(&mut session, &profile, false).await?));
        
        if task.lock().await.to_close() {
            session.close().await?;
            return Err(Error::BotLoginCanceled.into())
        }
        
        // init freelikes:
        let freelikes = FreeLikes::login(task.clone(), &profile.name, &mut session, vkontakte.clone()).await?;

        if task.lock().await.to_close() {
            session.close().await?;
            return Err(Error::BotLoginCanceled.into())
        }

        // init biglike:
        let biglike = BigLike::login(task.clone(), &profile.name, &mut session, vkontakte.clone()).await?;

        if task.lock().await.to_close() {
            session.close().await?;
            return Err(Error::BotLoginCanceled.into())
        }

        info!("({}) Session is ready.", &profile.name);

        Ok(Self {
                bot_id,
                task,

                port,
                profile,
                settings,

                session: Some(session),
                vkontakte,
                freelikes,
                biglike
            }
        )
    }

    /// Start farming
    pub async fn farm(&mut self) -> Result<()> {
        let mut likes_limit = self.profile.likes_limit;
        let mut friends_limit = self.profile.friends_limit;
        let mut subscribes_limit = self.profile.subscribes_limit;

        loop {
            match self.farm_handler(&mut likes_limit, &mut friends_limit, &mut subscribes_limit).await {
                Ok(status) => {
                    if !status {
                        warn!("({}) Farming canceled! Closing session..", self.profile.name);
                        self.close().await?;

                        return Ok(());
                    }
                    else if !self.task.lock().await.check_limits() {
                        warn!("({}) All tasks is completed! Closing session..", self.profile.name);
                        self.close().await?;

                        return Ok(());
                    }
                    else {
                        warn!("({}) Tasks is over, timeout for {} minutes..", self.profile.name, self.settings.pause_delay);

                        for _ in 0..(self.settings.pause_delay as u64 * 60) {
                            if self.task.lock().await.to_close() {
                                warn!("({}) Farming canceled! Closing session..", self.profile.name);
                                self.close().await?;

                                return Ok(());
                            }
                            
                            sleep(Duration::from_secs(1)).await;
                        }
                    }
                }

                Err(e) => {
                    err!("Farming panicked with error: {e}");
                    self.close().await?;

                    return Ok(());
                }
            }
        }
    }

    /// The farming handler
    async fn farm_handler(&mut self, mut likes_limit: &mut usize, mut friends_limit: &mut usize, mut subscribes_limit: &mut usize) -> Result<bool> {
        // <freelikes.online> farm likes:
        if self.task.lock().await.to_close() { return Ok(false); }
        if self.profile.farm_likes && *likes_limit > 0 {
            if let Err(e) = self.freelikes.vk_farm_likes(self.settings.freelikes.likes_min_price, &mut likes_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                warn!("({}) <freelikes.online> The tasks for 'likes' type are over, timeout for 10 seconds..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => return Err(e),
                        }
                    }

                    Err(e) => return Err(e)
                }
            }
        }

        // <freelikes.online> farm friends:
        if self.task.lock().await.to_close() { return Ok(false); }
        if self.profile.farm_friends && *friends_limit > 0 {
            if let Err(e) = self.freelikes.vk_farm_friends(self.settings.freelikes.friends_min_price, &mut friends_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                warn!("({}) <freelikes.online> The tasks for 'friends' type are over, timeout for 10 seconds..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => return Err(e),
                        }
                    }

                    Err(e) => return Err(e)
                }
            }
        }

        // <freelikes.online> farm subscribes:
        if self.task.lock().await.to_close() { return Ok(false); }
        if self.profile.farm_subscribes && *subscribes_limit > 0 {
            if let Err(e) = self.freelikes.vk_farm_subscribes(self.settings.freelikes.subscribes_min_price, &mut subscribes_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                warn!("({}) <freelikes.online> The tasks for 'subscribes' type are over, timeout for 10 seconds..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => return Err(e),
                        }
                    }

                    Err(e) => return Err(e)
                }
            }
        }

        // <biglike.org> farm likes:
        if self.task.lock().await.to_close() { return Ok(false); }
        if self.profile.farm_likes && *likes_limit > 0 {
            if let Err(e) = self.biglike.vk_farm_likes(self.settings.biglike.likes_min_price, &mut likes_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                warn!("({}) <biglike.org> The tasks for 'likes' type are over, timeout for 10 seconds..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => return Err(e),
                        }
                    }

                    Err(e) => return Err(e)
                }
            }
        }

        // <biglike.org> farm friends:
        if self.task.lock().await.to_close() { return Ok(false); }
        if self.profile.farm_friends && *friends_limit > 0 {
            if let Err(e) = self.biglike.vk_farm_friends(self.settings.biglike.friends_min_price, &mut friends_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                warn!("({}) <biglike.org> The tasks for 'friends' type are over, timeout for 10 seconds..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => return Err(e),
                        }
                    }

                    Err(e) => return Err(e)
                }
            }
        }

        // <biglike.org> farm subscribes:
        if self.task.lock().await.to_close() { return Ok(false); }
        if self.profile.farm_subscribes && *subscribes_limit > 0 {
            if let Err(e) = self.biglike.vk_farm_subscribes(self.settings.biglike.subscribes_min_price, &mut subscribes_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                warn!("({}) <biglike.org> The tasks for 'subscribes' type are over, timeout for 10 seconds..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => return Err(e),
                        }
                    }

                    Err(e) => return Err(e)
                }
            }
        }

        Ok(true)
    }

    /// Close profile session
    pub async fn close(&mut self) -> Result<()> {
        if let Some(session) = self.session.take() {
            session.close().await?;
        }

        self.task.lock().await.set_as_closed();

        emit_event("bot-stopped", hash_map! {
            "bot_id": Value::String(self.bot_id.clone()),
        });
        
        warn!("({}) Session is closed!", self.profile.name);
        Ok(())
    }
}
