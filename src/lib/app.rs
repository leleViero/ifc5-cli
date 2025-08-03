use std::fs;
use std::path::Path;

use crate::lib::parser::parse_ifc_entities;
use crate::lib::models::ifc_entity::IfcEntity;

/// Main orchestration logic
pub fn run_test(input_path: &str, output_path: &str) -> Result<(), String> {
    if !Path::new(input_path).exists() {
        return Err(format!("Input file '{}' does not exist.", input_path));
    }

    println!("[1] Reading from input: {}", input_path);
    println!("[2] Output will be written to: {}", output_path);

    let content = fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read input file: {}", e))?;

    println!("[3] Read {} bytes from input file.", content.len());

    // TODO: parser, transformer, writer

    println!("[✔] Pipeline skeleton complete.");
    Ok(())
}

pub fn run(input_path: &str, output_path: &str) -> Result<(), String> {
    if !Path::new(input_path).exists() {
        return Err(format!("Input file '{}' does not exist.", input_path));
    }

    println!("[1] Reading from input: {}", input_path);
    println!("[2] Output will be written to: {}", output_path);

    let content = fs::read_to_string(input_path)
        .map_err(|e| format!("Failed to read input file: {}", e))?;

    println!("[3] Read {} bytes from input file.", content.len());

    // Parse IFC entities (STEP → Vec<IfcEntity>)
    let entities: Vec<IfcEntity> = parse_ifc_entities(&content);
    println!("[4] Parsed {} entities from input file.", entities.len());

    for (i, ent) in entities.iter().take(5).enumerate() {
        println!("    [{}] {} → {}", i + 1, ent.id, ent.entity_type);
    }

    // TODO: Pass to transformer

    println!("[✔] Parsing complete, ready for transformation.");
    Ok(())
}

