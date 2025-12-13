use crate::register_package_def;

#[repr(C)]
pub struct PackageDefinitionV0 {
    pub ident: (&'static str, u16, u16, u16), // package name, semver major, semver minor, semver patch
    pub component_type: ComponentType,

    pub authors: &'static [&'static str],
    pub description: &'static str,
    pub homepage: &'static str,
}

#[repr(C)]
pub enum ComponentType {
    Gate,
    Connection,
    Data,
}

register_package_def!(PackageDefinitionV0, 0);

#[macro_export]
macro_rules! package_ident_v0 {
    ($name: expr) => {
        (
            $name,
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        )
    };
}

#[macro_export]
macro_rules! component_ident_v0 {
    ($name: expr) => {
        (
            $name,
            env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
        )
    };
}
