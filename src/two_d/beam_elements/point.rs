use super::super::Vector;
use super::{Beam, BoundaryCondition};
use std::cell::RefCell;
use std::rc::Weak;

#[derive(Debug)]
pub struct Point {
    pub id: usize,
    pub pos: Vector,
    pub bc: BoundaryCondition,
    pub beams: Vec<Weak<RefCell<Beam>>>,
}
