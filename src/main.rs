#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::LenientForm;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[derive(FromForm)]
struct SearchQuery {
    query: String,
}

#[get("/")]
fn index() -> Template {
    let context = HashMap::<String, String>::new();
    Template::render("index", context)
}

#[post("/search", data = "<query>")]
fn search(query: LenientForm<SearchQuery>) -> String {
    unimplemented!();
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index])
    .attach(Template::fairing())
    .launch();
}
