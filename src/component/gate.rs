use crate::graphics::Graphic;

pub trait Gate<I, O> {
    fn tick(input: I) -> O; // TODO

    fn draw(direction: (), dimension: ()) -> Graphic;

    // fn definition()
}
