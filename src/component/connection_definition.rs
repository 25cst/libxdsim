/// Information for a Connection
pub struct ConnectionDefinition {
    /// libxdsim version number
    version: u32,
    
    /// Data type the connection carries
    data: String,
    /// Name of the connection implementation
    flavour: String,
}
