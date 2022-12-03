use crate::errors::Result;

pub mod kv;
pub mod command;
pub mod sled;

pub trait KvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()>;
    fn get(&mut self, key: String) -> Result<Option<String>>;
    fn remove(&mut self, key: String) -> Result<()>;
}

#[derive(Debug)]
pub enum EngineType{
    Default,
    Sled
}

impl EngineType {

    pub fn default_type()->Self{
        EngineType::Default
    }

    pub fn to_string(&self)->String{
        match self {
            EngineType::Default=>String::from("kvs"),
            EngineType::Sled=>String::from("sled")
        }
    }
}