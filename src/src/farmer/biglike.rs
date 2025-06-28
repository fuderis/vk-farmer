use crate::{ prelude::*, Task };
use super::{ FarmTask, VKontakte };

use chromedriver_api::{ Session, Tab };
use tokio::time::{ sleep, Duration };
use serde_json::Value;

// Farmer for 'biglike.org'
#[derive(Debug)]
pub struct BigLike {
    task: Arc<Mutex<Task>>,
    
    pub(crate) profile: String,
    pub(crate) biglike: Arc<Mutex<Tab>>,
    pub(crate) vkontakte: Arc<Mutex<VKontakte>>,
}

impl BigLike {
    /// Login to profile
    pub async fn login<S: Into<String>>(task: Arc<Mutex<Task>>, profile: S, session: &mut Session, vkontakte: Arc<Mutex<VKontakte>>) -> Result<Self> {
        let profile = profile.into();
        
        // open tabs:
        let biglike = session.open("https://biglike.org/vklike").await?;
        let mut biglike_lock = biglike.lock().await;
        
        let mut vk_lock = vkontakte.lock().await;

        // login to account:
        if biglike_lock.inject(r##"return document.querySelector('a[data-target="#login"]')? true: false;"##).await?.to_string() == "true" {
            info!("({}) <biglike.org> Login to account ..", &profile);

            let _ = biglike_lock.inject(&("".to_string() + r##"
                let _btn_enter_ = document.querySelector('a[data-target="#login"]');
                _btn_enter_.focus();
                _btn_enter_.click();
            
                let _inp_link_ = document.querySelector('#form_login input#link');
                _inp_link_.value = 'https://vk.com/id"## + &vk_lock.id[..] + r##"';

                let _btn_login_ = document.querySelector('#form_login button#loginvk');
                _btn_login_.focus();
                _btn_login_.click();
            "##)).await?;
            sleep(Duration::from_secs(1)).await;

            while biglike_lock.inject(r##"return document.querySelector('a[data-target="#login"]')? true: false;"##).await?.to_string() == "true" {
                let log = biglike_lock.inject(r#"
                    let _txt_ = document.querySelector('#alerttxt b');
                
                    return _txt_.textContent;
                "#).await?.to_string();

                let text = &log[1..log.len()-1];

                let old_status = vk_lock.get_status().await?;
                vk_lock.set_status(text).await?;

                let _ = biglike_lock.inject(r#"
                    let _btn_login_ = document.querySelector('input[value="ВОЙТИ"]');
                    _btn_login_.focus();
                    _btn_login_.click();
                "#).await?;
                sleep(Duration::from_secs(1)).await;

                vk_lock.set_status(&old_status).await?;

                sleep(Duration::from_secs(5)).await;
            }
        }

        info!("({profile}) <biglike.org> Session is ready.");

        drop(biglike_lock);
        drop(vk_lock);
        Ok(Self {
            task,

            profile,
            biglike,
            vkontakte
        })
    }

    /// Open tasks page
    async fn open_tasks(&mut self, path: &str) -> Result<()> {
        let mut biglike = self.biglike.lock().await;
        
        // open page:
        biglike.open(&fmt!("https://biglike.org/{path}")).await?;

        // disabling pop-ups:
        biglike.inject(r#"
            window.open = function(_url, _name, _specs) {
                return null;
            };
        "#).await?;

        Ok(())
    }

    /// Search next task
    async fn next_task(&mut self) -> Result<Option<FarmTask>> {
        let mut biglike = self.biglike.lock().await;

        // searching next task:
        let log = biglike.inject(&("return ".to_owned() + r#"(() => {
            let tasks = document.querySelectorAll('#row_albom > div');

            if (tasks) {
                let task = Array.from(tasks).find(el => !el.hasAttribute('task-id') && el.querySelector('img[onclick]'));

                if (task) {
                    let id = 'id-' + Date.now().toString(36);
                    task.setAttribute('task-id', id);

                    let button_start = task.querySelector('img[onclick]');
                    let button_check = task.querySelector('button[onclick]');
                    let button_remove = task.querySelector('button[onclick] ~ button[onclick]');

                    if (button_start && button_check && button_remove) {
                        button_start.setAttribute('start-task', '');
                        button_check.setAttribute('check-task', '');
                        button_remove.setAttribute('remove-task', '');

                        let price = Number(button_start.parentNode.querySelector('p').textContent.match(/(\d+)/)[1]);
                        let url = button_start?.getAttribute('onclick')?.match(/(?:"|')(https:\/\/[^"']+)(?:"|')/)?.[1] ?? null;

                        if (url) {
                            return { "id": id, "url": url, "price": price };
                        }
                    }
                }
            }
        
            return null;
        })();"#)).await?.to_string();

        match serde_json::from_str(&log) {
            Ok(task) => Ok(Some(task)),
            Err(_) => Ok(None)
        }
    }

    /// Start task
    async fn start_task(&mut self, task: &FarmTask) -> Result<Value> {
        let mut biglike = self.biglike.lock().await;

        // click to 'start-task' button:
        let log = biglike.inject(&("return ".to_owned() + r#"(() => {
            let button_start = document.querySelector('div[task-id=""# + &task.id + r#""] img[start-task]');

            if (button_start) {
                button_start.focus();
                button_start.click();
            }
        })();"#)).await?;
        sleep(Duration::from_secs(1)).await;

        Ok(log)
    }

    /// Check task
    async fn check_task(&mut self, task: &FarmTask) -> Result<()> {
        let mut biglike = self.biglike.lock().await;

        // click to 'check task' button
        biglike.inject(&("return ".to_owned() + r#"(() => {
            let button_check = document.querySelector('div[task-id=""# + &task.id + r#""] button[check-task]');

            if (button_check) {
                button_check.focus();
                button_check.click();
            }
        })();"#)).await?;
        sleep(Duration::from_secs(1)).await;

        Ok(())
    }

    /// Remove task
    async fn remove_task(&mut self, task: &FarmTask) -> Result<()> {
        let mut biglike = self.biglike.lock().await;

        // click to 'remove task' button
        biglike.inject(&("return ".to_owned() + r#"(() => {
            let button_remove = document.querySelector('div[task-id=""# + &task.id + r#""] button[remove-task]');

            if (button_remove) {
                button_remove.focus();
                button_remove.click();
            }
        })();"#)).await?;
        sleep(Duration::from_secs(1)).await;

        Ok(())
    }

    // ________________ VK.COM ________________

    async fn vk_like(&mut self, url: &str) -> Result<bool> {
        Ok(self.vkontakte.lock().await.like(url).await?)
    }
    async fn vk_unlike(&mut self, url: &str) -> Result<bool> {
        Ok(self.vkontakte.lock().await.unlike(url).await?)
    }
    async fn vk_subscribe(&mut self, url: &str) -> Result<bool> {
        Ok(self.vkontakte.lock().await.subscribe(url).await?)
    }
    async fn vk_unsubscribe(&mut self, url: &str) -> Result<bool> {
        Ok(self.vkontakte.lock().await.unsubscribe(url).await?)
    }
    async fn vk_add_friend(&mut self, url: &str) -> Result<bool> {
        Ok(self.vkontakte.lock().await.add_friend(url).await?)
    }
    async fn vk_delete_friend(&mut self, url: &str) -> Result<bool> {
        Ok(self.vkontakte.lock().await.delete_friend(url).await?)
    }

    /// Farming VK likes
    pub async fn vk_farm_likes(&mut self, min_price: usize, limit: &mut usize) -> Result<()> {
        // open page with tasks:
        self.open_tasks("vklike").await?;
        
        while *limit > 0 && !self.task.lock().await.to_close() {
            info!("({}) <biglike.org> Searching task..", self.profile);
            
            // searching next task:
            match self.next_task().await? {
                Some(task) if task.price >= min_price => {
                    // starting task:
                    info!("({}) <biglike.org> Starting task ({}) ..", self.profile, task.url);
                    self.vk_unlike(&task.url).await?;
                    self.start_task(&task).await?;

                    // executing task:
                    if !self.vk_like(&task.url).await? {
                        info!("({}) <biglike.org> Task is broken, removing ..", self.profile);
                        self.remove_task(&task).await?;

                        continue;
                    }

                    // checking task:
                    info!("({}) <biglike.org> Checking task ..", self.profile);
                    self.check_task(&task).await?;

                    info!("({}) <biglike.org> Task completed!", self.profile);
                    self.task.lock().await.complete_task();
                    *limit -= 1;
                },

                Some(_) => continue,

                None => return Err(Error::NoMoreTasks.into())
            }
        }

        Ok(())
    }

    /// Farming VK friends
    pub async fn vk_farm_friends(&mut self, min_price: usize, limit: &mut usize) -> Result<()> {
        // open page with tasks:
        self.open_tasks("vkfriend").await?;
        
        while *limit > 0 && !self.task.lock().await.to_close() {
            info!("({}) <biglike.org> Searching task..", self.profile);
            
            // searching next task:
            match self.next_task().await? {
                Some(task) if task.price >= min_price => {
                    // starting task:
                    info!("({}) <biglike.org> Starting task ({}) ..", self.profile, task.url);
                    self.vk_delete_friend(&task.url).await?;
                    self.start_task(&task).await?;

                    // executing task:
                    if !self.vk_add_friend(&task.url).await? {
                        info!("({}) <biglike.org> Task is broken, removing ..", self.profile);
                        self.remove_task(&task).await?;

                        continue;
                    }

                    // checking task:
                    info!("({}) <biglike.org> Checking task ..", self.profile);
                    self.check_task(&task).await?;

                    info!("({}) <biglike.org> Task completed!", self.profile);
                    self.task.lock().await.complete_task();
                    *limit -= 1;
                },

                Some(_) => continue,

                None => return Err(Error::NoMoreTasks.into())
            }
        }

        Ok(())
    }

    /// Farming VK subscribes
    pub async fn vk_farm_subscribes(&mut self, min_price: usize, limit: &mut usize) -> Result<()> {
        // open page with tasks:
        self.open_tasks("vkgroup").await?;
        
        while *limit > 0 && !self.task.lock().await.to_close() {
            info!("({}) <biglike.org> Searching task..", self.profile);
            
            // searching next task:
            match self.next_task().await? {
                Some(task) if task.price >= min_price => {
                    // starting task:
                    info!("({}) <biglike.org> Starting task ({}) ..", self.profile, task.url);
                    self.vk_unsubscribe(&task.url).await?;
                    self.start_task(&task).await?;

                    // executing task:
                    if !self.vk_subscribe(&task.url).await? {
                        info!("({}) <biglike.org> Task is broken, removing ..", self.profile);
                        self.remove_task(&task).await?;

                        continue;
                    }

                    // checking task:
                    info!("({}) <biglike.org> Checking task ..", self.profile);
                    self.check_task(&task).await?;

                    info!("({}) <biglike.org> Task completed!", self.profile);
                    self.task.lock().await.complete_task();
                    *limit -= 1;
                },

                Some(_) => continue,

                None => return Err(Error::NoMoreTasks.into())
            }
        }

        Ok(())
    }

    /// Close profile session
    pub async fn close(&mut self) -> Result<()> {
        self.task.lock().await.close();
        
        Ok(())
    }
}
