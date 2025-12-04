use crate::graphics::element::Element;

pub struct Graphic {
    elements: Vec<Element>
}

// TODO: if we implement this, would it make it very hard
// to implement it in other languages (e.g. C)
impl Default for Graphic {
    fn default() -> Self {
        Self {
            elements: Vec::new()
        }
    }
}
