use std::path::PathBuf;
use crate::engine::KvsEngine;
use crate::errors::Result;
use crate::KvsError;

pub struct SledKvsEngine{
    db:sled::Db
}

impl SledKvsEngine {
    pub fn open(path:impl Into<PathBuf>)->Result<Self>{
        Ok(SledKvsEngine{
            db: sled::open(path.into())?
        })
    }
}

impl KvsEngine for SledKvsEngine {
    fn set(&mut self, key: String, value: String) -> Result<()> {
        self.db.insert(key,value.into_bytes())?;
        self.db.flush()?;
        Ok(())
    }

    fn get(&mut self, key: String) -> Result<Option<String>> {
        if let Some(v) = self.db.get(key)?.map(|v| v.to_vec()) {
            return Ok(Some(String::from_utf8(v)?));
        }
        Ok(None)
    }

    fn remove(&mut self, key: String) -> Result<()> {
        self.db.remove(key)?.ok_or(KvsError::KeyNotFound)?;
        self.db.flush()?;
        Ok(())
    }
}