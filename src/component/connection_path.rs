use std::collections::HashSet;

use crate::graphics::*;

pub enum ConnectionPath {
    Segment(Box<[Vec2]>),
    Junction(HashSet<ConnectionPath>), // TODO: this seems redundant as theres a max 3 outbound
                                       // connections to a junction
}
