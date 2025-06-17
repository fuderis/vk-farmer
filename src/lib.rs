pub mod error;      pub use error::{ Result, Error };
pub mod prelude;

pub mod config;     pub use config::{ Config, Profile, Settings, FarmSettings };
pub mod farmer;     pub use farmer::{ Farmer, FreeLikes, BigLike };

pub mod manager;    pub use manager::{ Manager, Task };
