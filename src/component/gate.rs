use crate::graphics::*;

pub trait Gate {
    /// - &mut self: the gate is allowed to change its state when received an input
    /// - input: list of pointers to inputs in the order they are defined in definition()
    /// - output: list of pointers to outputs in the order they are defined in definition()
    fn tick(&mut self, input: GateTickRequest) -> Vec<*const u8>;

    /// Produce a graphic of the gate
    // request will be a reference, because the same gate will be drawn many times
    // with the same request
    fn draw(&self, request: &GateDrawRequest) -> Graphic;

    // DEFINITION: Returns gate definition
    // This takes a self reference because the definition may depend on options (which is stored
    // as state)
    // below are possible implementations
    // 1. allow access to a self reference, but this will also allow it to access its state
    //    but definition should not depend on state
    // fn definition(&self) -> GateDefinition;
    //
    // 2. definition should only depend on the options, so pass in only the option as bytes and do
    //    the transmut trick on it again
    // fn definition(option_bytes: *const u8) -> GateDefinition;

    // TODO: get_option, set_options, etc. How do we do this without making a complete mess?
}

/// A single gate tick request
pub struct GateTickRequest {
    /// Inputs to the gate
    inputs: Vec<*const u8>
}

/// A single gate draw request
pub struct GateDrawRequest {
    /// One of the four the gate is facing (rotation)
    direction: Direction,
    /// The size of the bounding box previously provided
    dimension: Vec2,
}
