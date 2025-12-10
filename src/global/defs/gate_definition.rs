use std::any::Any;

use crate::private::defs::Sealed;

#[allow(private_bounds)]
pub trait GateDefinition: Sealed + Any {
    fn schema_version(&self) -> u32;
}
