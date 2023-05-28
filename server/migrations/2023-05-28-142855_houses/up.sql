-- Your SQL goes here
CREATE TABLE House (
  ID Integer PRIMARY KEY,
  HouseName varchar(255) NOT NULL,
  City varchar(255) NOT NULL,
  StateName varchar(255) NOT NULL,
  Photo varchar(255) NOT NULL,
  AvailableUnits varchar(255) NOT NULL,
  Wifi BOOLEAN NOT NULL,
  Laundry BOOLEAN NOT NULL
)