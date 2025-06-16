use crate::{ prelude::*, Profile, Settings };
use super::{ VKontakte, FreeLikes, BigLike };

use chromedriver_api::Session;
use tokio::time::{ sleep, Duration };

/// Farmer async controller
#[derive(Debug)]
pub struct Controller {
    to_close: bool,
}

impl Controller {
    /// Creates a new bot controller
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
            Self {
                to_close: false,
            }
        ))
    }
    
    /// Check session to alive
    pub fn is_alive(&self) -> bool {
        !self.to_close
    }
    
    /// Closes bot session
    pub fn close(&mut self) {
        self.to_close = true;
    }
}

/// Farmer bot
#[allow(dead_code)]
#[derive(Debug)]
pub struct Farmer {
    control: Arc<Mutex<Controller>>,
    
    port: String,
    profile: Profile,
    settings: Settings,
    
    session: Session,
    vkontakte: Arc<Mutex<VKontakte>>,
    freelikes: FreeLikes,
    biglike: BigLike,
}

impl Farmer {
    /// Login to profile
    pub async fn login<S: Into<String>>(control: Arc<Mutex<Controller>>, port: S, profile: Profile, settings: Settings) -> Result<Self> {
        let port = port.into();
        
        // run session:
        let path = fmt!("C:\\Users\\Synap\\AppData\\Local\\Google\\Chrome\\Profiles\\{}", &profile.name);
        let mut session = Session::run(&port, Some(&path)).await.map_err(|e| Error::from(fmt!("{e}")))?;

        // init task tabs:
        let vkontakte = Arc::new(Mutex::new(VKontakte::login(&mut session, &profile).await?));
        
        // init work tabs:
        let freelikes = FreeLikes::login(control.clone(), &profile.name, &mut session, vkontakte.clone()).await?;
        let biglike = BigLike::login(control.clone(), &profile.name, &mut session, vkontakte.clone()).await?;

        println!("[INFO] ({}) Session is ready.", &profile.name);

        Ok(Self {
            control,
            
            port,
            profile,
            settings,

            session,
            vkontakte,
            freelikes,
            biglike,
        })
    }

    /// Start farming
    pub async fn farm(mut self) -> Result<()> {
        let mut likes_limit = self.profile.likes_limit;
        let mut friends_limit = self.profile.friends_limit;
        let mut subscribes_limit = self.profile.subscribes_limit;
        
        // <freelikes.online> farm likes:
        if !self.control.lock().await.is_alive() { return self.close().await; }
        if self.profile.farm_likes {
            if likes_limit == 0 {
                println!("[INFO] <{}> The limit of tasks 'likes' are over ..", self.profile.name);
            }

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
        if !self.control.lock().await.is_alive() { return self.close().await; }
        if self.profile.farm_friends {
            if friends_limit == 0 {
                println!("[INFO] <{}> The limit of tasks 'friends' are over ..", self.profile.name);
            }
            
            else if let Err(e) = self.freelikes.vk_farm_friends(self.settings.freelikes.friends_min_price, &mut friends_limit).await {
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
        if !self.control.lock().await.is_alive() { return self.close().await; }
        if self.profile.farm_subscribes {
            if subscribes_limit == 0 {
                println!("[INFO] <{}> The limit of tasks 'subscribes' are over ..", self.profile.name);
            }
            
            else if let Err(e) = self.freelikes.vk_farm_subscribes(self.settings.freelikes.subscribes_min_price, &mut subscribes_limit).await {
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
        if !self.control.lock().await.is_alive() { return self.close().await; }
        if self.profile.farm_likes {
            if likes_limit == 0 {
                println!("[INFO] <{}> The limit of tasks 'likes' are over ..", self.profile.name);
            }
            
            else if let Err(e) = self.biglike.vk_farm_likes(self.settings.biglike.likes_min_price, &mut likes_limit).await {
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
        if !self.control.lock().await.is_alive() { return self.close().await; }
        if self.profile.farm_friends {
            if friends_limit == 0 {
                println!("[INFO] <{}> The limit of tasks 'friends' are over ..", self.profile.name);
            }
            
            else if let Err(e) = self.biglike.vk_farm_friends(self.settings.biglike.friends_min_price, &mut friends_limit).await {
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
        if !self.control.lock().await.is_alive() { return self.close().await; }
        if self.profile.farm_subscribes {
            if subscribes_limit == 0 {
                println!("[INFO] <{}> The limit of tasks 'subscribes' are over ..", self.profile.name);
            }
            
            else if let Err(e) = self.biglike.vk_farm_subscribes(self.settings.biglike.subscribes_min_price, &mut subscribes_limit).await {
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

        println!("[INFO] <{}> All tasks is completed! ..", self.profile.name);
        self.close().await
    }

    /// Close profile session
    pub async fn close(self) -> Result<()> {
        println!("[INFO] <{}> Closing bot session ..", self.profile.name);
        sleep(Duration::from_millis(100)).await;

        self.session.close().await.map_err(|e| Error::from(fmt!("{e}")))?;

        Ok(())
    }
}
