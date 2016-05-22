use std::{io, error, fmt};
use git2;
use rustc_serialize::json;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Git(git2::Error),
    JsonDecode(json::DecoderError),
    FolderNotEmpty(String, usize),
    InvalidPublicKey(String),
    LoadProjectError,
    DuplicateUser(String, String),
    TodoErr,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(_) => "IO error occurred",
            Error::Git(_) => "Libgit2 error occurred",
            Error::JsonDecode(_) => "Json decoding error occurred",
            Error::FolderNotEmpty(_, _) => "Root folder was not empty",
            Error::InvalidPublicKey(_) => "Invalid public key",
            Error::LoadProjectError => "Loading project failed",
            Error::DuplicateUser(_, _) => "User already exists",
            Error::TodoErr => "Todo",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
           Error::Io(ref err) => Some(err),
           Error::Git(ref err) => Some(err),
           Error::JsonDecode(ref err) => Some(err),
           Error::FolderNotEmpty(_, _) => None,
           Error::InvalidPublicKey(_) => None,
           Error::LoadProjectError => None,
           Error::DuplicateUser(_, _) => None,
           Error::TodoErr => None,
       }
   }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f,
                "IO error occurred: {}", error::Error::description(err)),
            Error::Git(ref err) => write!(f,
                "Libgit2 error occurred: {}", error::Error::description(err)),
            Error::JsonDecode(ref err) => write!(f,
                "Json decoding error occurred: {}", error::Error::description(err)),
            Error::FolderNotEmpty(ref root, count) => write!(f,
                "{} was not empty: {} files exist", root, count),
            Error::InvalidPublicKey(ref key) => write!(f, 
                "Public key '{}' is invalid", key),
            Error::LoadProjectError => write!(f, "Loading project failed"),
            Error::DuplicateUser(ref key, ref user) => write!(f,
                "User '{}' with key '{}' already exists", user, key),
            Error::TodoErr => write!(f, "TodoErr"),
        }
    }
}
