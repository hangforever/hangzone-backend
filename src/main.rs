#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::State;
use std::sync::Mutex;

#[derive(FromForm)]
struct Hanger {
    id: usize,
    name: String,
}

struct HangzoneState {
    hit_count: Mutex<usize>,
}

#[get("/")]
fn index(s: State<HangzoneState>) -> String {
    let state = s.inner();
    let mut count = state.hit_count.lock().unwrap();    
    *count += 1; 
    format!("Hello, hanger! This page has been visited {} times.", *count)
}

#[get("/?<hanger..>")]
fn hanger(hanger: Form<Hanger>) -> String {
    format!(
        "The hanger's name is {} and their id is {}",
        hanger.name, hanger.id
    )
}

fn main() {
    rocket::ignite()
        .manage(HangzoneState {
            hit_count: Mutex::new(0),
        })
        .mount("/", routes![index])
        .mount("/hanger", routes![hanger])
        .launch();
}
