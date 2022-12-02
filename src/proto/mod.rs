use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub enum Request{
    SET(String,String),
    GET(String),
    REMOVE(String)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response{
    Ok(Option<String>),
    Err(String)
}

