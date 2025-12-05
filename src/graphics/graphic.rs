use crate::graphics::element::Element;

// TODO: perhaps use a builder style way to create a graphic, like
// Graphic::default().line(...).circle(...)
// so we can compute the bounding box of the graphic
// which could be useful later on
pub struct Graphic {
    elements: Vec<Element>,
}

impl Default for Graphic {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}
