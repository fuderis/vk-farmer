pub mod error;      pub use error::{ Result, Error };
pub mod prelude;

pub mod config;     pub use config::{ Config, Profile, Settings, FarmSettings };
pub mod farmer;     pub use farmer::{ Farmer, FreeLikes, BigLike };

pub mod manager;    pub use manager::{ Manager, Task };

pub mod templates;


/// Generates an unique ID
pub fn uniq_id() -> String {
    use std::time::{ SystemTime, UNIX_EPOCH };
    
    let millis = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    let random: u16 = rand::random();
    format!("{}{:04x}", millis, random)
}

/// Generates dir path by program root path 
pub fn root_path<P: AsRef<std::path::Path>>(relative_path: P) -> Result<std::path::PathBuf> {
    let exe_path = std::env::current_exe()?;
    let exe_dir = exe_path.parent().ok_or_else(|| Error::InvalidRootPath)?;

    // converting path to string:
    let rel_str = relative_path.as_ref().to_str().ok_or_else(|| Error::InvalidPath)?;
    
    // removing start symbol '/' if it's exists
    let rel_str = if rel_str.starts_with('/') {
        &rel_str[1..]
    } else {
        rel_str
    };

    Ok(exe_dir.join(rel_str))
}
