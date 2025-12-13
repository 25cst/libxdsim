use std::collections::HashSet;

use crate::v0::graphics::*;

#[repr(C)]
pub enum ConnectionPath {
    Segment(Box<[Vec2]>),
    Junction(HashSet<ConnectionPath>), // TODO: this seems redundant as theres a max 3 outbound
                                       // connections to a junction
}
