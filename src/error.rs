use macron::{ Display, From, Error };

// The result alias
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[display = "There are no more tasks of this type"]
    NoMoreTasks,
}
