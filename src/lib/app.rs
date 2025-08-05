use std::fs;
use std::path::Path;

use crate::lib::parser::parse_ifc_entities;
use crate::lib::writer::write_ifcx_file;
use crate::lib::models::ifc_entity::IfcEntity;

use crate::lib::transformer::transform_to_ifcx;
use crate::lib::models::IfcxEntity;

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

    let parsed_entities = parse_ifc_entities(&content);
    let ifcx_entities: Vec<IfcxEntity> = transform_to_ifcx(&parsed_entities);

    println!("[5] Transformed {} entities to ifcx format.", ifcx_entities.len());

    for ent in ifcx_entities.iter().take(3) {
        println!("    - path: {}, type: {}", ent.path, ent.entity_type);
    }

    // TODO: Pass to transformer

    println!("[✔] Parsing complete, ready for transformation.");

    write_ifcx_file(output_path, &ifcx_entities)?;
    println!("[✔] Wrote .ifcx JSON file to {}", output_path);
    Ok(())
}

