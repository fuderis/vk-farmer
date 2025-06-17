use crate::{ prelude::*, Profile, Settings, Task };
use super::{ VKontakte, FreeLikes, BigLike };

use chromedriver_api::Session;
use tokio::time::{ sleep, Duration };

/// Farmer bot
#[allow(dead_code)]
#[derive(Debug)]
pub struct Farmer {
    task: Arc<Mutex<Task>>,
    
    port: String,
    profile: Profile,
    settings: Settings,
    
    session: Option<Session>,
    vkontakte: Arc<Mutex<VKontakte>>,
    freelikes: FreeLikes,
    biglike: BigLike
}

impl Farmer {
    /// Login to profile
    pub async fn login<S: Into<String>>(task: Arc<Mutex<Task>>, port: S, profile: Profile, settings: Settings) -> Result<Arc<Mutex<Self>>> {
        let port = port.into();
        
        // run session:
        let path = fmt!("C:\\Users\\Synap\\AppData\\Local\\Google\\Chrome\\Profiles\\{}", &profile.name);
        let mut session = Session::run(&port, Some(&path)).await.map_err(|e| Error::from(fmt!("{e}")))?;

        // init task tabs:
        let vkontakte = Arc::new(Mutex::new(VKontakte::login(&mut session, &profile).await?));
        
        // init work tabs:
        let freelikes = FreeLikes::login(task.clone(), &profile.name, &mut session, vkontakte.clone()).await?;
        let biglike = BigLike::login(task.clone(), &profile.name, &mut session, vkontakte.clone()).await?;

        println!("[INFO] ({}) Session is ready.", &profile.name);

        Ok(Arc::new(Mutex::new(
            Self {
                task,

                port,
                profile,
                settings,

                session: Some(session),
                vkontakte,
                freelikes,
                biglike
            }
        )))
    }

    /// Start farming
    pub async fn farm(&mut self) -> Result<()> {
        let mut likes_limit = self.profile.likes_limit;
        let mut friends_limit = self.profile.friends_limit;
        let mut subscribes_limit = self.profile.subscribes_limit;

        loop {
            if !self.farm_handler(&mut likes_limit, &mut friends_limit, &mut subscribes_limit).await? {
                println!("[INFO] ({}) Farming canceled! ..", self.profile.name);
                return Ok(());
            }

            if !self.task.lock().await.check_limits() {
                println!("[INFO] ({}) All tasks is completed! ..", self.profile.name);
            } else {
                println!("[INFO] ({}) Tasks is over, timeout for {} minutes ..", self.profile.name, self.settings.pause_delay);

                for _ in 0..(self.settings.pause_delay as u64 * 60) {
                    if self.task.lock().await.is_closed() {
                        println!("[INFO] ({}) Farming canceled! ..", self.profile.name);
                        return Ok(());
                    }
                    
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }

    /// The farming handler
    async fn farm_handler(&mut self, mut likes_limit: &mut usize, mut friends_limit: &mut usize, mut subscribes_limit: &mut usize) -> Result<bool> {
        // <freelikes.online> farm likes:
        if self.task.lock().await.is_closed() { return Ok(false); }
        if self.profile.farm_likes && *likes_limit > 0 {
            if let Err(e) = self.freelikes.vk_farm_likes(self.settings.freelikes.likes_min_price, &mut likes_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                println!("[INFO] ({}) <freelikes.online> The tasks for 'likes' type are over, timeout for 10 seconds ..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => eprintln!("[ERROR] ({}) <freelikes.online> {}", self.profile.name, e.as_ref())
                        }
                    }

                    Err(e) => {
                        eprintln!("[ERROR] ({}) <freelikes.online> {e}", self.profile.name);
                    }
                }
            }
        }

        // <freelikes.online> farm friends:
        if self.task.lock().await.is_closed() { return Ok(false); }
        if self.profile.farm_friends && *friends_limit > 0 {
            if let Err(e) = self.freelikes.vk_farm_friends(self.settings.freelikes.friends_min_price, &mut friends_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                println!("[INFO] ({}) <freelikes.online> The tasks for 'friends' type are over, timeout for 10 seconds ..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => eprintln!("[ERROR] ({}) <freelikes.online> {}", self.profile.name, e.as_ref())
                        }
                    }

                    Err(e) => {
                        eprintln!("[ERROR] ({}) <freelikes.online> {e}", self.profile.name);
                    }
                }
            }
        }

