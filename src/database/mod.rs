use rusqlite::{Connection, Result};
use std::error::Error;

pub mod model;
pub mod functions;

pub fn connect() -> Result<Connection, Box<dyn Error>> {
    let connection = Connection::open("database.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS Collections (
            CollectionId integer PRIMARY KEY,
            CollectionName text,
            CollectionDescription text,
            CreatedAt text,
            UpdatedAt text
            )",
        []
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS Labels (
            LabelId integer PRIMARY KEY,
            LabelName text,
            CollectionId integer REFERENCES Collections
            )",
        []
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS Items (
            ItemId integer PRIMARY KEY,
            ItemName text,
         	CollectionId integer REFERENCES Collections,
         	ItemDescription text
            )",
        []
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS ItemsLabels (
  			CollectionId integer REFERENCES Collections,
            ItemId integer REFERENCES Items,
            LabelId integer REFERENCES Labels,
            PRIMARY KEY (CollectionId, ItemId, LabelId)
            )",
        []
    )?;

    Ok(connection)
}
