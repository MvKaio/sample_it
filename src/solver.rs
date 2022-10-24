use std::error::Error;

use rusqlite::Connection;

use crate::database;
use crate::database::model::{Sample, SampleResult, Constraint};

pub fn solve(sample: Sample, connection: &Connection) -> Result<SampleResult, Box<dyn Error>> {
    let collection = database::functions::get_collection(sample.collection_id, connection)?;
    dbg!(sample.constraints.clone());
    let constraints: Vec<Constraint> = sample.constraints.iter().map(|cons| {
        let mut new_constraint = cons.clone();
        new_constraint.label = (collection.labels.iter().find(|l| l.name == cons.label.name).unwrap()).clone();
        new_constraint
    }).collect();
    dbg!(constraints);


    Ok(SampleResult::Solved { items: database::functions::get_collection_items(sample.collection_id, connection)? })
}
