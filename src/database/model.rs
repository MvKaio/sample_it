use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
    description: String,
    labels: Vec<Label>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Label {
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Collection {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub description: String,
    pub items: Vec<Item>
}

// pub fn get_collection_labels(collection: &Collection) -> Vec<Label>;
// pub fn get_items_by_label(collection: &Collection, label: &Label) -> Vec<Item>;
// pub fn get_label_frequency(collection: &Collection) -> HashMap<Label, u32>;
