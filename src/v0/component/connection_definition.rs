use crate::register_conn_def;

/// Information for a Connection
#[repr(C)]
pub struct ConnectionDefinitionV0 {
    /// Definition schema version number
    pub version: u32,

    /// Data type the connection carries
    pub data_type: (&'static str, u16, u16), // e.g. (package_name, semver major, semver minor)

    /// Connection identifier: the unique identifier for the connection type
    /// filled in by macro
    pub identifier: (&'static str, u16, u16), // e.g. (package_name, semver major, semver minor)
}

register_conn_def!(ConnectionDefinitionV0, 0);
