#![allow(unused_imports, dead_code)]
use std::fs;
use std::collections::{HashMap, HashSet};
use rusqlite::{Connection, Result};

pub mod model;

#[derive(Debug, PartialEq)]
pub struct Item {
    description: String,
    labels: HashSet<String>
}

#[derive(Debug, PartialEq)]
struct ItemLabel {
    item_description: String,
    label_description: String
}

pub fn connect() -> Result<Connection, Box<dyn std::error::Error>> {
    let connection = Connection::open("database.db")?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS Collections (
            CollectionId integer PRIMARY KEY,
            CollectionDescription text NOT NULL UNIQUE
            )",
        []
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS Labels (
            LabelId integer PRIMARY KEY,
            LabelDescription text NOT NULL UNIQUE
            )",
        []
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS Items (
            ItemId integer,
         	CollectionId integer REFERENCES Collection,
         	ItemDescription text NOT NULL UNIQUE,
  			PRIMARY KEY (ItemId, CollectionId) 
            )",
        []
    )?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS ItemsLabels (
            ItemId integer REFERENCES Items,
  			CollectionID integer REFERENCES Collections,
            LabelId integer REFERENCES Labels,
            PRIMARY KEY (ItemId, CollectionId, LabelId)
            )",
        []
    )?;


    Ok(connection)
}

fn get_all_items(
    connection: &Connection
) -> Result<Vec<Item>, Box<dyn std::error::Error>> {

    let mut stmt = connection.prepare("\
        SELECT ItemDescription, LabelDescription
        FROM ItemsLabels JOIN Items JOIN Labels
    ")?;
    
    let items_iter = stmt.query_map([], |row| {
        Ok(ItemLabel{
            item_description: row.get(0)?, 
            label_description: row.get(1)?
        })
    })?;


    let mut item_labels = HashMap::<String, Vec<String>>::new();
    for item in items_iter.flatten() {
        let entry = item_labels.entry(item.item_description).or_insert(vec![]);
        entry.push(item.label_description);
    }

    let mut items = Vec::new();
    for (key, values) in item_labels.iter() {
        items.push(Item {
            description: key.clone(),
            labels: HashSet::from_iter(values.iter().cloned())
        });
    }

    Ok(items)
}

mod tests {
    use super::*;

    #[test]
    fn connects_without_error() {
        assert!(matches!(connect(), Ok(_)));
    }

    #[test]
    fn gets_items() {
        let connection = connect().unwrap();
        assert!(matches!(get_all_items(&connection), Ok(_)));
    }

    #[test]
    fn single_insert() {
        let mut connection = connect().unwrap();
        let transaction = connection.transaction().unwrap();

        transaction.execute(
            "INSERT INTO Items (ItemId, ItemDescription) VALUES (1, 'teste')",
            []
        ).unwrap();

        transaction.execute(
            "INSERT INTO Labels(LabelId, LabelDescription) VALUES (1, 'label1')",
            []
        ).unwrap();

        transaction.execute(
            "INSERT INTO Labels(LabelId, LabelDescription) VALUES (2, 'label2')",
            []
        ).unwrap();

        transaction.execute(
            "INSERT INTO ItemsLabels(ItemId, LabelId) VALUES (1, 1)",
            []
        ).unwrap();

        transaction.execute(
            "INSERT INTO ItemsLabels(ItemId, LabelId) VALUES (1, 2)",
            []
        ).unwrap();

        let expected_item = Item {
            description: String::from("teste"),
            labels: HashSet::from_iter(vec![String::from("label1"), String::from("label2")].iter().cloned())
        };

        match get_all_items(&transaction) {
            Ok(items) => {
                assert_eq!(items.len(), 1);
                assert_eq!(items[0], expected_item);
            },
            Err(_) => panic!()
        };
    }
}
