pub mod inmemory;

#[derive(Serialize, Deserialize)]
pub struct Value {
    pub key: String,
    pub value: String,
}

pub trait ValueRepository: Send + 'static {
    fn get(&self, key: String) -> Result<String, &str>;
    fn put(&mut self, key: String, value: String);
}
