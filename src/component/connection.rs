use crate::graphics::Graphic;

pub trait Connection<D> {
    fn draw(state: D) -> Graphic;
}
