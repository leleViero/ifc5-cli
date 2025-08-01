/// A raw parsed IFC entity from a STEP file line.
/// Example line: `#123= IFCWALL(...);`
#[derive(Debug, Clone)]
pub struct IfcEntity {
    pub id: String,           // "#123"
    pub entity_type: String,  // "IFCWALL"
    pub raw_data: String,     // remaining of the input IFC line... between ( and );
}