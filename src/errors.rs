use std::{io, result};
use failure::Fail;
use serde_json;

pub type Result<T>=result::Result<T,KvsError>;

#[derive(Fail,Debug)]
pub enum KvsError{
    #[fail(display = "{}", _0)]
    Io(io::Error),
    
    #[fail(display="{}",_0)]
    Serde(serde_json::Error),
    
    #[fail(display = "Key not found")]
    KeyNotFound,
    
    #[fail(display = "Unknown command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> KvsError {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> KvsError {
        KvsError::Serde(err)
    }
}