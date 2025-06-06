use crate::prelude::*;
use super::Task;

use chromedriver_api::{ Session, Tab };
use tokio::time::{ sleep, Duration };
use serde_json::Value;

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

    /// Open tasks page
    async fn open_tasks(&mut self, path: &str) -> Result<()> {
        let mut work_tab = self.work_tab.lock().await;
        
        // open page:
        work_tab.open(&fmt!("https://freelikes.online/earn/{path}")).await?;

        // disabling pop-ups:
        work_tab.inject(r#"
            window.open = function(_url, _name, _specs) {
                return null;
            };
        "#).await?;

        Ok(())
    }

    /// Search next task
    async fn next_task(&mut self) -> Result<Option<Task>> {
        let mut work_tab = self.work_tab.lock().await;

        // searching next task:
        let log = work_tab.inject(&("return ".to_owned() + r#"(() => {
            let tasks = document.querySelectorAll('button.do-task');

            if (tasks) {
                let task = Array.from(tasks).find(el => !el.hasAttribute('task-id') && el.querySelector('.insidebtn[onclick]'));

                if (task) {
                    let id = 'id-' + Date.now().toString(36);
                    task.setAttribute('task-id', id);

                    let button_elems = task.querySelectorAll('.insidebtn[onclick]');
                    let buttons = Array.from(button_elems);

                    if (buttons.length == 2) {
                        let button_start = buttons[0];
                        button_start.setAttribute('start-task', '');

                        let button_check = buttons[1];
                        button_check.setAttribute('check-task', '');

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
    async fn start_task(&mut self, task: &Task, script: Option<&str>) -> Result<Option<Value>> {
        let mut work_tab = self.work_tab.lock().await;
        let mut task_tab = self.task_tab.lock().await;

        // open task page:
        task_tab.open(&task.url).await?;
        sleep(Duration::from_secs(1)).await;
        
        // inject script to task page:
        let log = if let Some(script) = script {
            let log = task_tab.inject(script).await?;
            sleep(Duration::from_secs(2)).await;

            Some(log)
        } else {
            None
        };

        // click to 'start-task' button:
        let _ = work_tab.inject(&("return ".to_owned() + r#"(() => {
            var button_start = document.querySelector('button[task-id="# + &task.id + r#"] *[start-task]');

            if (button_start) {
                button_start.focus();
                button_start.click();
            }
        })();"#)).await?;
        sleep(Duration::from_secs(1)).await;

        Ok(log)
    }

    /// Inject script to process task
    async fn do_task(&mut self, script: &str) -> Result<Value> {
        let mut task_tab = self.task_tab.lock().await;
        
        let log = task_tab.inject(script).await?;
        sleep(Duration::from_secs(2)).await;

        Ok(log)
    }

    /// Check task
    async fn check_task(&mut self, task: &Task) -> Result<()> {
        let mut work_tab = self.work_tab.lock().await;

        // click to 'check task' button
        work_tab.inject(&("return ".to_owned() + r#"(() => {
            var button_check = document.querySelector('button[task-id="# + &task.id + r#"] *[check-task]');

            if (button_check) {
                button_check.focus();
                button_check.click();
            }
        })();"#)).await?;
        sleep(Duration::from_secs(1)).await;

        Ok(())
    }

    // ________________ VK.COM ________________

    /// Farming VK likes
    pub async fn vk_farm_likes(&mut self, min_price: usize, mut limit: usize) -> Result<()> {
        // open page with tasks:
        self.open_tasks("vkontakte/vklike").await?;
        
        while limit > 0 {
            println!("[INFO] ({}) <freelikes.online> Searching task..", self.profile);
            
            // searching next task:
            match self.next_task().await? {
                Some(task) if task.price >= min_price => {
                    limit -= 1;

                    // starting task:
                    println!("[INFO] ({}) <freelikes.online> Starting task ({}) ..", self.profile, task.url);
                    self.start_task(&task, Some(&("".to_owned() + r#"
                        let path = window.location.pathname;

                        // post:
                        if (path.startsWith('/wall')) {
                            let button_like = document.querySelector('*[data-section-ref="reactions-button"]');

                            if (button_like && button_like.classList.contains('PostButtonReactions--active')) {
                                button_like.focus();
                                button_like.click();
                            }
                        }

                        // video:
                        else if (path.startsWith('/video')) {
                            let button_like = document.querySelector('*[data-testid="video_modal_like_button"]');

                            if (button_like && button_like.getAttribute('style').match('--vkui--color_accent_red')) {
                                button_like.focus();
                                button_like.click();
                            }
                        }

                        // photo:
                        else {
                            let button_like = document.querySelector('.like_btn');

                            if (button_like && button_like.classList.contains('active')) {
                                button_like.focus();
                                button_like.click();
                            }
                        }
                    "#))).await?;

                    // injecting task script:
                    let _ = self.do_task(&("".to_owned() + r#"
                        let path = window.location.pathname;

                        // post:
                        if (path.startsWith('/wall')) {
                            let button_like = document.querySelector('*[data-section-ref="reactions-button"]');

                            if (button_like && !button_like.classList.contains('PostButtonReactions--active')) {
                                button_like.focus();
                                button_like.click();
                            }
                        }

                        // video:
                        else if (path.startsWith('/video')) {
                            let button_like = document.querySelector('*[data-testid="video_modal_like_button"]');

                            if (button_like && !button_like.getAttribute('style').match('--vkui--color_accent_red')) {
                                button_like.focus();
                                button_like.click();
                            }
                        }

                        // photo:
                        else {
                            let button_like = document.querySelector('.like_btn');

                            if (button_like && !button_like.classList.contains('active')) {
                                button_like.focus();
                                button_like.click();
                            }
                        }
                    "#)).await?;

                    // checking task:
                    println!("[INFO] ({}) <freelikes.online> Checking task ..", self.profile);
                    self.check_task(&task).await?;

                    println!("[INFO] ({}) <freelikes.online> Task completed!", self.profile);
                },

                Some(_) => continue,

                None => return Err(Error::NoMoreTasks.into())
            }
        }

        Ok(())
    }

    /// Farming VK friends
    pub async fn vk_farm_friends(&mut self, min_price: usize, mut limit: usize) -> Result<()> {
        // open page with tasks:
        self.open_tasks("vkontakte/vkfriend").await?;
        
        while limit > 0 {
            println!("[INFO] ({}) <freelikes.online> Searching task..", self.profile);
            
            // searching next task:
            match self.next_task().await? {
                Some(task) if task.price >= min_price => {
                    limit -= 1;

                    // starting task:
                    println!("[INFO] ({}) <freelikes.online> Starting task ({}) ..", self.profile, task.url);
                    self.start_task(&task, Some(&("".to_owned() + r#"(() => {
                        let button_subscribe = document.querySelector('.ProfileHeaderButton > button[aria-label="Друзья"]');

                        if (button_subscribe) {
                            const hover_event = document.createEvent("MouseEvent");
                            hover_event.initMouseEvent("mouseover", true, true, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
                                                        
                            button_subscribe.dispatchEvent(hover_event);

                            setTimeout(function() {
                                let button_unsubscribe = document.querySelector('*[data-testid="dropdownactionsheet-item"]');

                                if (button_unsubscribe) {
                                    button_unsubscribe.focus();
                                    button_unsubscribe.click();
                                }
                            }, 500);
                        } else {
                            let button_subscribe = document.querySelector('.ProfileHeaderButton > button[aria-expanded]');

                            if (button_subscribe && button_subscribe.querySelector('.vkuiButton__content') && (button_subscribe.querySelector('.vkuiButton__content').textContent.match('Вы подписаны') || button_subscribe.querySelector('.vkuiButton__content').textContent.match('Заявка отправлена'))) {
                                const hover_event = document.createEvent("MouseEvent");
                                hover_event.initMouseEvent("mouseover", true, true, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
                                                            
                                button_subscribe.dispatchEvent(hover_event);

                                setTimeout(function() {
                                    let button_unsubscribe = document.querySelector('*[data-testid="dropdownactionsheet-item"]');

                                    if (button_unsubscribe) {
                                        button_unsubscribe.focus();
                                        button_unsubscribe.click();
                                    }
                                }, 500);
                            }
                        }
                    })();"#))).await?;

                    // injecting task script:
                    let _ = self.do_task(&("".to_owned() + r#"(() => {
                        let button_subscribe = document.querySelector('.ProfileHeaderButton > button[aria-expanded]');
                        
                        if (button_subscribe && button_subscribe.querySelector('.vkuiButton__content') && button_subscribe.querySelector('.vkuiButton__content').textContent.match('Подписан')) {
                            const hover_event = document.createEvent("MouseEvent");
                            hover_event.initMouseEvent("mouseover", true, true, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
                                                        
                            button_subscribe.dispatchEvent(hover_event);

                            setTimeout(function() {
                                let button_subscribe = document.querySelector('*[data-testid="dropdownactionsheet-item"]');

                                if (button_subscribe) {
                                    button_subscribe.focus();
                                    button_subscribe.click();
                                }
                            }, 500);
                        } else {
                            let button_subscribe = document.querySelector('.ProfileHeaderButton > button');
                        
                            if (button_subscribe && !button_subscribe.hasAttribute('aria-expanded')) {
                                button_subscribe.focus();
                                button_subscribe.click();
                            }
                        }
                    })();"#)).await?;

                    // checking task:
                    println!("[INFO] ({}) <freelikes.online> Checking task ..", self.profile);
                    self.check_task(&task).await?;

                    println!("[INFO] ({}) <freelikes.online> Task completed!", self.profile);
                },

                Some(_) => continue,

                None => return Err(Error::NoMoreTasks.into())
            }
        }

        Ok(())
    }

    /// Farming VK subscribes
    pub async fn vk_farm_subscribes(&mut self, min_price: usize, mut limit: usize) -> Result<()> {
        // open page with tasks:
        self.open_tasks("vkontakte/vkgroup").await?;
        
        while limit > 0 {
            println!("[INFO] ({}) <freelikes.online> Searching task..", self.profile);
            
            // searching next task:
            match self.next_task().await? {
                Some(task) if task.price >= min_price => {
                    limit -= 1;

                    // starting task:
                    println!("[INFO] ({}) <freelikes.online> Starting task ({}) ..", self.profile, task.url);
                    self.start_task(&task, Some(&("".to_owned() + r#"(() => {
                        let subscriber_panel = document.querySelector('#page_subscribers > div[onmouseover]');

                        if (subscriber_panel) {
                            const hover_event = document.createEvent("MouseEvent");
                            hover_event.initMouseEvent("mouseover", true, true, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
                                                        
                            subscriber_panel.dispatchEvent(hover_event);

                            setTimeout(function() {
                                let button_unsubscribe = subscriber_panel.querySelector('*[onkeydown]');

                                if (button_unsubscribe) {
                                    button_unsubscribe.focus();
                                    button_unsubscribe.click();
                                }
                            }, 500);
                        }
                    })();"#))).await?;

                    // injecting task script:
                    let _ = self.do_task(&("".to_owned() + r#"(() => {
                        let subscribe_button = document.querySelector('#public_subscribe');

                        if (subscribe_button) {
                            subscribe_button.focus();
                            subscribe_button.click();
                        } else {
                            let subscribe_button = document.querySelector('#join_button');

                            if (subscribe_button) {
                                subscribe_button.focus();
                                subscribe_button.click();
                            }
                        }
                    })();"#)).await?;

                    // checking task:
                    println!("[INFO] ({}) <freelikes.online> Checking task ..", self.profile);
                    self.check_task(&task).await?;

                    println!("[INFO] ({}) <freelikes.online> Task completed!", self.profile);
                },

                Some(_) => continue,

                None => return Err(Error::NoMoreTasks.into())
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
