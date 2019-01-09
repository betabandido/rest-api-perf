use std::sync::Arc;

use futures::{future, Future, Stream};
use futures::future::FutureResult;
use hyper::{Body, Chunk, Method, Request, Response, StatusCode};
use hyper::service::{NewService, Service};
use hyper::server::Server;
use parking_lot::RwLock;
use regex::Regex;

use repositories::{Value, ValueRepository};

lazy_static! {
    static ref GET_VALUE_REGEX: Regex = Regex::new(r"^/api/values/(.*)$").unwrap();
}

pub struct ValueService {
    pub repo: Arc<RwLock<Box<dyn ValueRepository>>>,
}

impl NewService for ValueService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = hyper::Error;
    type Service = ValueService;
    type Future = Box<Future<Item=Self::Service, Error=Self::InitError> + Send>;
    type InitError = hyper::Error;

    fn new_service(&self) -> Self::Future {
        Box::new(future::ok(Self {
            repo: self.repo.clone(),
        }))
    }
}

impl Service for ValueService {
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

impl ValueService {
    pub fn start(self, port: u16) {
        let addr = format!("0.0.0.0:{}", port).parse().unwrap();
        let server = Server::bind(&addr)
            .serve(self)
            .map_err(|e| eprintln!("error: {}", e));

        println!("Serving at {}", addr);
        hyper::rt::run(server);
    }

    fn handle_get_request(&self, request: Request<<ValueService as Service>::ReqBody>)
                          -> <ValueService as Service>::Future {
        let resp = match GET_VALUE_REGEX.captures(request.uri().path()) {
            Some(capture) => {
                let key = capture.get(1).unwrap().as_str().to_string();
                let repo = &self.repo.read();
                match repo.get(key) {
                    Ok(value) => {
                        let content = serde_json::to_string(&value).unwrap();
                        Response::builder()
                            .status(StatusCode::OK)
                            .body(Body::from(content))
                            .unwrap()
                    }
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

    fn handle_put_request(&mut self, request: Request<<ValueService as Service>::ReqBody>)
                          -> <ValueService as Service>::Future {
        if request.uri().path() == "/api/values" {
            let repo = self.repo.clone();

            let resp = request
                .into_body()
                .concat2()
                .and_then(Self::parse_body)
                .and_then(move |value| {
                    let repo = repo.clone();
                    let repo = &mut repo.write();
                    let repo = &mut **repo;
                    Self::update_value(value, repo)
                })
                .then(Self::make_put_response);

            Box::new(resp)
        } else {
            Box::new(future::ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())
                .unwrap()
            ))
        }
    }

    fn bad_request_error_response(&self) -> <ValueService as Service>::Future {
        Box::new(future::ok(
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .unwrap()
        ))
    }

    fn parse_body(body: Chunk) -> FutureResult<Value, hyper::Error> {
        let value: Value = serde_json::from_slice(&body).unwrap();
        future::ok(value)
    }

    fn update_value(value: Value, repo: &mut Box<dyn ValueRepository>)
                    -> FutureResult<i64, hyper::Error> {
        repo.put(value);
        future::ok(0)
    }

    fn make_put_response(_result: Result<i64, hyper::Error>)
                         -> FutureResult<
                             Response<<ValueService as Service>::ResBody>,
                             hyper::Error
                         > {
        future::ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::empty())
            .unwrap())
    }
}

