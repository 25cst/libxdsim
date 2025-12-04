use crate::graphics::Vec2;

/// TODO: this will need to have a stable byte structure
/// probably need to tag repr(C) or something
/// I need to read the nomicon
pub struct GateDefinition {
    /// Definition schema version number
    version: u32,

    /// The ordered input that the gate takes
    inputs: Vec<GateIOEntry>,
    /// The ordered output that the gate produces
    outputs: Vec<GateIOEntry>,

    /// The visual bounding box (dimension) of the gate
    /// The bottom left corner is (0, 0), top right corner is (width, height)
    bounding_box: Vec2,

    /// Gate identifier: the unique identifier for the gate type
    /// filled in by macro
    identifier: &'static str, // e.g. package_name-0.1.0::and
}

/// Representing a single input or output connection that the gate take.
/// - name: the unique name of the input/output
/// - data_type: the type name of the input/output
/// - (x_pos, y_pos): a finite f64 that is on the bounding box
pub struct GateIOEntry {
    name: String,
    data_type: &'static str,
    x_pos: f64,
    y_pos: f64,
}
