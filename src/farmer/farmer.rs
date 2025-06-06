use crate::{ prelude::*, Profile, Settings };
use super::{ FreeLikes, BigLike };

use chromedriver_api::Session;
use tokio::time::{ sleep, Duration };

// Farmer bot
#[derive(Debug)]
pub struct Farmer {
    profile: Profile,
    settings: Settings,
    session: Session,
    freelikes: FreeLikes,
    biglike: BigLike,
}

impl Farmer {
    /// Login to profile
    pub async fn login(settings: Settings, profile: Profile) -> Result<Self> {
        // run session:
        let path = fmt!("C:\\Users\\Synap\\AppData\\Local\\Google\\Chrome\\Profiles\\{}", &profile.profile);
        let mut session = Session::run(&profile.port, Some(&path)).await?;

        // init task tab:
        let task_tab = session.open("https://vk.com/").await?;
        
        // init sources:
        let freelikes = FreeLikes::login(&profile.profile, &mut session, task_tab.clone()).await?;
        let biglike = BigLike::login(&profile.profile, &mut session, task_tab.clone()).await?;

        println!("[INFO] ({}) Session is ready.", &profile.profile);

        Ok(Self {
            profile,
            settings,
            session,
            freelikes,
            biglike,
        })
    }

    /// Start farming
    pub async fn farm(&mut self) -> Result<()> {
        loop {
            // <freelikes.online> farm likes:
            if self.profile.farm_likes {
                if let Err(e) = self.freelikes.vk_farm_likes(self.settings.freelikes.likes_min_price, self.settings.freelikes.likes_limit).await {
                    match e.downcast::<Error>() {
                        Ok(e) => {
                            match e.as_ref() {
                                Error::NoMoreTasks => {
                                    println!("[INFO] ({}) <freelikes.online> The tasks 'likes' are over, timeout for 10 seconds ..", self.profile.profile);
                                    sleep(Duration::from_secs(10)).await;
                                }
                            }
                        }

                        Err(e) => {
                            eprintln!("[ERROR] ({}) <freelikes.online> {e}", self.profile.profile);
                        }
                    }
                }
            }

            // <freelikes.online> farm friends:
            if self.profile.farm_friends {
                if let Err(e) = self.freelikes.vk_farm_friends(self.settings.freelikes.friends_min_price, self.settings.freelikes.friends_limit).await {
                    match e.downcast::<Error>() {
                        Ok(e) => {
                            match e.as_ref() {
                                Error::NoMoreTasks => {
                                    println!("[INFO] ({}) <freelikes.online> The tasks 'friends' are over, timeout for 10 seconds ..", self.profile.profile);
                                    sleep(Duration::from_secs(10)).await;
                                }
                            }
                        }

                        Err(e) => {
                            eprintln!("[ERROR] ({}) <freelikes.online> {e}", self.profile.profile);
                        }
                    }
                }
            }

            // <freelikes.online> farm subscribes:
            if self.profile.farm_subscribes {
                if let Err(e) = self.freelikes.vk_farm_subscribes(self.settings.freelikes.subscribes_min_price, self.settings.freelikes.subscribes_limit).await {
                    match e.downcast::<Error>() {
                        Ok(e) => {
                            match e.as_ref() {
                                Error::NoMoreTasks => {
                                    println!("[INFO] ({}) <freelikes.online> The tasks 'subscribes' are over, timeout for 10 seconds ..", self.profile.profile);
                                    sleep(Duration::from_secs(10)).await;
                                }
                            }
                        }

                        Err(e) => {
                            eprintln!("[ERROR] ({}) <freelikes.online> {e}", self.profile.profile);
                        }
                    }
                }
            }


            // <biglike.org> farm likes:
            if self.profile.farm_likes {
                if let Err(e) = self.biglike.vk_farm_likes(self.settings.biglike.likes_min_price, self.settings.biglike.likes_limit).await {
                    match e.downcast::<Error>() {
                        Ok(e) => {
                            match e.as_ref() {
                                Error::NoMoreTasks => {
                                    println!("[INFO] ({}) <biglike.org> The tasks 'likes' are over, timeout for 10 seconds ..", self.profile.profile);
                                    sleep(Duration::from_secs(10)).await;
                                }
                            }
                        }

                        Err(e) => {
                            eprintln!("[ERROR] ({}) <biglike.org> {e}", self.profile.profile);
                        }
                    }
                }
            }

            // <biglike.org> farm friends:
            if self.profile.farm_friends {
                if let Err(e) = self.biglike.vk_farm_friends(self.settings.biglike.friends_min_price, self.settings.biglike.friends_limit).await {
                    match e.downcast::<Error>() {
                        Ok(e) => {
                            match e.as_ref() {
                                Error::NoMoreTasks => {
                                    println!("[INFO] ({}) <biglike.org> The tasks 'friends' are over, timeout for 10 seconds ..", self.profile.profile);
                                    sleep(Duration::from_secs(10)).await;
                                }
                            }
                        }

                        Err(e) => {
                            eprintln!("[ERROR] ({}) <biglike.org> {e}", self.profile.profile);
                        }
                    }
                }
            }

            // <biglike.org> farm subscribes:
            if self.profile.farm_subscribes {
                if let Err(e) = self.biglike.vk_farm_subscribes(self.settings.biglike.subscribes_min_price, self.settings.biglike.subscribes_limit).await {
                    match e.downcast::<Error>() {
                        Ok(e) => {
                            match e.as_ref() {
                                Error::NoMoreTasks => {
                                    println!("[INFO] ({}) <biglike.org> The tasks 'subscribes' are over, timeout for 10 seconds ..", self.profile.profile);
                                    sleep(Duration::from_secs(10)).await;
                                }
                            }
                        }

                        Err(e) => {
                            eprintln!("[ERROR] ({}) <biglike.org> {e}", self.profile.profile);
                        }
                    }
                }
            }

            println!("[INFO] (FARMER) <{}> Farming paused for 10 minutes ..", self.profile.profile);
            sleep(Duration::from_secs(60 * 10)).await;
        }
    }

    /// Close profile session
    pub async fn close(self) -> Result<()> {
        sleep(Duration::from_millis(100)).await;
        self.session.close().await?;

        Ok(())
    }
}
