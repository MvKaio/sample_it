use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Item {
    id: u32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
    description: String,
    labels: Vec<Label>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Label {
    id: u32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    title: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub description: String,
    pub items: Vec<Item>
}

impl Collection {
	pub fn new(id : u32,
			   created_at: DateTime<Utc>,
			   updated_at: DateTime<Utc>,
			   title: String,
			   description: String,
			   items: Vec<Item>
			   ) -> Self {
		Collection {
			id,
			created_at,
			updated_at,
			title,
			description,
			items
		}
	}
}

// pub fn get_collection_labels(collection: &Collection) -> Vec<Label>;
// pub fn get_items_by_label(collection: &Collection, label: &Label) -> Vec<Item>;
// pub fn get_label_frequency(collection: &Collection) -> HashMap<Label, u32>;
