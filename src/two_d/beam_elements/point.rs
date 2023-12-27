use super::super::Vector;
use super::BoundaryCondition;

#[derive(Debug)]
pub struct Point {
    pub id: usize,
    pub pos: Vector,
    pub bc: BoundaryCondition,
    pub beams: Vec<usize>,
}
