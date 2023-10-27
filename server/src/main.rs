#![feature(proc_macro_hygiene, decl_macro)]
#![allow(non_snake_case)]
use rocket::routes;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
use self::api::*;
pub mod api;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};

fn main() {
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = vec![
        rocket::http::Method::Get,
        rocket::http::Method::Delete,
    ]
    .into_iter()
    .map(From::from)
    .collect();

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Failed to create CORS");

    rocket::ignite()
        .attach(cors)
        .mount(
            "/",
            routes![
                create_house_listing,
                delete_house_listing,
                get_house_listing,
                get_all_house_listings
            ],
        )
        .launch();
}
