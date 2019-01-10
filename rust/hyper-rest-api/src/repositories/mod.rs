use std::fmt::Debug;

pub mod inmemory;
pub mod mongo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Value {
    pub key: String,
    pub value: String,
}

// TODO: is Sync needed/correct here?
pub trait ValueRepository: Debug + Send + Sync + 'static {
    fn get(&self, key: String) -> Result<Value, &str>;
    fn put(&mut self, value: Value);
}
