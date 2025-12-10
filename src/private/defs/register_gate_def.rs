#[macro_export]
macro_rules! register_gate_def {
    ($name: ident, $version: expr) => {
        impl crate::private::defs::Sealed for $name {}
        impl crate::global::defs::GateDefinition for $name {
            fn schema_version(&self) -> u32 {
                $version
            }
        }
    };
}
