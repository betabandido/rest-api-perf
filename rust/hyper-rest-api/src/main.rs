extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
#[macro_use(bson, doc)]
extern crate mongodb;
extern crate parking_lot;
extern crate pretty_env_logger;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod repositories;
mod service;

use std::env;
use std::sync::Arc;

use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use parking_lot::RwLock;

use repositories::ValueRepository;
use repositories::inmemory::InMemoryRepository;
use repositories::mongo::MongoRepository;
use service::ValueService;

fn main() {
    pretty_env_logger::init();

    let repository_type = match env::var("REST_API_PERF_REPO") {
        Ok(value) => value,
        _ => "in-memory".to_string(),
    };

    let entity_repository: Box<dyn ValueRepository> =
        match repository_type.as_str() {
            "mongo" => {
                let client = Client::connect("localhost", 27017)
                    .ok().expect("failed to connect to mongodb");

                Box::new(
                    MongoRepository::new(client.db("test").collection("values"))
                )
            },
            "in-memory" => Box::new(InMemoryRepository::new()),
            &_ => unimplemented!()
        };

    println!("Using {:?}", entity_repository);

    let service = ValueService{repo: Arc::new(RwLock::new(entity_repository))};
    service.start(8000);
}
