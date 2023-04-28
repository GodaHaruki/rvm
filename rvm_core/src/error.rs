#[derive(Debug)]
pub enum Error{
    MergeError,
    PushError,
    BranchNotFound,
}

impl std::error::Error for Error{}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        use self::Error::*;
        match self {
            MergeError => write!(f, "MergeError"),
            PushError => write!(f, "PushError"),
            BranchNotFound => write!(f, "BranchNotFound")
            _ => todo!()
        }
    }
}
