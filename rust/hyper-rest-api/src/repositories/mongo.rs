use mongodb::{Bson, from_bson, to_bson};
use mongodb::coll::Collection;

use repositories::{Value, ValueRepository};

#[derive(Debug)]
pub struct MongoRepository {
    collection: Collection,
}

impl MongoRepository {
    pub fn new(collection: Collection) -> Self {
        Self {
            collection,
        }
    }
}

impl ValueRepository for MongoRepository {
    fn get(&self, key: String) -> Result<Value, &str> {
        match self.collection.find_one(Some(doc! {"key": key}), None) {
            Ok(result) => match result {
                Some(doc) => Ok(from_bson::<Value>(Bson::Document(doc)).unwrap()),
                _ => Err("Not found")
            }
            Err(_) => Err("Not Found")
        }
    }

    fn put(&mut self, value: Value) {
        let doc =
            to_bson(&value)
                .and_then(|bson| {
                    Ok(bson.as_document().unwrap().clone())
                });

        if doc.is_ok() {
            // TODO: allow for updates!
            self.collection.insert_one(doc.unwrap(), None)
                .ok().expect("failed to insert document");
        }
    }
}
