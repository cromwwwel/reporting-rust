use serde::{Deserialize,Serialize};
use schemars::JsonSchema;
use chrono::{ DateTime, Utc };

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Metadata {
    pub id: String,
    pub title: String,
    pub version: String,
    pub author: Option<String>,
    pub created_at: DateTime<Utc>,    
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Report {
    pub metadata: Metadata,
    pub data: serde_json::Value,
}

fn main() {
    let schema = schemars::schema_for!(Report);
    let schema_json = serde_json::to_string_pretty(&schema).unwrap();
    std::fs::write("report.schema.json", schema_json).unwrap();
    println!("Esquema generado en report.schema.json");
}
