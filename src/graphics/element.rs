use crate::graphics::style::*;

pub enum Element {
    Line {
        points: Vec<(f64, f64)>,
        stroke: StrokeStyle,
    },
    Rect {
        pos: (f64, f64),
        size: (f64, f64),
        stroke: StrokeStyle,
        fill: FillStyle,
    },
}
