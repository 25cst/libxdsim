use crate::{component::connection_definition::ConnectionDefinition, graphics::Graphic};

/// A connection that can carry a value
/// It may (or not) necessarily be connected to anything
pub trait Connection {
    fn draw(data: *const u8) -> Graphic;
    fn definition() -> ConnectionDefinition;
}
