// @generated automatically by Diesel CLI.
#![allow(non_snake_case)]
diesel::table! {
    House (ID) {
        ID -> Integer,
        HouseName -> Text,
        City -> Text,
        StateName -> Text,
        Photo -> Text,
        AvailableUnits -> Text,
        Wifi -> Bool,
        Laundry -> Bool,
    }
}