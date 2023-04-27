#[derive(Debug)]
pub enum Error{
    MergeError,
    PushError,
}

impl std::error::Error for Error{}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Error::*;
        match self {
            MergeError => write!(f, "MergeError"),
            PushError => write!(f, "PushError"),
        }
    }
}
