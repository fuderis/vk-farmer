use macron::{ Display, From, Error };

// The result alias
pub type StdResult<T, E> = std::result::Result<T, E>;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static>>;

// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    #[display = "{0}"]
    String(String),
    
    #[display = "There are no more tasks of this type"]
    NoMoreTasks,

    #[display = "Invalid bot name id"]
    InvalidBotNameID,

    #[display = "The bot's name is already used"]
    BotNameIDIsBusy
}
