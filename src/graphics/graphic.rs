use crate::graphics::element::Element;

pub struct Graphic {
    elements: Vec<Element>
}

impl Default for Graphic {
    fn default() -> Self {
        Self {
            elements: Vec::new()
        }
    }
}
