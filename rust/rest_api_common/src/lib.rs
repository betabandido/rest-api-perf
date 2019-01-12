#[macro_use(bson, doc)]
extern crate mongodb;
#[macro_use]
extern crate serde_derive;

mod inmemory;
mod mongo;

use std::env;
use std::fmt::Debug;

use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

use crate::inmemory::InMemoryRepository;
use crate::mongo::MongoRepository;

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

pub fn make_value_repository() -> Box<dyn ValueRepository> {
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
            }
            "in-memory" => Box::new(InMemoryRepository::new()),
            &_ => unimplemented!()
        };

    println!("Using {:?}", entity_repository);

    entity_repository
}
