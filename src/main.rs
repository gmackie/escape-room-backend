#![feature(proc_macro_hygiene, decl_macro)]
use rocket::response::content;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> content::Json<&'static str> {
        content::Json("{ 'hi': 'world' }")
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
