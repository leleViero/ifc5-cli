use crate::lib::models::IfcxEntity;
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Root .ifcx structure
#[derive(Serialize)]
struct IfcxDocument {
    ifcx: String,
    entities: Vec<IfcxEntity>,
}

/// Writes a .ifcx JSON file
pub fn write_ifcx_file(output_path: &str, entities: &[IfcxEntity]) -> Result<(), String> {
    let doc = IfcxDocument {
        ifcx: "alpha".to_string(),
        entities: entities.to_vec(),
    };

    let json = serde_json::to_string_pretty(&doc)
        .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

    let mut file = File::create(Path::new(output_path))
        .map_err(|e| format!("Failed to create file: {}", e))?;

    file.write_all(json.as_bytes())
        .map_err(|e| format!("Failed to write JSON to file: {}", e))?;

    Ok(())
}