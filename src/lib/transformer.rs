use crate::lib::models::{IfcEntity, IfcxEntity};
use std::collections::HashMap;

/// Maps raw IFC entities to structured IfcxEntity objects
pub fn transform_to_ifcx(entities: &[IfcEntity]) -> Vec<IfcxEntity> {
    entities.iter().enumerate().map(|(index, ent)| {
        // For now: simple path convention
        let path = format!("/entities/{}", index);

        // Placeholder: no real attribute parsing yet
        let mut attributes = HashMap::new();
        attributes.insert("raw".to_string(), ent.raw_data.clone());

        IfcxEntity {
            path,
            entity_type: ent.entity_type.clone(),
            attributes,
        }
    }).collect()
}


/*
Explanation
transform_to_ifcx() takes a slice of IfcEntity references and returns structured .ifcx-ready entities

path: dummy convention for now (/entities/0, /entities/1, ...)

attributes: stub — currently just stores the full raw data under "raw" key */