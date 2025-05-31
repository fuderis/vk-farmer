use crate::prelude::*;

use chromedriver_api::{ Session, Tab };
use tokio::time::{ sleep, Duration };

// Farmer for 'biglike.org'
#[derive(Debug)]
pub struct BigLike {
    pub(crate) profile: String,
    pub(crate) work_tab: Arc<Mutex<Tab>>,
    pub(crate) task_tab: Arc<Mutex<Tab>>
}

impl BigLike {
    /// Login to profile
    pub async fn login<S: Into<String>>(profile: S, session: &mut Session, task_tab: Arc<Mutex<Tab>>) -> Result<Self> {
        let profile = profile.into();
        
        // open tabs:
        let work_tab = session.open("https://biglike.org/").await?;

        println!("[INFO] ({profile}) <biglike.org> Session is ready.");

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
        work_tab.open("https://biglike.org/vklike").await?;
        
        while limit > 0 {
            println!("[INFO] ({}) <biglike.org> Searching task..", self.profile);

            // searching next task:
            let result = work_tab.inject(&fmt!(r#"{{
                let tasks = document.querySelectorAll('#row_albom > div');

                if (tasks) {{
                    let task = Array.from(tasks)
                        .find(el => !el.hasAttribute('completed-task'));

                    if (task) {{
                        let price = task.querySelector('p')
                            .textContent
                            .match(/\+(\d+)/)[1];

                        if (Number(price) < {min_price}) {{
                            task.setAttribute('completed-task', '');

                            return null;
                        }}
                        
                        let task_url = task.querySelector('img')
                            .getAttribute('onclick')
                            .match(/(?:'|")(https?:\/\/([^"]|[^'])*)(?:'|")/)[1];

                        if (task_url) {{
                            task.setAttribute('active-task', '');

                            return task_url;
                        }}
                    }}
                }}

                return null;
            }}"#)).await?;

            let task_url = result.to_string().trim().replace("\"", "").replace("'", "");
            
            // set like:
            if task_url != "null" {
                println!("[INFO] ({}) <biglike.org> Processing task on '{task_url}'..", self.profile);
                
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

                        if (button_like && button_like.getAttribute('style').match('--vkui--color_accent_red')) {{
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
                    let task = document.querySelector('#row_albom > div[active-task]');
                    let check_button = task.querySelector('button');

                    task.removeAttribute('active-task');
                    task.setAttribute('completed-task', '');
               
                    check_button.focus();
                    check_button.click();
                }}"#)).await?;

                limit -= 1;

                println!("[INFO] ({}) <biglike.org> Task is completed, tasks remained {limit}", self.profile);
                sleep(Duration::from_secs(2)).await;
            } else {
                println!("[INFO] ({}) <biglike.org> The tasks for the 'likes' type are over..", self.profile);
                
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
        work_tab.open("https://biglike.org/vkgroup").await?;
        
        while limit > 0 {
            println!("[INFO] ({}) <biglike.org> Searching task..", self.profile);

            // searching next task:
            let log = work_tab.inject(&fmt!(r#"{{
                let tasks = document.querySelectorAll('#row_albom > div');

                if (tasks) {{
                    let task = Array.from(tasks)
                        .find(el => !el.hasAttribute('completed-task'));

                    if (task) {{
                        let price = task.querySelector('p')
                            .textContent
                            .match(/\+(\d+)/)[1];

                        if (Number(price) < {min_price}) {{
                            task.setAttribute('completed-task', '');

                            return null;
                        }}
                        
                        let task_url = task.querySelector('img')
                            .getAttribute('onclick')
                            .match(/(?:'|")(https?:\/\/([^"]|[^'])*)(?:'|")/)[1];

                        if (task_url) {{
                            task.setAttribute('active-task', '');

                            return task_url;
                        }}
                    }}
                }}

                return null;
            }}"#)).await?;

            let task_url = log.to_string().trim().replace("\"", "").replace("'", "");
            
            // if task is finded:
            if task_url != "null" {
                println!("[INFO] ({}) <biglike.org> Processing task on '{task_url}'..", self.profile);

                task_tab.open(&task_url).await?;
                sleep(Duration::from_secs(1)).await;

                // click to 'subscribe' button
                task_tab.inject(&fmt!(r#"{{
                    let subscribe_button = document.querySelector('button#public_subscribe');

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
                work_tab.inject(r#"{{
                    let task = document.querySelector('#row_albom > div[active-task]');
                    let check_button = task.querySelector('button');

                    task.removeAttribute('active-task');
                    task.setAttribute('completed-task', '');
               
                    check_button.focus();
                    check_button.click();
                }}"#).await?;

                limit -= 1;

                println!("[INFO] ({}) <biglike.org> Task is completed, tasks remained {limit}", self.profile);
                sleep(Duration::from_secs(2)).await;
            } else {
                println!("[INFO] ({}) <biglike.org> The tasks for the 'subscribers' type are over..", self.profile);
                
                return Err(Error { kind: ErrorKind::NoMoreTasks }.into())
            }
        }

        Ok(())
    }

    /// Close profile session
    pub async fn close(self) -> Result<()> {
        sleep(Duration::from_millis(100)).await;

        self.work_tab.lock().await.close().await?;
        self.task_tab.lock().await.close().await?;
        
        Ok(())
    }
}