        // <freelikes.online> farm subscribes:
        if self.task.lock().await.is_closed() { return Ok(false); }
        if self.profile.farm_subscribes && *subscribes_limit > 0 {
            if let Err(e) = self.freelikes.vk_farm_subscribes(self.settings.freelikes.subscribes_min_price, &mut subscribes_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                println!("[INFO] ({}) <freelikes.online> The tasks for 'subscribes' type are over, timeout for 10 seconds ..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => eprintln!("[ERROR] ({}) <freelikes.online> {}", self.profile.name, e.as_ref())
                        }
                    }

                    Err(e) => {
                        eprintln!("[ERROR] ({}) <freelikes.online> {e}", self.profile.name);
                    }
                }
            }
        }

        // <biglike.org> farm likes:
        if self.task.lock().await.is_closed() { return Ok(false); }
        if self.profile.farm_likes && *likes_limit > 0 {
            if let Err(e) = self.biglike.vk_farm_likes(self.settings.biglike.likes_min_price, &mut likes_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                println!("[INFO] ({}) <biglike.org> The tasks for 'likes' type are over, timeout for 10 seconds ..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => eprintln!("[ERROR] ({}) <biglike.org>> {}", self.profile.name, e.as_ref())
                        }
                    }

                    Err(e) => {
                        eprintln!("[ERROR] ({}) <biglike.org> {e}", self.profile.name);
                    }
                }
            }
        }

        // <biglike.org> farm friends:
        if self.task.lock().await.is_closed() { return Ok(false); }
        if self.profile.farm_friends && *friends_limit > 0 {
            if let Err(e) = self.biglike.vk_farm_friends(self.settings.biglike.friends_min_price, &mut friends_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                println!("[INFO] ({}) <biglike.org> The tasks for 'friends' type are over, timeout for 10 seconds ..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => eprintln!("[ERROR] ({}) <biglike.org>> {}", self.profile.name, e.as_ref())
                        }
                    }

                    Err(e) => {
                        eprintln!("[ERROR] ({}) <biglike.org> {e}", self.profile.name);
                    }
                }
            }
        }

        // <biglike.org> farm subscribes:
        if self.task.lock().await.is_closed() { return Ok(false); }
        if self.profile.farm_subscribes && *subscribes_limit > 0 {
            if let Err(e) = self.biglike.vk_farm_subscribes(self.settings.biglike.subscribes_min_price, &mut subscribes_limit).await {
                match e.downcast::<Error>() {
                    Ok(e) => {
                        match e.as_ref() {
                            Error::NoMoreTasks => {
                                println!("[INFO] ({}) <biglike.org> The tasks for 'subscribes' type are over, timeout for 10 seconds ..", self.profile.name);
                                sleep(Duration::from_secs(10)).await;
                            },

                            _ => eprintln!("[ERROR] ({}) <biglike.org>> {}", self.profile.name, e.as_ref())
                        }
                    }

                    Err(e) => {
                        eprintln!("[ERROR] ({}) <biglike.org> {e}", self.profile.name);
                    }
                }
            }
        }

        Ok(true)
    }

    /// Close profile session
    pub async fn close(&mut self) -> Result<()> {
        println!("[INFO] ({}) Closing bot session ..", self.profile.name);

        if let Some(session) = self.session.take() {
            session.close().await.map_err(|e| Error::from(fmt!("{e}")))?;
        }

        Ok(())
    }
}
