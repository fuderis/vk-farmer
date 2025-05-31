use macron::{ Display, From, Error };

// The result alias
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Display, Error, From)]
pub struct Error {
    pub kind: ErrorKind
}

// The error
#[derive(Debug, Display, Eq, PartialEq)]
pub enum ErrorKind {
    #[display = "There are no more tasks of this type"]
    NoMoreTasks,
}
