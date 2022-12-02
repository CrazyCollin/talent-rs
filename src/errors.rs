use std::string::FromUtf8Error;
use std::{io, result};
use failure::Fail;
use serde_json;
use sled;

pub type Result<T>=result::Result<T,KvsError>;

#[derive(Fail,Debug)]
pub enum KvsError{
    #[fail(display = "{}", _0)]
    Io(io::Error),
    
    #[fail(display="{}",_0)]
    Serde(serde_json::Error),
    
    #[fail(display="{}",_0)]
    Sled(sled::Error),

    #[fail(display="{}",_0)]
    FromUTF8Err(FromUtf8Error),

    #[fail(display="{}",_0)]
    StrErr(String),

    #[fail(display = "Key not found")]
    KeyNotFound,
    
    #[fail(display = "Unknown command type")]
    UnexpectedCommandType,

    #[fail(display = "Unknown engine type")]
    EngineErr

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

impl From<sled::Error> for KvsError {
    fn from(err: sled::Error) -> KvsError {
        KvsError::Sled(err)
    }
}

impl From<FromUtf8Error> for KvsError {
    fn from(err: FromUtf8Error) -> Self {
        KvsError::FromUTF8Err(err)
    }
}