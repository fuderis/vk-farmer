use crate::prelude::*;

use chromedriver_api::{ Session, Tab };
use tokio::time::{ sleep, Duration };

// Farmer for 'freelikes.online'
#[derive(Debug)]
pub struct FreeLikes {
    pub(crate) profile: String,
    pub(crate) work_tab: Arc<Mutex<Tab>>,
    pub(crate) task_tab: Arc<Mutex<Tab>>
}

impl FreeLikes {
    /// Login to profile
    pub async fn login<S: Into<String>>(profile: S, session: &mut Session, task_tab: Arc<Mutex<Tab>>) -> Result<Self> {
        let profile = profile.into();
        
        // open tabs:
        let work_tab = session.open("https://freelikes.online/").await?;

        println!("[INFO] ({profile}) <freelikes.online> Session is ready.");

        Ok(Self {
            profile,
            work_tab,
            task_tab
        })
    }

    // Earn on put likes
    pub async fn farm_likes(&mut self, min_price: usize, mut limit: usize) -> Result<()> {
        let mut work_tab = self.work_tab.lock().await;
        let mut task_tab = self.task_tab.lock().await;
        
        // open page with tasks:
        work_tab.open("https://freelikes.online/earn/vkontakte/vklike").await?;
        
        while limit > 0 {
            println!("[INFO] ({}) <freelikes.online> Searching task..", self.profile);

            // searching next task:
            let log = work_tab.inject(&fmt!(r#"{{
                var button = document.querySelector('button.do-task .insidebtn[onclick]');

                if (button && Number( button.textContent.match(/(\d+) баллов/)[1] ) >= {min_price}) {{
                    button.classList.add("is_active");
                
                    var button_check = button.parentElement.querySelector('.insidebtn:not(.is_active)[onclick]');
                    button_check.classList.add("check_task");

                    task_url = button?.getAttribute('onclick')?.match(/(?:"|')(https?:\/\/[^"][^']*)(?:"|')/)?.[1] ?? null;
                    return task_url;
                }}
                
                return null;
            }}"#)).await?;

            let task_url = log.to_string().trim().replace("\"", "").replace("'", "");
            
            // if task is finded:
            if task_url != "null" {
                println!("[INFO] ({}) <freelikes.online> Processing task on '{task_url}'..", self.profile);
                
                task_tab.open(&task_url).await?;
                sleep(Duration::from_secs(1)).await;

                // click to 'like' button
                task_tab.inject(&fmt!(r#"{{
                    let button_like = document.querySelector('div[aria-label="Отправить реакцию «Лайк»"]');

                    if (!button_like) {{
                        button_like = document.querySelector('div.like_button_count');
                    }}
                        
                    if (!button_like) {{
                        button_like = document.querySelector('div[data-testid="video_modal_like_button"]');

                        if (button_like.getAttribute('style').match('--vkui--color_accent_red')) {{
                            button_like = null;
                        }}
                    }}

                    if (button_like) {{
                        button_like.focus();
                        button_like.click();
                    }}
                }}"#)).await?;

                sleep(Duration::from_secs(1)).await;

                // click to 'check task' button
               work_tab.inject(&fmt!(r#"{{
                    var button_check = document.querySelector('.check_task');
                    button_check.classList.remove('check_task');
                    button_check.parentElement.classList.remove('do-task');

                    if (button_check) {{
                        button_check.focus();
                        button_check.click();
                    }}
                }}"#)).await?;

                limit -= 1;

                println!("[INFO] ({}) <freelikes.online> Task is completed, tasks remained {limit}", self.profile);
                sleep(Duration::from_secs(2)).await;
            } else {
                println!("[INFO] ({}) <freelikes.online> The tasks for the 'likes' type are over..", self.profile);
                
                return Err(Error { kind: ErrorKind::NoMoreTasks }.into())
            }
        }

        Ok(())
    }

