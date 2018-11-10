use std::collections::HashMap;

struct Value {
    key: String,
    value: String,
}

trait ValueRepository {
    fn get(&self, key: String) -> Result<Value, &str>;
    fn put(&self, value: Value);
}

pub struct InMemoryRepository {
    dict: HashMap<String, String>,
}

impl InMemoryRepository {
    fn new() -> InMemoryRepository {
        InMemoryRepository {
            dict: HashMap::new(),
        }
    }

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
