use std::string::FromUtf8Error;
use std::{io, result};
use failure::Fail;
use serde_json;
use sled;
use thiserror::Error;

pub type Result<T>=result::Result<T,KvsError>;

#[derive(Error,Debug)]
pub enum KvsError{

    #[error("{0}")]
    Io(#[from]io::Error),
    
    #[error("{0}")]
    Serde(#[from]serde_json::Error),
    
    #[error("{0}")]
    Sled(#[from]sled::Error),

    #[error("{0}")]
    FromUTF8Err(#[from]FromUtf8Error),

    #[error("{0}")]
    StrErr(String),

    #[error("Key not found")]
    KeyNotFound,
    
    #[error("Unknown command type")]
    UnexpectedCommandType,

    #[error("Unknown engine type")]
    EngineErr

}