    // Earn on put subscribers
    pub async fn farm_subscribes(&mut self, min_price: usize, mut limit: usize) -> Result<()> {
        let mut work_tab = self.work_tab.lock().await;
        let mut task_tab = self.task_tab.lock().await;
        
        // open page with tasks:
        work_tab.open("https://freelikes.online/earn/vkontakte/vkgroup").await?;
        
        while limit > 0 {
            println!("[INFO] ({}) <freelikes.online> Searching task..", self.profile);

            // searching next task:
            let log = work_tab.inject(&fmt!(r#"{{
                var task_button = document.querySelector('button.do-task .insidebtn[onclick]');

                if (task_button && Number( task_button.textContent.match(/(\d+) баллов/)[1] ) >= {min_price}) {{
                    task_button.classList.add('is_active');
                
                    var button_check = task_button.parentElement.querySelector('.insidebtn:not(.is_active)[onclick]');
                    button_check.classList.add('check_task');

                    task_url = task_button?.getAttribute('onclick')?.match(/(?:"|')(https?:\/\/([^"]|[^'])*)(?:"|')/)?.[1] ?? null;
                    return task_url;
                }}
                
                return null;
            }}"#)).await?;

            let task_url = log.to_string().trim().replace("\"", "").replace("'", "");
            
            // if task is finded:
            if task_url != "null" {
                println!("[INFO] ({}) <freelikes.online> Processing task on '{task_url}'..", self.profile);

                task_tab.open(&task_url).await?;
                sleep(Duration::from_secs(1)).await;

                // click to 'subscribe' button
                task_tab.inject(&fmt!(r#"{{
                    var subscribe_button = document.querySelector('button#public_subscribe');

                    if (!subscribe_button) {{
                        subscribe_button = document.querySelector('button#join_button');
                    }}
                    
                    if (subscribe_button) {{
                        subscribe_button.focus();
                        subscribe_button.click();
                    }}
                }}"#)).await?;

                sleep(Duration::from_secs(1)).await;

                // click to 'check task' button
                work_tab.inject(&fmt!(r#"{{
                    var button_check = document.querySelector('.check_task');
                    button_check.classList.remove("check_task");
                    button_check.parentElement.classList.remove("do-task");

                    if (button_check) {{
                        button_check.focus();
                        button_check.click();
                    }}
                }}"#)).await?;

                limit -= 1;

                println!("[INFO] ({}) <freelikes.online> Task is completed, tasks remained {limit}", self.profile);
                sleep(Duration::from_secs(2)).await;
            } else {
                println!("[INFO] ({}) <freelikes.online> The tasks for the 'subscribers' type are over..", self.profile);
                
                return Err(Error { kind: ErrorKind::NoMoreTasks }.into())
            }
        }

        Ok(())
    }

    // Earn on put friends
    /* TODO: pub async fn farm_friends(&mut self, mut limit: usize) -> Result<()> {
        work_tab.open("https://freelikes.online/earn/vkontakte/vkfriend").await?;
        
        while limit > 0 {
            println!("[INFO] ({}) <freelikes.online> Searching task..", self.profile);

            // searching next task:
            let log = work_tab.inject(&fmt!(r#"
                var __button__ = document.querySelector('button.do-task .insidebtn[onclick]');

                if (__button__ && Number( __button__.textContent.match(/(\d+) баллов/)[1] ) >= {ADD_FRIEND_MIN_PRICE}) {{
                    __button__.classList.add("is_active");
                
                    var button_check = __button__.parentElement.querySelector('.insidebtn:not(.is_active)[onclick]');
                    button_check.classList.add("check_task");

                    task_url = __button__?.getAttribute('onclick')?.match(/"(https?:\/\/[^"]*)"/)?.[1] ?? null;
                    return task_url;
                }}
                
                return null;
            "#)).await?;

            let task_url = log.to_string().trim().replace("\"", "").replace("'", "");
            
            // if task is finded:
            if task_url != "null" {
                println!("[INFO] ({}) <freelikes.online> Processing task on '{task_url}'..", self.profile);

                task_tab.open(&task_url).await?;
                sleep(Duration::from_secs(1)).await;

                // click to 'subscribe' button
                task_tab.inject(r#"
                    var __button_subscribe__ = document.querySelector('.ProfileHeaderButton button');

                    if (__button_subscribe__) {
                        __button_subscribe__.focus();
                        __button_subscribe__.click();
                    }
                "#).await?;

                sleep(Duration::from_secs(1)).await;

                // click to 'check task' button
                work_tab.inject(r#"
                    var button_check = document.querySelector('.check_task');
                    button_check.classList.remove("check_task");
                    button_check.parentElement.classList.remove("do-task");

                    if (button_check) {
                        button_check.focus();
                        button_check.click();
                    }
                "#).await?;

                limit -= 1;

                println!("[INFO] ({}) <freelikes.online> Task is completed, tasks remained {limit}", self.profile);
                sleep(Duration::from_secs(2)).await;
            } else {
                println!("[INFO] ({}) <freelikes.online> The tasks for the 'subscribers' type are over..", self.profile);
                
                return Err(Error { kind: ErrorKind::NoMoreTasks }.into())
            }
        }

        Ok(())
    } */

    /// Close profile session
    pub async fn close(self) -> Result<()> {
        sleep(Duration::from_millis(100)).await;

        self.work_tab.lock().await.close().await?;
        self.task_tab.lock().await.close().await?;
        
        Ok(())
    }
}
