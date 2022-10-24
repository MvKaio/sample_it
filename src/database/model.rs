use chrono::{DateTime, Utc};
use serde::{Serialize, Serializer, Deserialize, Deserializer};

use std::fmt;
use serde::de::{self, Visitor};

fn default_id() -> u32 { 0 }

// Definition of The Three Main Data Types of the Application

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    #[serde(default = "default_id")]
    pub id: u32,
    pub name: String,
    pub description: String,
    pub labels: Vec<Label>
}

#[derive(Debug, Clone)]
pub struct Label {
    pub id: u32,
    pub name: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    #[serde(default = "default_id")]
    pub id: u32,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub items: Vec<Item>,
    pub labels: Vec<Label>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    LessOrEqual, Equal, GreaterOrEqual
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub operator: Operator,
    pub number: u32,
    pub label: Label
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    #[serde(default = "default_id")]
    pub id: u32,
    pub name: String,
    pub description: String,
    pub collection_id: u32,
    pub size: u32,
    pub constraints: Vec<Constraint>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SampleResult {
    SolutionNotFound,
    Solved { items: Vec<Item> }
}

// Implementation of Serialization and Deserialization methods

// Label Serialization

impl Serialize for Label {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.name.as_str())
    }
}

// Label Deserialization

struct LabelVisitor;

impl<'de> Visitor<'de> for LabelVisitor {
    type Value = Label;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing a Label")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error, {
        Ok(Self::Value {
            id: 0,
            name: String::from(v)
        })
    }
}

impl<'de> Deserialize<'de> for Label {
    fn deserialize<D>(deserializer: D) -> Result<Label, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(LabelVisitor)
    }
}
