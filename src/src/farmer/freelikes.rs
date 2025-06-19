use crate::{ prelude::*, Task };
use super::{ FarmTask, VKontakte };

use chromedriver_api::{ Session, Tab };
use tokio::time::{ sleep, Duration };
use serde_json::Value;

// Farmer for 'freelikes.online'
#[derive(Debug)]
pub struct FreeLikes {
    task: Arc<Mutex<Task>>,
    
    pub(crate) profile: String,
    pub(crate) freelikes: Arc<Mutex<Tab>>,
    pub(crate) vkontakte: Arc<Mutex<VKontakte>>,
}

impl FreeLikes {
    /// Login to profile
    pub async fn login<S: Into<String>>(task: Arc<Mutex<Task>>, profile: S, session: &mut Session, vkontakte: Arc<Mutex<VKontakte>>) -> Result<Self> {
        let profile = profile.into();
        
        // open website:
        let freelikes = session.open("https://freelikes.online/vkontakte/vklike").await?;
        let mut freelikes_lock = freelikes.lock().await;
        
        let mut vk_lock = vkontakte.lock().await;

        // login to account:
        if freelikes_lock.inject(r#"return document.querySelector('button[onclick="open_login_win();"]')? true: false;"#).await?.to_string() == "true" {
            info!("({}) <freelikes.online> Login to account ..", &profile);

            let _ = freelikes_lock.inject(&("".to_string() + r#"
                let _inp_link_ = document.querySelector('#form_login input');
                _inp_link_.value = 'https://vk.com/id"# + &vk_lock.id[..] + r#"';

                let _btn_login_ = document.querySelector('#form_login button.btnenter');
                _btn_login_.focus();
                _btn_login_.click();
            "#)).await?;
            sleep(Duration::from_secs(1)).await;

            while freelikes_lock.inject(r#"return document.querySelector('button[onclick="open_login_win();"]')? true: false;"#).await?.to_string() == "true" {
                let log = freelikes_lock.inject(r#"
                    window.open = function(url, ...args) {
                        window.url_link = url;
                        return null;
                    };
                
                    let _btn_link_ = document.querySelector('div#form_like button[onclick="open_login_win();"]');

                    let _click_ev_ = document.createEvent("MouseEvent");
                    _click_ev_.initMouseEvent("click", true, true, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
                                                
                    _btn_link_.dispatchEvent(_click_ev_);

                    return window.url_link;
                "#).await?;

                let url = log.to_string().replace("'", "").replace("\"", "");
                if url.starts_with("https://") {
                    sleep(Duration::from_millis(500)).await;
                    vk_lock.like(&url).await?;
                } else {
                    continue;
                }

                let _ = freelikes_lock.inject(r#"
                    let _btn_check_ = document.querySelector('div#form_like button[onclick="chek_login_like()"]');

                    let _click_ev_ = document.createEvent("MouseEvent");
                    _click_ev_.initMouseEvent("click", true, true, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
                                                
                    _btn_check_.dispatchEvent(_click_ev_);
                "#).await?;

                sleep(Duration::from_secs(5)).await;
            }
        }

        info!("({}) <freelikes.online> Session is ready.", &profile);

        drop(freelikes_lock);
        drop(vk_lock);
        Ok(Self {
            task,
            
            profile,
            freelikes,
            vkontakte,
        })
    }

    /// Open tasks page
    async fn open_tasks(&mut self, path: &str) -> Result<()> {
        let mut freelikes = self.freelikes.lock().await;
        
        // open page:
        freelikes.open(&fmt!("https://freelikes.online/earn/{path}")).await.map_err(|e| {dbg!(&e); Error::from(fmt!("{e}"))})?;

        // disabling pop-ups:
        freelikes.inject(r#"
            window.open = function(_url, _name, _specs) {
                return null;
            };
        "#).await?;

        Ok(())
    }

    /// Search next task
    async fn next_task(&mut self) -> Result<Option<FarmTask>> {
        let mut freelikes = self.freelikes.lock().await;

        // searching next task:
        let log = freelikes.inject(&("return ".to_owned() + r#"(() => {
            let tasks = document.querySelectorAll('article.panel');

            if (tasks) {
                let task = Array.from(tasks).find(el => !el.hasAttribute('task-id') && el.querySelector('button .insidebtn[onclick]'));

                if (task) {
                    let id = 'id-' + Date.now().toString(36);
                    task.setAttribute('task-id', id);

                    let button_elems = task.querySelectorAll('button .insidebtn[onclick]');
                    let buttons = Array.from(button_elems);

                    if (buttons.length == 2) {
                        let button_start = buttons[0];
                        button_start.setAttribute('start-task', '');

                        let button_check = buttons[1];
                        button_check.setAttribute('check-task', '');

                        let button_remove = task.querySelector('a.delete');
                        button_remove.setAttribute('remove-task', '');

                        let price = Number(button_start.textContent.match(/(\d+)/)[1]);
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
        let mut freelikes = self.freelikes.lock().await;

        // click to 'start-task' button:
        let log = freelikes.inject(&("return ".to_owned() + r#"(() => {
            var button_start = document.querySelector('article.panel[task-id="# + &task.id + r#"] *[start-task]');

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
        let mut freelikes = self.freelikes.lock().await;

        // click to 'check task' button
        freelikes.inject(&("return ".to_owned() + r#"(() => {
            var button_check = document.querySelector('article.panel[task-id="# + &task.id + r#"] *[check-task]');

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
        let mut freelikes = self.freelikes.lock().await;

        // click to 'remove task' button
        freelikes.inject(&("return ".to_owned() + r#"(() => {
            var button_remove = document.querySelector('article.panel[task-id="# + &task.id + r#"] *[remove-task]');

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
        self.open_tasks("vkontakte/vklike").await?;
        
        while *limit > 0 && !self.task.lock().await.is_closed() {
            info!("({}) <freelikes.online> Searching task..", self.profile);
            
            // searching next task:
            match self.next_task().await? {
                Some(task) if task.price >= min_price => {
                    // starting task:
                    info!("({}) <freelikes.online> Starting task ({}) ..", self.profile, task.url);
                    self.vk_unlike(&task.url).await?;
                    self.start_task(&task).await?;

                    // executing task:
                    if !self.vk_like(&task.url).await? {
                        info!("({}) <freelikes.online> Task is broken, removing ..", self.profile);
                        self.remove_task(&task).await?;

                        continue;
                    }

                    // checking task:
                    info!("({}) <freelikes.online> Checking task ..", self.profile);
                    self.check_task(&task).await?;

                    info!("({}) <freelikes.online> Task completed!", self.profile);
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
        self.open_tasks("vkontakte/vkfriend").await?;
        
        while *limit > 0 && !self.task.lock().await.is_closed() {
            info!("({}) <freelikes.online> Searching task..", self.profile);
            
            // searching next task:
            match self.next_task().await? {
                Some(task) if task.price >= min_price => {
                    // starting task:
                    info!("({}) <freelikes.online> Starting task ({}) ..", self.profile, task.url);
                    self.vk_delete_friend(&task.url).await?;
                    self.start_task(&task).await?;

                    // executing task:
                    if !self.vk_add_friend(&task.url).await? {
                        info!("({}) <freelikes.online> Task is broken, removing ..", self.profile);
                        self.remove_task(&task).await?;

                        continue;
                    }

                    // checking task:
                    info!("({}) <freelikes.online> Checking task ..", self.profile);
                    self.check_task(&task).await?;

                    info!("({}) <freelikes.online> Task completed!", self.profile);
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
        self.open_tasks("vkontakte/vkgroup").await?;
        
        while *limit > 0 && !self.task.lock().await.is_closed() {
            info!("({}) <freelikes.online> Searching task..", self.profile);
            
            // searching next task:
            match self.next_task().await? {
                Some(task) if task.price >= min_price => {
                    // starting task:
                    info!("({}) <freelikes.online> Starting task ({}) ..", self.profile, task.url);
                    self.vk_unsubscribe(&task.url).await?;
                    self.start_task(&task).await?;

                    // executing task:
                    if !self.vk_subscribe(&task.url).await? {
                        info!("({}) <freelikes.online> Task is broken, removing ..", self.profile);
                        self.remove_task(&task).await?;

                        continue;
                    }

                    // checking task:
                    info!("({}) <freelikes.online> Checking task ..", self.profile);
                    self.check_task(&task).await?;

                    info!("({}) <freelikes.online> Task completed!", self.profile);
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
