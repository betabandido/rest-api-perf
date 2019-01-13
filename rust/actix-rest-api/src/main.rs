extern crate actix_web;
extern crate rest_api_common;

use std::sync::Arc;

use actix_web::{
    App,
    http,
    HttpResponse,
    Json,
    Path,
    server,
    State
};
use parking_lot::RwLock;

use rest_api_common::{make_value_repository, Value, ValueRepository};

struct AppState {
    repo: Arc<RwLock<Box<dyn ValueRepository>>>
}

fn get_value(state: State<AppState>, path: Path<String>) -> actix_web::Result<Json<Value>> {
    let key = path.to_string();
    let repo = &state.repo.read();
    match repo.get(key) {
        Ok(value) => Ok(Json(value)),
        Err(e) => Err(actix_web::error::ErrorNotFound(e.to_string())),
    }
}

fn put_value(state: State<AppState>, value: Json<Value>) -> HttpResponse {
    let repo = &mut state.repo.write();
    repo.put(value.0);
    HttpResponse::new(http::StatusCode::OK)
}

fn main() {
    let repo = make_value_repository();
    let repo = Arc::new(RwLock::new(repo));

    server::new(move || {
        App::with_state(AppState { repo: repo.clone() })
            .prefix("/api")
            .resource("/values/{key}",
                      |r| r.method(http::Method::GET).with(get_value))
            .resource("/values",
                      |r| r.method(http::Method::PUT).with(put_value))
    })
        .bind("127.0.0.1:8000")
        .expect("Cannot bind to port 8000")
        .run();
}
