use crate::graphics::{Vec2, style::*};

pub enum Element {
    Line {
        points: Box<[Vec2]>,
        stroke: StrokeStyle,
    },
    Rect {
        pos: Vec2,
        size: Vec2,
        stroke: StrokeStyle,
        fill: FillStyle,
    },
}
