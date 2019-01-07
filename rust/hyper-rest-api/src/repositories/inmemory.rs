use std::collections::HashMap;

use repositories::ValueRepository;

pub struct InMemoryRepository {
    dict: HashMap<String, String>,
}

impl InMemoryRepository {
    pub fn new() -> InMemoryRepository {
        InMemoryRepository {
            dict: HashMap::new(),
        }
    }
}

impl ValueRepository for InMemoryRepository {
    fn get(&self, key: String) -> Result<String, &str> {
        match self.dict.get(&key) {
            Some(value) => Ok(value.to_string()),
            _ => Err("Not Found")
        }
    }

    fn put(&mut self, key: String, value: String) {
        self.dict.insert(key, value);
    }
}
