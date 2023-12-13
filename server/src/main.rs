#![feature(proc_macro_hygiene, decl_macro)]
#![allow(non_snake_case)]
use rocket::get;
use rocket::routes;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
use self::api::*;
pub mod api;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use prometheus::{Opts, Registry, Counter, TextEncoder, Encoder, CounterVec};
use rocket::response::content;
use rocket::State;


pub struct Metrics {
    pub counter: Counter,
    pub registry: Registry,
    pub http_responses: CounterVec
}

impl Metrics {
    pub fn new() -> Metrics {
        let counter_opts = Opts::new("test_counter", "A counter for testing");
        let counter = Counter::with_opts(counter_opts).unwrap();

        let http_responses_opts = Opts::new("http_responses", "HTTP Response Status Codes");
        let http_responses = CounterVec::new(http_responses_opts, &["status"]).unwrap();

        let registry = Registry::new();
        registry.register(Box::new(counter.clone())).unwrap();
        registry.register(Box::new(http_responses.clone())).unwrap(); // Register the http_responses counter

        Metrics { counter, registry, http_responses }
    }
}


#[get("/metrics")]
fn metrics(metrics: State<Metrics>) -> content::Plain<String> {
    let encoder = TextEncoder::new();
    let metric_families = metrics.registry.gather();
    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();
    content::Plain(String::from_utf8(buffer).unwrap())
}

use rocket::{fairing::{Fairing, Info, Kind}, Request, Response};

pub struct MetricsFairing;

impl Fairing for MetricsFairing {
    fn info(&self) -> Info {
        Info {
            name: "Metrics Recorder",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if let Some(metrics) = request.guard::<State<Metrics>>().succeeded() {
            let status_code = response.status().code.to_string();
            metrics.inner().http_responses.with_label_values(&[&status_code]).inc();
        }
    }
}

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
    let metrics = Metrics::new();
    rocket::ignite()
        .manage(metrics)
        .attach(cors)
        .attach(MetricsFairing)
        .mount(
            "/",
            routes![
                create_house_listing,
                delete_house_listing,
                get_house_listing,
                get_all_house_listings,
                metrics
            ],
        )
        .launch();
}
