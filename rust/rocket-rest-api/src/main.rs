#![feature(proc_macro_hygiene, decl_macro)]

extern crate parking_lot;
#[macro_use]
extern crate rocket;
extern crate rest_api_common;

use parking_lot::RwLock;
use rocket::State;
use rocket_contrib::json::Json;

use rest_api_common::{make_value_repository, Value, ValueRepository};

#[get("/values/<key>")]
fn get_value(key: String, app: State<App>) -> Option<Json<Value>> {
    let repo = &app.repo.read();
    match repo.get(key) {
        Ok(value) => Some(Json(value)),
        Err(_) => None
    }
}

#[put("/values", data = "<value>")]
fn put_value(value: Json<Value>, app: State<App>) {
    let repo = &mut app.repo.write();
    repo.put(value.0);
}

struct App {
    repo: RwLock<Box<dyn ValueRepository>>
}

fn main() {
    let repo = make_value_repository();

    rocket::ignite()
        .mount("/api", routes![get_value, put_value])
        .manage(App { repo: RwLock::new(repo) })
        .launch();
}
