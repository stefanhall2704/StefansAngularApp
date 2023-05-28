#![allow(non_snake_case)]
use rocket::{post, delete};
use serde_json::{to_string, Value};
use server::*;
use rocket_contrib::json::{Json, JsonValue};

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;



#[post("/api/house/create", format = "text/plain", data = "<json>")]
pub fn create_house_listing(json: String) -> Result<&'static str, ()> {
    let connection = &mut establish_connection();

    let data_string = json;
    let data: &str = &data_string;
    let v: Value = serde_json::from_str(data).unwrap();


    let houseName = v["houseName"].as_str().unwrap().to_owned();
    println!("This is a print statement");
    println!("ecnj{}", houseName);
    let cityName = v["cityName"].as_str().unwrap().to_owned();
    let stateName = v["stateName"].as_str().unwrap().to_owned();
    let photo = v["photo"].as_str().unwrap().to_owned();
    let wifi: bool = v["wifi"].as_bool().unwrap();
    let laundry: bool = v["laundry"].as_bool().unwrap();
    let available_units: String = to_string("0").unwrap();

    create_db_house(
        connection,
        houseName,
        cityName,
        stateName,
        photo,
        available_units,
        wifi,
        laundry
    );
    Ok("House")
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