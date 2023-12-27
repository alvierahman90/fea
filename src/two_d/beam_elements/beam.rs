use super::*;
use crate::Material;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct Beam {
    pub id: usize,
    pub points: (Weak<RefCell<Point>>, Weak<RefCell<Point>>),
    pub material: Material,
    pub cross_section: CrossSection,
}

pub struct NewBeam {
    pub material: Material,
    pub cross_section: CrossSection,
}

impl Beam {
    pub fn new(p1: Weak<RefCell<Point>>, p2: Weak<RefCell<Point>>, props: NewBeam) -> Beam {
        Beam {
            id: 0,
            points: (p1, p2),
            material: props.material,
            cross_section: props.cross_section,
        }
    }

    fn get_points(&self) -> (Rc<RefCell<Point>>, Rc<RefCell<Point>>) {
        (
            self.points.0.upgrade().unwrap(),
            self.points.1.upgrade().unwrap(),
        )
    }

    pub fn stiffness(&self) -> f32 {
        self.cross_section.area() * self.material.youngs_modulus / self.length()
    }

    pub fn length(&self) -> f32 {
        let (p1, p2) = self.get_points();
        let (p1, p2) = (p1.borrow(), p2.borrow());
        p1.pos.distance(&p2.pos)
    }

    pub fn angle(&self) -> f32 {
        let (p1, p2) = self.get_points();
        let (p1, p2) = (p1.borrow(), p2.borrow());
        let dx = p1.pos.0 - p2.pos.0;
        let dy = p1.pos.1 - p2.pos.1;

        (dy / dx).atan()
    }

    pub fn other_point(&self, p: &Point) -> Option<Rc<RefCell<Point>>> {
        let (p0, _) = self.get_points();
        if p.id == p0.borrow().id {
            self.points.1.upgrade()
        } else {
            self.points.0.upgrade()
        }
    }
}
