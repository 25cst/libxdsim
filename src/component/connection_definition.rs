/// Information for a Connection
pub struct ConnectionDefinition {
    /// Definition schema version number
    version: u32,
    
    /// Data type the connection carries
    data_type: &'static str,

    /// Connection identifier: the unique identifier for the connection type
    /// filled in by macro
    identifier: &'static str, // e.g. package_name-0.1.0::wire
}
