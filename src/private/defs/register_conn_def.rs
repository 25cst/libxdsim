#[macro_export]
macro_rules! register_conn_def {
    ($name: ident, $version: expr) => {
        impl crate::private::defs::Sealed for $name {}
        impl crate::global::defs::ConnectionDefinition for $name {
            fn schema_version(&self) -> u32 {
                $version
            }
        }
    };
}
