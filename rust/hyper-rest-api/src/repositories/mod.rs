pub mod inmemory;

#[derive(Serialize, Deserialize, Clone)]
pub struct Value {
    pub key: String,
    pub value: String,
}

pub trait ValueRepository: Send + 'static {
    fn get(&self, key: String) -> Result<Value, &str>;
    fn put(&mut self, value: Value);
}
