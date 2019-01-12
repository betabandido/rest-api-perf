extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate parking_lot;
extern crate pretty_env_logger;
extern crate regex;
extern crate rest_api_common;
extern crate serde_json;

mod service;

use std::sync::Arc;

use parking_lot::RwLock;

use rest_api_common::{make_value_repository};

use service::ValueService;

fn main() {
    pretty_env_logger::init();

    let service = ValueService{
        repo: Arc::new(RwLock::new(make_value_repository()))
    };
    service.start(8000);
}
