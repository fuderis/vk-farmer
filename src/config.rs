use crate::prelude::*;

use std::fs;

/// The bot subconfig
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FarmSettings {
    pub farm_likes: bool,
    pub likes_limit: usize,
    pub likes_min_price: usize,

    pub farm_subscribes: bool,
    pub subscribes_limit: usize,
    pub subscribes_min_price: usize,

    pub farm_friends: bool,
    pub friends_limit: usize,
    pub friends_min_price: usize,
}

impl ::std::default::Default for FarmSettings {
    fn default() -> Self {
        Self {
            farm_likes: true,
            likes_limit: 30,
            likes_min_price: 5,

            farm_subscribes: true,
            subscribes_limit: 10,
            subscribes_min_price: 5,

            farm_friends: true,
            friends_limit: 10,
            friends_min_price: 5,
        }
    }
}

/// The user profile config
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub port: String,
    pub profile: String,
}

impl ::std::default::Default for Profile {
    fn default() -> Self {
        Self {
            port: "54477".into(),
            profile: "Default".into(),
        }
    }    
}


/// The farm settings config
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub freelikes: FarmSettings,
    pub biglike: FarmSettings,
}

impl ::std::default::Default for Settings {
    fn default() -> Self {
        Self {
            freelikes: FarmSettings {
                farm_likes: true,
                likes_limit: 30,
                likes_min_price: 3,

                farm_subscribes: true,
                subscribes_limit: 10,
                subscribes_min_price: 5,

                farm_friends: true,
                friends_limit: 10,
                friends_min_price: 5,
            },

            biglike: FarmSettings {
                farm_likes: true,
                likes_limit: 30,
                likes_min_price: 10,

                farm_subscribes: true,
                subscribes_limit: 10,
                subscribes_min_price: 10,

                farm_friends: true,
                friends_limit: 10,
                friends_min_price: 10,
            },
        }
    }    
}


/// The bot config
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub profiles: Vec<Profile>,
    pub settings: Settings,
}

impl Config {
    /// Reads config from file or creates default
    pub fn read_or_save<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>
    {
        let config = if Path::new("config.json").exists() {
            Config::read("config.json")?
        } else {
            let cfg = Config::default();
            cfg.save(path)?;

            cfg
        };

        Ok(config)

    }
    
    /// Reads config from file
    pub fn read<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>
    {
        let path = path.as_ref();

        // read file:
        let json_str = fs::read_to_string(path)?;

        // parse json:
        Ok(serde_json::from_str(&json_str)?)
    }
    
    /// Saves config to file
    pub fn save<P>(&self, path: P) -> Result<()>
    where
        P: AsRef<Path>
    {
        let path = path.as_ref();
        
        // to json string:
        let json_str = serde_json::to_string_pretty(self)?;

        // create dir:
        if let Some(dir) = path.parent() {
            fs::create_dir_all(dir)?;
        }
        
        // write file:
        fs::write(path, json_str)?;
        
        Ok(())
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            profiles: vec![ Profile::default() ],
            settings: Settings::default()
        }
    }
}
