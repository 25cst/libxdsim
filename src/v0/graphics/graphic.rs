use crate::v0::graphics::element::Element;

// TODO: perhaps use a builder style way to create a graphic, like
// Graphic::default().line(...).circle(...)
// so we can compute the bounding box of the graphic
// which could be useful later on
#[repr(C)]
pub struct Graphic {
    elements: Box<[Element]>,
}

impl Graphic {
    pub fn from_vec(elements: Vec<Element>) -> Self {
        Self {
            elements: elements.into_boxed_slice(),
        }
    }
}

impl Default for Graphic {
    fn default() -> Self {
        Self {
            elements: Box::new([]),
        }
    }
}
