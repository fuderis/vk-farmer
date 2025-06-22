use crate::prelude::*;

use std::fs;

/// The bot subconfig
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FarmSettings {
    pub likes_min_price: usize,
    pub friends_min_price: usize,
    pub subscribes_min_price: usize,
}

impl ::std::default::Default for FarmSettings {
    fn default() -> Self {
        Self {
            likes_min_price: 5,
            friends_min_price: 5,
            subscribes_min_price: 5,
        }
    }
}

/// The user profile config
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub vk_id: String,

    pub farm_likes: bool,
    pub likes_limit: usize,

    pub farm_friends: bool,
    pub friends_limit: usize,

    pub farm_subscribes: bool,
    pub subscribes_limit: usize,
}

impl ::std::default::Default for Profile {
    fn default() -> Self {
        Self {
            name: "".into(),
            vk_id: "".into(),

            farm_likes: true,
            likes_limit: 100,

            farm_friends: true,
            friends_limit: 20,

            farm_subscribes: true,
            subscribes_limit: 20,
        }
    }
}



/// The default farm settings
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Defaults {
    pub farm_likes: bool,
    pub likes_limit: usize,

    pub farm_friends: bool,
    pub friends_limit: usize,

    pub farm_subscribes: bool,
    pub subscribes_limit: usize,
}


/// The farm settings config
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    pub defaults: Defaults,
    pub freelikes: FarmSettings,
    pub biglike: FarmSettings,
    pub pause_delay: usize,
}

impl ::std::default::Default for Settings {
    fn default() -> Self {
        Self {
            defaults: Defaults {
                farm_likes: true,
                likes_limit: 100,

                farm_friends: true,
                friends_limit: 20,

                farm_subscribes: true,
                subscribes_limit: 20,
            },
            
            freelikes: FarmSettings {
                likes_min_price: 3,
                subscribes_min_price: 5,
                friends_min_price: 5,
            },

            biglike: FarmSettings {
                likes_min_price: 10,
                subscribes_min_price: 10,
                friends_min_price: 10,
            },

            pause_delay: 10,
        }
    }
}


/// The bot config
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(default, skip_serializing, skip_deserializing)]
    path: PathBuf,

    pub profiles: HashMap<String, Profile>,
    pub settings: Settings,
}

impl Config {
    /// Reads/writes config file
    pub fn read_or_write<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = crate::root_path(path)?;
        
        // reading config file:
        let config = if path.exists() {
            Config::read(path)?
        }
        // or writing default config file:
        else {
            let mut cfg = Config::default();
            cfg.save_to(path)?;

            cfg
        };

        Ok(config)
    }
    
    /// Reads config from file
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = crate::root_path(path)?;

        // read file:
        let json_str = fs::read_to_string(&path)?;

        let mut cfg: Config = serde_json::from_str(&json_str)?;
        cfg.path = path;

        // parse json:
        Ok(cfg)
    }
    
    /// Updates a config file
    pub fn save(&mut self) -> Result<()> {
        self.save_to(&self.path.clone())
    }

    /// Saves config to file
    pub fn save_to<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        self.path = crate::root_path(path)?;
        
        // to json string:
        let json_str = serde_json::to_string_pretty(self)?;

        // create dir:
        if let Some(dir) = self.path.parent() {
            fs::create_dir_all(dir)?;
        }
        
        // write file:
        fs::write(&self.path, json_str)?;
        
        Ok(())
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            path: "config.json".into(),
            profiles: HashMap::new(),
            settings: Settings::default()
        }
    }
}
