use std::collections::HashMap;

use repositories::{Value, ValueRepository};

#[derive(Debug)]
pub struct InMemoryRepository {
    dict: HashMap<String, Value>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            dict: HashMap::new(),
        }
    }
}

impl ValueRepository for InMemoryRepository {
    fn get(&self, key: String) -> Result<Value, &str> {
        match self.dict.get(&key) {
            Some(value) => Ok(value.clone()),
            _ => Err("Not Found")
        }
    }

    fn put(&mut self, value: Value) {
        self.dict.insert(value.key.clone(), value.clone());
    }
}
