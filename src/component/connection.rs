use crate::{component::connection_definition::ConnectionDefinition, graphics::Graphic};

pub trait Connection<D> {
    fn draw(state: D) -> Graphic;

    fn definition() -> ConnectionDefinition;
}
