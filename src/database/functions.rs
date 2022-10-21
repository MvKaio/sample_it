use chrono::{DateTime, self};
use rusqlite::Connection;
use serde_rusqlite::from_rows;
use std::error::Error;

use super::model::{Item, Label, Collection};

pub fn get_collections(connection: &Connection) -> Result<Vec<Collection>, Box<dyn Error>> {
    let mut statement = connection.prepare("\
        SELECT * FROM Collections
    ")?;

    let query_result = statement.query_map([], |row| {
        Ok(Collection {
            name: row.get("CollectionName")?,
            description: row.get("CollectionDescription")?,
            created_at: chrono::offset::Utc::now(),
            updated_at: chrono::offset::Utc::now(),
            items: vec![]
        })
    })?;

    let query_result = query_result.map(|opt| opt.unwrap());
    let collections: Vec<Collection> = query_result.collect();

    Ok(collections)
}

pub fn get_collection(id: String, connection: &Connection) -> Result<Collection, Box<dyn Error>> {
    let mut statement = connection.prepare(format!("\
        SELECT * FROM Collections
        WHERE ID = {}
    ", id).as_str())?;

    let mut query_result = from_rows::<Collection>(statement.query([]).unwrap());
    Ok(query_result.next().unwrap()?)
}
