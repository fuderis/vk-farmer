use crate::prelude::*;
use super::{ Config, FreeLikes, BigLike };

use chromedriver_api::Session;
use tokio::time::{ sleep, Duration };

// Farmer bot
#[derive(Debug)]
pub struct Farmer {
    config: Config,
    session: Session,
    freelikes: FreeLikes,
    biglike: BigLike,
}

impl Farmer {
    /// Login to profile
    pub async fn login(config: Config) -> Result<Self> {
        // run session:
        let path = fmt!("C:\\Users\\Synap\\AppData\\Local\\Google\\Chrome\\Profiles\\{}", &config.profile);
        let mut session = Session::run(&config.port, Some(&path)).await?;

        // init task tab:
        let task_tab = session.open("https://vk.com/").await?;
        
        // init sources:
        let freelikes = FreeLikes::login(&config.profile, &mut session, task_tab.clone()).await?;
        let biglike = BigLike::login(&config.profile, &mut session, task_tab.clone()).await?;

        println!("[INFO] ({}) Session is ready.", &config.profile);

        Ok(Self {
            config,
            session,
            freelikes,
            biglike,
        })
    }

    /// Start farming
    pub async fn farm(&mut self) -> Result<()> {
        loop {
            // <freelikes.online> farm likes:
            if self.config.freelikes.farm_likes {
                match self.freelikes.farm_likes(self.config.freelikes.likes_min_price, self.config.freelikes.likes_limit).await {
                    Err(e) => {
                        match e.downcast::<Error>() {
                            Ok(err) => {
                                match err.kind {
                                    ErrorKind::NoMoreTasks => {
                                        println!("[INFO] ({}) <freelikes.online> The tasks are over, timeout for 10 seconds ..", self.config.profile);
                                        sleep(Duration::from_secs(10)).await;
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("[ERROR] ({}) <freelikes.online> {e}", self.config.profile);
                            }
                        }
                    }

                    _ => {}
                }
            }

            // <freelikes.online> farm subscribes:
            if self.config.freelikes.farm_subscribes {
                match self.freelikes.farm_subscribes(self.config.freelikes.subscribes_min_price, self.config.freelikes.subscribes_limit).await {
                    Err(e) => {
                        match e.downcast::<Error>() {
                            Ok(err) => {
                                match err.kind {
                                    ErrorKind::NoMoreTasks => {
                                        println!("[INFO] ({}) <freelikes.online> The tasks are over, timeout for 10 seconds ..", self.config.profile);
                                        sleep(Duration::from_secs(10)).await;
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("[ERROR] ({}) <freelikes.online> {e}", self.config.profile);
                            }
                        }
                    }

                    _ => {}
                }
            }

            // <biglike.org> farm likes:
            if self.config.biglike.farm_likes {
                match self.biglike.farm_likes(self.config.biglike.likes_min_price, self.config.biglike.likes_limit).await {
                    Err(e) => {
                        match e.downcast::<Error>() {
                            Ok(err) => {
                                match err.kind {
                                    ErrorKind::NoMoreTasks => {
                                        println!("[INFO] ({}) <freelikes.online> The tasks are over, timeout for 10 seconds ..", self.config.profile);
                                        sleep(Duration::from_secs(10)).await;
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("[ERROR] ({}) <biglike.org> {e}", self.config.profile);
                            }
                        }
                    }

                    _ => {}
                }
            }

            // <biglike.org> farm subscribes:
            if self.config.biglike.farm_subscribes {
                match self.biglike.farm_subscribes(self.config.biglike.subscribes_min_price, self.config.biglike.subscribes_limit).await {
                    Err(e) => {
                        match e.downcast::<Error>() {
                            Ok(err) => {
                                match err.kind {
                                    ErrorKind::NoMoreTasks => {
                                        println!("[INFO] ({}) <freelikes.online> The tasks are over, timeout for 10 seconds ..", self.config.profile);
                                        sleep(Duration::from_secs(10)).await;
                                    }
                                }
                            }

                            Err(e) => {
                                eprintln!("[ERROR] ({}) <biglike.org> {e}", self.config.profile);
                            }
                        }
                    }

                    _ => {}
                }
            }

            println!("[INFO] (FARMER) <{}> Farming paused for 10 minutes ..", self.config.profile);
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
