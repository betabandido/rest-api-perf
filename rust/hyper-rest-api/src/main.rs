extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate pretty_env_logger;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod repositories;
mod service;

use std::sync::{Arc, Mutex};

use repositories::{Value, ValueRepository};
use repositories::inmemory::InMemoryRepository;
use service::ValueService;

fn main() {
    pretty_env_logger::init();

    let entity_repository = Arc::new(Mutex::new(InMemoryRepository::new()));

    {
        let repo = &mut *entity_repository.lock().unwrap();
        repo.put(Value{key: "foo".to_string(), value: "bar".to_string()});
        repo.put(Value{key: "abc".to_string(), value: "123".to_string()});
    }

    let service = ValueService{repo: entity_repository};
    service.start(8000);
}
