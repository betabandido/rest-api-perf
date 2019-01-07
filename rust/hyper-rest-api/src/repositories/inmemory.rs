use std::collections::HashMap;

use repositories::{Value, ValueRepository};

pub struct InMemoryRepository {
    dict: HashMap<String, Value>,
}

impl InMemoryRepository {
    pub fn new() -> InMemoryRepository {
        InMemoryRepository {
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
