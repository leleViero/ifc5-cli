use serde::Serialize;
use std::collections::HashMap;

/// A structured entity ready to be serialized as part of a .ifcx file
#[derive(Debug, Serialize)]
#[derive(Clone)]
pub struct IfcxEntity {
    pub path: String,
    #[serde(rename = "type")] // ensures the output field is "type", not entity_type, to match the .ifcx schema
    pub entity_type: String,
    pub attributes: HashMap<String, String>, // for now we store key-value as strings
}