use crate::graphics::*;
use super::gate_definition::GateDefinition;

pub trait Gate<I, O> {
    fn tick(input: I) -> O;

    fn draw(direction: Direction, size: Vec2) -> Graphic;

    fn definition() -> GateDefinition;
}
