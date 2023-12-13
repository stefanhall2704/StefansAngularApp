use diesel::connection::Connection;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use std::env;

use self::models::NewHouse;
use self::models::House as house;
use self::schema::House as house_schema;

pub mod models;
pub mod schema;

#[allow(non_snake_case)]
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


use diesel::result::Error as DieselError;

pub fn create_db_house(
    conn: &mut SqliteConnection,
    house_name: String,
    city: String,
    state_name: String,
    photo: String,
    available_units: String,
    wifi: bool,
    laundry: bool,
) -> Result<(), DieselError> {
    let house = NewHouse {
        HouseName: house_name, 
        City: city, 
        StateName: state_name, 
        Photo: photo, 
        AvailableUnits: available_units, 
        Wifi: wifi, 
        Laundry: laundry
    };

    diesel::insert_into(house_schema::table)
        .values(&house)
        .execute(conn)
        .map(|_| ()) // Convert the successful result to ()
}


pub fn get_db_house_by_id(id: i32) -> Result<NewHouse, ()> {
    let connection = &mut establish_connection();
    let house_db = house_schema::table
        .filter(house_schema::ID.eq(id))
        .first::<house>(connection)
        .unwrap();
    let data = NewHouse {
        HouseName: house_db.HouseName,
        City: house_db.City,
        StateName: house_db.StateName,
        Photo: house_db.Photo,
        AvailableUnits: house_db.AvailableUnits,
        Wifi: house_db.Wifi,
        Laundry: house_db.Laundry,
    };
    Ok(data)
}

pub fn delete_db_house(conn: &mut SqliteConnection, id: i32) {
    diesel::delete(house_schema::table.find(id))
        .execute(conn)
        .expect("Error deleting team");
}

pub fn get_db_house_listings(
    conn: &mut SqliteConnection,
) -> Result<Vec<models::House>, ()> {
    let house_listings_db = house_schema::table
        .load::<house>(conn)
        .unwrap();
    Ok(house_listings_db)
}