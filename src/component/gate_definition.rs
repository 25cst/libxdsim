pub struct GateDefinition {
    version: u32,
    input_positions: Vec<(f64, f64)>,
    output_positions: Vec<(f64, f64)>,
    bounding_rect: (f64, f64),  // size, bottomleft defined to be 0, 0
}
