//#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate serde_json;

mod repositories;

//use repositories::inmemory::InMemoryRepository;

use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper::service::service_fn;
use futures::{Future, Stream};
use futures::future;
use serde_json::Value;

//use std::sync::{Mutex, Arc};

//struct AppState {
//    repo: Arc<Mutex<InMemoryRepository>>
//}

fn handle_request(req: Request<Body>)
                  -> Box<Future<Item=Response<Body>, Error=hyper::Error> + Send> {
    let (parts, body) = req.into_parts();

    match (parts.method, parts.uri.path()) {
        (Method::PUT, "/api/values") => {
            let entire_body = body.concat2();

            let resp = entire_body.map(|body| {
                let v: Value = serde_json::from_slice(&body).unwrap();
                let body = Body::from(format!("body: {}", v.to_string()));
                Response::new(body)
            });

            Box::new(resp)
        }
        _ => {
            Box::new(future::ok(
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from(""))
                    .unwrap()
            ))
        }
    }
}

fn main() {
    pretty_env_logger::init();

    let addr = "127.0.0.1:8000".parse().unwrap();

//    InMemoryRepository

//    let entity_repository = Arc::new(
//        Mutex::new(
//            InMemoryRepository::new()
//        )
//    );
//
//    let repo = Arc::clone(&entity_repository);
//
//    {
//        let mut repo = repo.lock().unwrap();
//
//        repo.put("foo".to_string(), "bar".to_string());
//        repo.put("abc".to_string(), "123".to_string());
//    }

    hyper::rt::run(future::lazy(move || {
        let new_service = move || {
            service_fn(move |req| {
                handle_request(req)
            })
        };

        let server = Server::bind(&addr)
            .serve(new_service)
            .map_err(|e| eprintln!("server error: {}", e));

        println!("Listening on http://{}", addr);

        server
    }))
}
