#![allow(non_snake_case)]
use rocket::{State, post, delete, get, response::status, http::Status};
use serde_json::{to_string, Value};
use server::*;
use rocket_contrib::json::{Json, JsonValue};
use crate::Metrics;


extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

#[derive(serde::Deserialize)]
pub struct HouseData {
    houseName: String,
    cityName: String,
    stateName: String,
    photo: String,
    wifi: bool,
    laundry: bool,
}


#[post("/api/house/create", format = "json", data = "<json>")]
pub fn create_house_listing(
    state: State<Metrics>, 
    json: Json<HouseData>
) -> Result<status::Custom<String>, status::Custom<String>> {
    state.counter.inc();
    let connection = &mut establish_connection();

    let houseName = &json.houseName;
    let cityName = &json.cityName;
    let stateName = &json.stateName;
    let photo = &json.photo;
    let wifi = json.wifi;
    let laundry = json.laundry;
    let available_units  = "0";

    match create_db_house(
        connection,
        houseName.clone(),
        cityName.clone(),
        stateName.clone(),
        photo.clone(),
        available_units.to_string(),
        wifi,
        laundry
    ) {
        Ok(_) => {
            state.http_responses.with_label_values(&["200"]).inc();
            Ok(status::Custom(Status::Created, "House created".to_string()))
        },
        Err(_) => {
            state.http_responses.with_label_values(&["400"]).inc();
            Err(status::Custom(Status::BadRequest, "Failed to create house".to_string()))
        }
    }
}

#[delete("/api/houseListing/<id>", format = "application/json", data = "<json>")]
pub fn delete_house_listing(id: i32, json: Json<JsonValue>) -> Result<std::string::String, ()> {
    //required print statement
    println!("{}", json.to_string());
    let connection = &mut establish_connection();
    delete_db_house(connection, id);
    let response = format!("House Listing Deleted{}", id);
    Ok(response)
}

#[get("/api/listing/<id>")]
pub fn get_house_listing(id: i32) -> Result<std::string::String, ()> {
    let house_listing = get_db_house_by_id(id).unwrap();
    let user_json = to_string(&house_listing).unwrap();
    Ok(user_json)
}

#[get("/api/all_listings")]
pub fn get_all_house_listings() -> Result<std::string::String, ()> {
    let connection = &mut establish_connection();
    let listings = get_db_house_listings(connection).unwrap();
    let listings_json = serde_json::to_string(&listings).unwrap();
    Ok(listings_json)
}

