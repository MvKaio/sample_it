use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
    description: String,
    labels: Vec<Label>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Label {
    id: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Collection {
    id : String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
    description: String,
    items: Vec<Item>
}

// pub fn get_collection_labels(collection: &Collection) -> Vec<Label>;
// pub fn get_items_by_label(collection: &Collection, label: &Label) -> Vec<Item>;
// pub fn get_label_frequency(collection: &Collection) -> HashMap<Label, u32>;
