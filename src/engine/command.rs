use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Command{
    SET(String,String),
    REMOVE(String),
}