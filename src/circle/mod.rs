
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::Coord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Circle<C: Coord> {
    pub center: C,
    pub radius: u32,
}

// https://github.com/YendorEngine/yendor/blob/main/crates/yendor_rl/src/geometry/shapes/circle.rs