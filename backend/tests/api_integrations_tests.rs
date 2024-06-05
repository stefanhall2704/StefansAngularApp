use rocket::local::Client;
use rocket::http::{Status, ContentType};
use rocket::routes;
use serde_json::json; // Make sure serde_json is in your Cargo.toml
use serde::{Serialize, Deserialize};

// api_integration_tests.rs in the tests directory

extern crate server; // This line is often optional with Rust 2018 edition and onwards but can be necessary for clarity.
use server::api::{create_house_listing, delete_house_listing, get_house_listing, get_all_house_listings}; // Adjust according to your actual module structure and exports

// Your test code here


#[derive(Serialize, Deserialize)]
struct HouseData {
    houseName: String,
    cityName: String,
    stateName: String,
    photo: String,
    wifi: bool,
    laundry: bool,
}

#[test]
fn test_create_house_listing() {
    let rocket = rocket::ignite().mount("/", routes![create_house_listing, delete_house_listing, get_house_listing, get_all_house_listings]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let data = HouseData {
        houseName: "Example House".into(),
        cityName: "Example City".into(),
        stateName: "Example State".into(),
        photo: "example.jpg".into(),
        wifi: true,
        laundry: false,
    };

    let serialized_data = serde_json::to_string(&data).expect("Failed to serialize data");

    let response = client.post("/api/house/create")
        .header(ContentType::JSON)
        .body(serialized_data) 
        .dispatch();

    assert_eq!(response.status(), Status::Ok); 
}
