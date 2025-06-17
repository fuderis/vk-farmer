use crate::{ prelude::*, Profile };

use chromedriver_api::{ Session, Tab };
use tokio::time::{ sleep, Duration };

/// Social 'VKontakte' controller
#[derive(Debug)]
pub struct VKontakte {
    pub id: String,
    tab: Arc<Mutex<Tab>>
}

impl VKontakte {
    /// Login into VK account
    pub async fn login(session: &mut Session, profile: &Profile) -> Result<Self> {
        let tab = session.open("https://vk.com/").await.map_err(|e| Error::from(fmt!("{e}")))?;
        sleep(Duration::from_secs(3)).await;

        // waiting login to account:
        while tab.lock().await.inject(r#"return window.location.pathname != '/feed'"#).await.map_err(|e| Error::from(fmt!("{e}")))?.to_string() == "true" {
            println!("[INFO] ({}) <vk.com> Waiting for login to account ..", &profile.name);
            sleep(Duration::from_secs(5)).await;
        }
        println!("[INFO] ({}) <vk.com> Session is ready.", &profile.name);
        
        Ok(Self {
            id: profile.vk_id.clone(),
            tab
        })
    }

    /// Get account status text
    pub async fn get_status(&mut self) -> Result<String> {
        let mut tab = self.tab.lock().await;
        
        // open page:
        if tab.inject("return window.location.href == 'https://vk.com/edit';").await.map_err(|e| Error::from(fmt!("{e}")))?.to_string() == "false" {
            tab.open("https://vk.com/edit").await.map_err(|e| Error::from(fmt!("{e}")))?;
        }

        let log = tab.inject(r#"
            let _text_area_ = document.querySelector('textarea#pedit_general_short_information');

            if (_text_area_) {
                return _text_area_.value;
            }
            
            return "";
        "#).await.map_err(|e| Error::from(fmt!("{e}")))?;
        sleep(Duration::from_secs(1)).await;

        let text = log.to_string();
        
        if text.len() > 2 {
            Ok(text[1..text.len()-1].to_owned())
        } else {
            Ok(String::new())
        }
    }
    
    /// Set account status text
    pub async fn set_status(&mut self, text: &str) -> Result<bool> {
        let mut tab = self.tab.lock().await;
        
        // open page:
        if tab.inject("return window.location.href == 'https://vk.com/edit';").await.map_err(|e| Error::from(fmt!("{e}")))?.to_string() == "false" {
            tab.open("https://vk.com/edit").await.map_err(|e| Error::from(fmt!("{e}")))?;
        }

        let log = tab.inject(&("".to_string() + r#"
            let _text_area_ = document.querySelector('textarea#pedit_general_short_information');
            let _btn_save_ = document.querySelector('button[onclick="ProfileEditor.saveGeneral(this)"]');
            

            if (_text_area_ && _btn_save_) {
                _text_area_.value = '"# + text + r#"';
                
                _btn_save_.focus();
                _btn_save_.click();

                return true;
            }
            
            return false;
        "#)).await.map_err(|e| Error::from(fmt!("{e}")))?;
        sleep(Duration::from_secs(1)).await;

        Ok(log.to_string() == "true")
    }

    /// Set like to post or photo
    pub async fn like(&mut self, url: &str) -> Result<bool> {
        let mut tab = self.tab.lock().await;
        
        // open page:
        if tab.inject(&fmt!("return window.location.href == '{url}';")).await.map_err(|e| Error::from(fmt!("{e}")))?.to_string() == "false" {
            tab.open(url).await.map_err(|e| Error::from(fmt!("{e}")))?;
        }

        let log = tab.inject(r#"
            let _path_ = window.location.pathname;

            // post:
            if (_path_.startsWith('/wall')) {
                let _btn_like_ = document.querySelector('*[data-section-ref="reactions-button"]');

                if (_btn_like_ && !_btn_like_.classList.contains('PostButtonReactions--active')) {
                    _btn_like_.focus();
                    _btn_like_.click();

                    return true;
                }
            }

            // video:
            else if (_path_.startsWith('/video')) {
                let _btn_like_ = document.querySelector('*[data-testid="video_modal_like_button"]');

                if (_btn_like_ && !_btn_like_.getAttribute('style').match('--vkui--color_accent_red')) {
                    _btn_like_.focus();
                    _btn_like_.click();

                    return true;
                }
            }

            // photo:
            else {
                let _btn_like_ = document.querySelector('.like_btn');

                if (_btn_like_ && !_btn_like_.classList.contains('active')) {
                    _btn_like_.focus();
                    _btn_like_.click();

                    return true;
                }
            }

            return false;
        "#).await.map_err(|e| Error::from(fmt!("{e}")))?;
        sleep(Duration::from_secs(1)).await;

        Ok(log.to_string() == "true")
    }

    /// Unset like to post or photo
    pub async fn unlike(&mut self, url: &str) -> Result<bool> {
        let mut tab = self.tab.lock().await;
        
        // open page:
        if tab.inject(&fmt!("return window.location.href == '{url}';")).await.map_err(|e| Error::from(fmt!("{e}")))?.to_string() == "false" {
            tab.open(url).await.map_err(|e| Error::from(fmt!("{e}")))?;
        }

        let log = tab.inject(r#"
            let _path_ = window.location.pathname;

            // post:
            if (_path_.startsWith('/wall')) {
                let _btn_like_ = document.querySelector('*[data-section-ref="reactions-button"]');

                if (_btn_like_ && _btn_like_.classList.contains('PostButtonReactions--active')) {
                    _btn_like_.focus();
                    _btn_like_.click();

                    return true;
                }
            }

            // video:
            else if (_path_.startsWith('/video')) {
                let _btn_like_ = document.querySelector('*[data-testid="video_modal_like_button"]');

                if (_btn_like_ && _btn_like_.getAttribute('style').match('--vkui--color_accent_red')) {
                    _btn_like_.focus();
                    _btn_like_.click();

                    return true;
                }
            }

            // photo:
            else {
                let _btn_like_ = document.querySelector('.like_btn');

                if (_btn_like_ && _btn_like_.classList.contains('active')) {
                    _btn_like_.focus();
                    _btn_like_.click();
                    
                    return true;
                }
            }

            return false;
        "#).await.map_err(|e| Error::from(fmt!("{e}")))?;
        sleep(Duration::from_secs(1)).await;

        Ok(log.to_string() == "true")
    }

    /// Subscribe to channel
    pub async fn subscribe(&mut self, url: &str) -> Result<bool> {
        let mut tab = self.tab.lock().await;
        
        // open page:
        if tab.inject(&fmt!("return window.location.href == '{url}';")).await.map_err(|e| Error::from(fmt!("{e}")))?.to_string() == "false" {
            tab.open(url).await.map_err(|e| Error::from(fmt!("{e}")))?;
        }

        let log = tab.inject(r#"
            let _btn_sub_ = document.querySelector('#public_subscribe');

            if (_btn_sub_) {
                _btn_sub_.focus();
                _btn_sub_.click();

                return true;
            } else {
                let _btn_sub_ = document.querySelector('#join_button');

                if (_btn_sub_) {
                    _btn_sub_.focus();
                    _btn_sub_.click();

                    return true;
                }
            }

            return false;
        "#).await.map_err(|e| Error::from(fmt!("{e}")))?;
        sleep(Duration::from_secs(1)).await;

        Ok(log.to_string() == "true")
    }

    /// Unsubscribe from channel
    pub async fn unsubscribe(&mut self, url: &str) -> Result<bool> {
        let mut tab = self.tab.lock().await;
        
        // open page:
        if tab.inject(&fmt!("return window.location.href == '{url}';")).await.map_err(|e| Error::from(fmt!("{e}")))?.to_string() == "false" {
            tab.open(url).await.map_err(|e| Error::from(fmt!("{e}")))?;
        }

        let log = tab.inject(r#"
            let _sub_panel_ = document.querySelector('#page_subscribers > div[onmouseover]');

            if (_sub_panel_) {
                let _hover_ev_ = document.createEvent("MouseEvent");
                _hover_ev_.initMouseEvent("mouseover", true, true, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
                                            
                _sub_panel_.dispatchEvent(_hover_ev_);

                setTimeout(function() {
                    let _btn_unsub_ = _sub_panel_.querySelector('*[onkeydown]');

                    if (_btn_unsub_) {
                        _btn_unsub_.focus();
                        _btn_unsub_.click();

                        return true;
                    }
                }, 500);
            }

            return false;
        "#).await.map_err(|e| Error::from(fmt!("{e}")))?;
        sleep(Duration::from_secs(1)).await;

        Ok(log.to_string() == "true")
    }

    /// Add user to friends
    pub async fn add_friend(&mut self, url: &str) -> Result<bool> {
        let mut tab = self.tab.lock().await;
        
        // open page:
        if tab.inject(&fmt!("return window.location.href == '{url}';")).await.map_err(|e| Error::from(fmt!("{e}")))?.to_string() == "false" {
            tab.open(url).await.map_err(|e| Error::from(fmt!("{e}")))?;
        }

        let log = tab.inject(r#"
            let _btn_add_friend_ = document.querySelector('.ProfileHeaderButton > button[aria-expanded]');
             
            if (_btn_add_friend_ && _btn_add_friend_.querySelector('.vkuiButton__content') && _btn_add_friend_.querySelector('.vkuiButton__content').textContent.match('Подписан')) {
                let _hover_ev_ = document.createEvent("MouseEvent");
                _hover_ev_.initMouseEvent("mouseover", true, true, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
                                            
                _btn_add_friend_.dispatchEvent(_hover_ev_);

                setTimeout(function() {
                    let _btn_add_friend_ = document.querySelector('*[data-testid="dropdownactionsheet-item"]');

                    if (_btn_add_friend_) {
                        _btn_add_friend_.focus();
                        _btn_add_friend_.click();

                        return true;
                    }
                }, 500);
            } else {
                let _btn_add_friend_ = document.querySelector('.ProfileHeaderButton > button');
            
                if (_btn_add_friend_ && !_btn_add_friend_.hasAttribute('aria-expanded')) {
                    _btn_add_friend_.focus();
                    _btn_add_friend_.click();

                    return true;
                }
            }

            return false;
        "#).await.map_err(|e| Error::from(fmt!("{e}")))?;
        sleep(Duration::from_secs(1)).await;

        Ok(log.to_string() == "true")
    }

    /// Delete user from friends
    pub async fn delete_friend(&mut self, url: &str) -> Result<bool> {
        let mut tab = self.tab.lock().await;
        
        // open page:
        if tab.inject(&fmt!("return window.location.href == '{url}';")).await.map_err(|e| Error::from(fmt!("{e}")))?.to_string() == "false" {
            tab.open(url).await.map_err(|e| Error::from(fmt!("{e}")))?;
        }

        let log = tab.inject(r#"
            let _btn_add_friend_ = document.querySelector('.ProfileHeaderButton > button[aria-label="Друзья"]');

            if (_btn_add_friend_) {
                let _hover_ev_ = document.createEvent("MouseEvent");
                _hover_ev_.initMouseEvent("mouseover", true, true, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
                                            
                _btn_add_friend_.dispatchEvent(_hover_ev_);

                setTimeout(function() {
                    let _btn_del_friend_ = document.querySelector('*[data-testid="dropdownactionsheet-item"]');

                    if (_btn_del_friend_) {
                        _btn_del_friend_.focus();
                        _btn_del_friend_.click();

                        return true;
                    }
                }, 500);
            } else {
                let _btn_add_friend_ = document.querySelector('.ProfileHeaderButton > button[aria-expanded]');

                if (_btn_add_friend_ && _btn_add_friend_.querySelector('.vkuiButton__content') && (_btn_add_friend_.querySelector('.vkuiButton__content').textContent.match('Вы подписаны') || _btn_add_friend_.querySelector('.vkuiButton__content').textContent.match('Заявка отправлена'))) {
                    let _hover_ev_ = document.createEvent("MouseEvent");
                    _hover_ev_.initMouseEvent("mouseover", true, true, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
                                                
                    _btn_add_friend_.dispatchEvent(_hover_ev_);

                    setTimeout(function() {
                        let _btn_del_friend_ = document.querySelector('*[data-testid="dropdownactionsheet-item"]');

                        if (_btn_del_friend_) {
                            _btn_del_friend_.focus();
                            _btn_del_friend_.click();

                            return true;
                        }
                    }, 500);
                }
            }

            return false;
        "#).await.map_err(|e| Error::from(fmt!("{e}")))?;
        sleep(Duration::from_secs(1)).await;

        Ok(log.to_string() == "true")
    }

    /// Close VKontakte tab
    pub async fn close(self) -> Result<()> {
        sleep(Duration::from_millis(100)).await;
        self.tab.lock().await.close().await.map_err(|e| Error::from(fmt!("{e}")))?;

        Ok(())
    }
}
