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

pub fn get_collection(collection_id: u32, connection: &Connection) -> Result<Collection, Box<dyn Error>> {
    let mut statement = connection.prepare(format!("\
        SELECT * FROM Collections
        WHERE CollectionId = {}
    ", collection_id).as_str())?;

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
    let mut query_result = query_result.map(|opt| opt.unwrap());

    let mut collection = query_result.next().unwrap();
    collection.labels = get_collection_labels(collection.id, connection).unwrap();
    collection.items = get_collection_items(collection.id, connection).unwrap();

    Ok(collection)
}


pub fn delete_collection(collection_id: u32, connection: &Connection) -> Result<(), Box<dyn Error>> {
    let mut statement = connection.prepare(format!("\
        DELETE FROM ItemsLabels
        WHERE CollectionId = {}
    ", collection_id).as_str())?;

    statement.execute([])?;

    let mut statement = connection.prepare(format!("\
        DELETE FROM Labels
        WHERE CollectionId = {}
    ", collection_id).as_str())?;

    statement.execute([])?;

    let mut statement = connection.prepare(format!("\
        DELETE FROM Items
        WHERE CollectionId = {}
    ", collection_id).as_str())?;

    statement.execute([])?;

    let mut statement = connection.prepare(format!("\
        DELETE FROM Collections
        WHERE CollectionId = {}
    ", collection_id).as_str())?;

    statement.execute([])?;

    Ok(())
}

pub fn update_collection(collection_id: u32, collection: &Collection, connection: &Connection) -> Result<Collection, Box<dyn Error>> {
    let mut collection = collection.clone();
    collection.id = collection_id;
    delete_collection(collection_id, connection)?;
    push_collection(&collection, connection, true)
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

pub fn push_item(item: &Item, collection_id: u32, connection: &Connection) -> Result<Item, Box<dyn Error>> {
    let mut statement = connection.prepare(format!("
        INSERT INTO Items (ItemName, ItemDescription, CollectionId)
        VALUES ('{}', '{}', {})
    ",  item.name,
        item.description,
        collection_id).as_str())?;
    
    let rowid = statement.insert([])?;
    let mut item = item.clone();
    item.id = rowid as u32;

    for label in item.labels.iter() {
        let mut label_statement = connection.prepare(format!("
            INSERT INTO ItemsLabels (CollectionId, ItemId, LabelId)
            VALUES ({}, {}, {})
        ", collection_id, item.id, label.id).as_str())?;
        label_statement.execute([])?;
    }

    Ok(item)
}

pub fn push_label(label: &Label, collection_id: u32, connection: &Connection) -> Result<Label, Box<dyn Error>> {
    let mut statement = connection.prepare(format!("
        INSERT INTO Labels (LabelName, CollectionId)
        VALUES ('{}', {})
    ",  label.name,
        collection_id).as_str())?;
    
    let rowid = statement.insert([])?;
    
    let mut label = label.clone();
    label.id = rowid as u32;

    Ok(label)
}

pub fn push_collection(collection: &Collection, connection: &Connection, use_collection_id: bool) -> Result<Collection, Box<dyn Error>> {
    let mut statement = 
        if use_collection_id {
            connection.prepare(format!("
                INSERT INTO Collections (CollectionId, CollectionName, CollectionDescription, CreatedAt, UpdatedAt)
                VALUES ('{}', '{}', '{}', '{}', '{}')
                ",
                collection.id,  
                collection.name,
                collection.description,
                collection.created_at.format("%+"),
                collection.updated_at.format("%+")).as_str()
            )?
        } else {
            connection.prepare(format!("
            INSERT INTO Collections (CollectionName, CollectionDescription, CreatedAt, UpdatedAt)
            VALUES ('{}', '{}', '{}', '{}')
            ",     
                collection.name,
                collection.description,
                collection.created_at.format("%+"),
                collection.updated_at.format("%+")).as_str()
            )?
        };
    let rowid = statement.insert([])?;
    let mut collection = collection.clone();
    if use_collection_id == false {
        collection.id = rowid as u32;
    }

    collection.labels = collection.labels.iter().map(
        |label| push_label(label, collection.id, connection).unwrap()
        ).collect();

    collection.items = collection.items.iter().map(
        |item| {
            let mut item_with_correct_labels = item.clone();
            item_with_correct_labels.labels = item.labels.iter().map(
                |label| {
                    (collection.labels.iter().find(|l| l.name == label.name).unwrap()).clone()
                }
            ).collect();
            push_item(&item_with_correct_labels, collection.id, connection).unwrap()
        }).collect();

    Ok(collection)
}
