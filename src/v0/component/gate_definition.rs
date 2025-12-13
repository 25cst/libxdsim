use crate::{register_gate_def, v0::graphics::Vec2};

/// TODO: this will need to have a stable byte structure
/// probably need to tag repr(C) or something
/// I need to read the nomicon
#[repr(C)]
pub struct GateDefinitionV0 {
    /// The ordered input that the gate takes
    pub inputs: Box<[GateIOEntry]>,
    /// The ordered output that the gate produces
    pub outputs: Box<[GateIOEntry]>,

    /// The visual bounding box (dimension) of the gate
    /// The bottom left corner is (0, 0), top right corner is (width, height)
    pub bounding_box: Vec2,

    /// Gate identifier: the unique identifier for the gate type
    /// filled in by macro
    pub identifier: (&'static str, u16, u16), // e.g. (package_name, gate-name, semver major, semver minor)
}

/// Representing a single input or output connection that the gate take.
/// - name: the unique name of the input/output
/// - data_type: the type name of the input/output
/// - position: a point that is on the bounding box
#[repr(C)]
pub struct GateIOEntry {
    pub name: Box<str>,
    pub data_type: (&'static str, u16, u16), // e.g. (package_name, gate-name, semver major, semver minor)
    pub position: Vec2,
}

register_gate_def!(GateDefinitionV0, 0);
