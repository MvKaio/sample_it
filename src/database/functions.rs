use rusqlite::Connection;
use std::error::Error;

use super::model::{Item, Label, Collection};

pub fn get_collections(connection: &Connection) -> Result<Vec<Collection>, Box<dyn Error>> {
    let mut statement = connection.prepare("\
        SELECT * FROM Collections
    ")?;

    let query_result = statement.query_map([], |row| {
        Ok(Collection {
            id: row.get("CollectionId")?,
            name: row.get("CollectionName")?,
            description: row.get("CollectionDescription")?,
            created_at: row.get("CreatedAt")?,
            updated_at: row.get("UpdatedAt")?,
            items: vec![],
            labels: vec![]
        })
    })?;

    let query_result = query_result.map(|opt| opt.unwrap());
    let collections = query_result.map(|collection| {
        let mut collection_with_labels = collection.clone();
        collection_with_labels.labels = get_collection_labels(collection.id, connection).unwrap();
        collection_with_labels
    });

    let collections = collections.map(|collection| {
        let mut collection_with_items = collection.clone();
        collection_with_items.items = get_collection_items(collection.id, connection).unwrap();
        collection_with_items
    });

    let collections: Vec<Collection> = collections.collect();

    Ok(collections)
}

pub fn get_collection_items(collection_id: u32, connection: &Connection) -> Result<Vec<Item>, Box<dyn Error>> {
    let mut statement = connection.prepare(format!("\
        SELECT *
        FROM Items
        WHERE CollectionId = {}
    ", collection_id).as_str())?;

    let query_result = statement.query_map([], |row| {
        Ok(Item {
            id: row.get("ItemId")?,
            name: row.get("ItemName")?,
            description: row.get("ItemDescription")?,
            labels: vec![],
        })
    })?;

    let query_result = query_result.map(|opt| opt.unwrap());
    let items = query_result.map(|item| {
        let mut item_with_labels = item.clone();
        item_with_labels.labels = get_item_labels(collection_id, item.id, connection).unwrap();
        item_with_labels
    });

    let items: Vec<Item> = items.collect();

    Ok(items)
}

pub fn get_collection_labels(collection_id: u32, connection: &Connection) -> Result<Vec<Label>, Box<dyn Error>> {
    let mut statement = connection.prepare(format!("\
        SELECT *
        FROM Labels
        WHERE CollectionId = {}
    ", collection_id).as_str())?;

    let query_result = statement.query_map([], |row| {
        Ok(Label {
            id: row.get("LabelId")?,
            name: row.get("LabelName")?,
        })
    })?;

    let query_result = query_result.map(|opt| opt.unwrap());

    Ok(query_result.collect())
}

pub fn get_item_labels(collection_id: u32, item_id: u32, connection: &Connection) -> Result<Vec<Label>, Box<dyn Error>> {
    let mut statement = connection.prepare(format!("\
        SELECT *
        FROM ItemsLabels NATURAL JOIN Labels
        WHERE CollectionId = {} AND ItemId = {}
    ", collection_id, item_id).as_str())?;

    let query_result = statement.query_map([], |row| {
        Ok(Label {
            id: row.get("LabelId")?,
            name: row.get("LabelName")?,
        })
    })?;

    let query_result = query_result.map(|opt| opt.unwrap());

    Ok(query_result.collect())
}
