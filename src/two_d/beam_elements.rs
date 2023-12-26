mod beam;
mod cross_section;
mod world;

pub use beam::*;
pub use cross_section::*;
pub use world::*;

use super::Vector;
use crate::Material;

#[derive(Debug)]
pub struct Point {
    pub id: usize,
    pub pos: Vector,
    pub bc: BoundaryCondition,
    pub beams: Vec<usize>,
}

#[derive(Debug)]
pub enum BoundaryCondition {
    Free,
    Fixed,
    Force(Vector),
}
