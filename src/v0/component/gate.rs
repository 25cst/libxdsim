use crate::{
    defs::GateDefinition,
    v0::{app_state::PropertiesContainer, component::Type, graphics::*},
};

pub trait Gate {
    /// - &mut self: the gate is allowed to change its state when received an input
    /// - input: list of pointers to inputs in the order they are defined in definition()
    /// - output: list of pointers to outputs in the order they are defined in definition()
    fn tick(&mut self, request: GateTickRequest) -> Box<[Box<dyn Type>]>;

    /// Produce a graphic of the gate
    // request will be a reference, because the same gate will be drawn many times
    // with the same request
    fn draw(&self, request: &GateDrawRequest) -> Graphic;

    // DEFINITION: Returns gate definition
    // This takes a self reference because the definition may depend on options (which is stored
    // as state)
    fn definition(&self) -> Box<dyn GateDefinition>;

    /// Get the property container
    /// this is to be implemented by macro
    fn properties_container(&self) -> &dyn PropertiesContainer;

    /// Get the property container (mutable)
    /// this is to be implemented by macro
    fn properties_container_mut(&mut self) -> &mut dyn PropertiesContainer;

    /// Serialize gate into bytes
    fn serialize(&self) -> Box<[u8]>;
}

/// A single gate tick request
#[repr(C)]
pub struct GateTickRequest<'a> {
    /// Inputs to the gate
    pub inputs: Box<[&'a dyn Type]>,
}

impl<'a> GateTickRequest<'a> {
    pub fn get_input<T>(&self, index: u32) -> &'a T {
        let ptr: *const dyn Type = &*self.inputs[index as usize];
        let ptr: *const T = ptr as *const T;
        unsafe { &*ptr }
    }
}

/// A single gate draw request
#[repr(C)]
pub struct GateDrawRequest {
    /// One of the four the gate is facing (rotation)
    pub direction: Direction,
    /// The size of the bounding box previously provided
    pub dimension: Vec2,
}
