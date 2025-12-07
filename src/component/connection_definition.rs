/// Information for a Connection
pub struct ConnectionDefinition {
    /// Definition schema version number
    pub version: u32,

    /// Data type the connection carries
    pub data_type: (&'static str, &'static str, u16, u16), // e.g. (package_name, gate-name, semver major, semver minor)

    /// Connection identifier: the unique identifier for the connection type
    /// filled in by macro
    pub identifier: (&'static str, &'static str, u16, u16), // e.g. (package_name, gate-name, semver major, semver minor)
}
