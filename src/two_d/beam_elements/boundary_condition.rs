use crate::two_d::Vector;

#[derive(Debug)]
pub enum BoundaryCondition {
    Free,
    Fixed,
    Force(Vector),
}
