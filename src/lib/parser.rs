use crate::models::IfcEntity;

/// Parses raw IFC content line by line and extracts entities like `#123= IFCTYPE(...)`.
pub fn parse_ifc_entities(input: &str) -> Vec<IfcEntity> {
    let mut entities = Vec::new();

    for line in input.lines() {
        // Skip comments or empty lines
        let trimmed = line.trim();
        if trimmed.is_empty() || !trimmed.starts_with('#') {
            continue;
        }

        // Try to split line: `#123= IFCTYPE(...);`
        if let Some(eq_pos) = trimmed.find('=') {
            let (id_part, rest) = trimmed.split_at(eq_pos);
            let id = id_part.trim().to_string();
            let rest = rest[1..].trim(); // skip '='

            // Find entity type, e.g. `IFCWALL(...)`
            if let Some(paren_pos) = rest.find('(') {
                let entity_type = rest[..paren_pos].trim().to_string();
                let raw_data = rest.to_string();

                entities.push(IfcEntity {
                    id,
                    entity_type,
                    raw_data,
                });
            }
        }
    }

    entities
}
