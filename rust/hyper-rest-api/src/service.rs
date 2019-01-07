use futures::{future, Future, Stream};
use futures::future::FutureResult;
use hyper::{Body, Chunk, Method, Request, Response, StatusCode};
use hyper::service::{NewService, Service};
use hyper::server::Server;
use regex::Regex;
use std::sync::{Arc, Mutex};

use repositories::{Value, ValueRepository};

lazy_static! {
    static ref GET_VALUE_REGEX: Regex = Regex::new(r"^/api/values/(.*)$").unwrap();
}

pub struct ValueService<Repo: ValueRepository> {
    pub repo: Arc<Mutex<Repo>>
}

impl<Repo: ValueRepository> NewService for ValueService<Repo> {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Service = ValueService<Repo>;
    type Future = Box<Future<Item=Self::Service, Error=Self::InitError> + Send>;
    type InitError = hyper::Error;

    fn new_service(&self) -> Self::Future {
        Box::new(future::ok(Self {
            repo: self.repo.clone(),
        }))
    }
}

impl<Repo: ValueRepository> Service for ValueService<Repo> {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Response<Body>, Error=Self::Error> + Send>;

    fn call(&mut self, request: Request<Self::ReqBody>) -> Self::Future {
        match request.method() {
            &Method::GET => self.handle_get_request(request),
            &Method::PUT => self.handle_put_request(request),
            _ => self.bad_request_error_response(),
        }
    }
}

impl<Repo: ValueRepository> ValueService<Repo> {
    pub fn start(self, port: u16) {
        let addr = format!("0.0.0.0:{}", port).parse().unwrap();
        let server = Server::bind(&addr)
            .serve(self)
            .map_err(|e| eprintln!("error: {}", e));

        println!("Serving at {}", addr);
        hyper::rt::run(server);
    }

    fn handle_get_request(&self, request: Request<<ValueService<Repo> as Service>::ReqBody>)
                          -> <ValueService<Repo> as Service>::Future {
        let repo = self.repo.lock().unwrap();

        let resp = match GET_VALUE_REGEX.captures(request.uri().path()) {
            Some(capture) => {
                let key = capture.get(1).unwrap().as_str().to_string();
                match repo.get(key) {
                    Ok(value) => Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(value))
                        .unwrap(),
                    Err(_) => Response::builder()
                        .status(StatusCode::NOT_FOUND)
                        .body(Body::empty())
                        .unwrap(),
                }
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()
        };

        Box::new(future::ok(resp))
    }

    fn handle_put_request(&mut self, request: Request<<ValueService<Repo> as Service>::ReqBody>)
                          -> <ValueService<Repo> as Service>::Future {
        if request.uri().path() == "/api/values" {
            let repo = self.repo.clone();

            let resp = request
                .into_body()
                .concat2()
                .and_then(parse_body)
                .and_then(move |value| {
                    let repo = repo.clone();
                    let repo = &mut *repo.lock().unwrap();
                    update_value(value, repo)
                })
                .then(make_put_response::<Repo>);

            Box::new(resp)
        } else {
            Box::new(future::ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()
            ))
        }
    }

    fn bad_request_error_response(&self) -> <ValueService<Repo> as Service>::Future {
        Box::new(future::ok(
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .unwrap()
        ))
    }
}

fn parse_body(body: Chunk) -> FutureResult<Value, hyper::Error> {
    let value: Value = serde_json::from_slice(&body).unwrap();
    future::ok(value)
}

fn update_value<Repo: ValueRepository>(value: Value, repo: &mut Repo) -> FutureResult<i64, hyper::Error> {
    repo.put(value.key, value.value);
    future::ok(0)
}

fn make_put_response<Repo: ValueRepository>(_result: Result<i64, hyper::Error>)
                     -> FutureResult<
                         Response<<ValueService<Repo> as Service>::ResBody>,
                         hyper::Error
                     > {
    future::ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap())
}
