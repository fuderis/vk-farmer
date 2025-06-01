use crate::prelude::*;

/// The bot subconfig
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SubConfig {
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

/// The bot config
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub profile: String,
    pub port: String,
    
    pub freelikes: SubConfig,
    pub biglike: SubConfig,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            profile: "Bot1".into(),
            port: "54477".into(),
            
            freelikes: SubConfig {
                farm_likes: true,
                likes_limit: 50,
                likes_min_price: 3,

                farm_subscribes: true,
                subscribes_limit: 10,
                subscribes_min_price: 5,

                farm_friends: true,
                friends_limit: 10,
                friends_min_price: 5,
            },

            biglike: SubConfig {
                farm_likes: true,
                likes_limit: 50,
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
