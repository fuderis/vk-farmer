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

    /// Start settings
    pub async fn farm(&mut self) -> Result<()> {
        loop {
            // <freelikes.online> farm likes:
            if self.settings.freelikes.farm_likes {
                match self.freelikes.farm_likes(self.settings.freelikes.likes_min_price, self.settings.freelikes.likes_limit).await {
                    Err(e) => {
                        match e.downcast::<Error>() {
                            Ok(err) => {
                                match err.kind {
                                    ErrorKind::NoMoreTasks => {
                                        println!("[INFO] ({}) <freelikes.online> The tasks are over, timeout for 10 seconds ..", self.profile.profile);
                                        sleep(Duration::from_secs(10)).await;
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("[ERROR] ({}) <freelikes.online> {e}", self.profile.profile);
                            }
                        }
                    }

                    _ => {}
                }
            }

            // <freelikes.online> farm subscribes:
            if self.settings.freelikes.farm_subscribes {
                match self.freelikes.farm_subscribes(self.settings.freelikes.subscribes_min_price, self.settings.freelikes.subscribes_limit).await {
                    Err(e) => {
                        match e.downcast::<Error>() {
                            Ok(err) => {
                                match err.kind {
                                    ErrorKind::NoMoreTasks => {
                                        println!("[INFO] ({}) <freelikes.online> The tasks are over, timeout for 10 seconds ..", self.profile.profile);
                                        sleep(Duration::from_secs(10)).await;
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("[ERROR] ({}) <freelikes.online> {e}", self.profile.profile);
                            }
                        }
                    }

                    _ => {}
                }
            }

            // <biglike.org> farm likes:
            if self.settings.biglike.farm_likes {
                match self.biglike.farm_likes(self.settings.biglike.likes_min_price, self.settings.biglike.likes_limit).await {
                    Err(e) => {
                        match e.downcast::<Error>() {
                            Ok(err) => {
                                match err.kind {
                                    ErrorKind::NoMoreTasks => {
                                        println!("[INFO] ({}) <freelikes.online> The tasks are over, timeout for 10 seconds ..", self.profile.profile);
                                        sleep(Duration::from_secs(10)).await;
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("[ERROR] ({}) <biglike.org> {e}", self.profile.profile);
                            }
                        }
                    }

                    _ => {}
                }
            }

            // <biglike.org> farm subscribes:
            if self.settings.biglike.farm_subscribes {
                match self.biglike.farm_subscribes(self.settings.biglike.subscribes_min_price, self.settings.biglike.subscribes_limit).await {
                    Err(e) => {
                        match e.downcast::<Error>() {
                            Ok(err) => {
                                match err.kind {
                                    ErrorKind::NoMoreTasks => {
                                        println!("[INFO] ({}) <freelikes.online> The tasks are over, timeout for 10 seconds ..", self.profile.profile);
                                        sleep(Duration::from_secs(10)).await;
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("[ERROR] ({}) <biglike.org> {e}", self.profile.profile);
                            }
                        }
                    }

                    _ => {}
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
