use std::collections::HashMap;

/// The `KvStore` stores string key/value pairs.
///
/// Key/value pairs are stored in a `HashMap` in memory and not persisted to disk.
///
/// Example:
///
/// ```rust
/// # use kvs::KvStore;
/// let mut store = KvStore::new();
/// store.set("key".to_owned(), "value".to_owned());
/// let val = store.get("key".to_owned());
/// assert_eq!(val, Some("value".to_owned()));
/// ```
#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new()->KvStore{
        KvStore{
            map:HashMap::new(),
        }
    }

    pub fn set(&mut self,key:String,value:String){
        self.map.insert(key,value);
    }

    pub fn get(&self,key:String)->Option<String>{
        self.map.get(&key).cloned()
    }

    pub fn remove(&mut self, key:String){
        self.map.remove(&key);
    }

}
