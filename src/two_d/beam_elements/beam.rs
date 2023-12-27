use super::*;
use crate::Material;

#[derive(Debug)]
pub struct Beam {
    pub id: usize,
    pub points: (usize, usize),
    pub material: Material,
    pub cross_section: CrossSection,
}

pub struct NewBeam {
    pub material: Material,
    pub cross_section: CrossSection,
}

impl Beam {
    pub fn new(p1: usize, p2: usize, props: NewBeam) -> Beam {
        Beam {
            id: 0,
            points: (p1, p2),
            material: props.material,
            cross_section: props.cross_section,
        }
    }

    fn get_points<'a>(&self, world: &'a World) -> (&'a Point, &'a Point) {
        let p1 = world.points.get(&self.points.0).unwrap();
        let p2 = world.points.get(&self.points.1).unwrap();

        (p1, p2)
    }

    pub fn stiffness(&self, world: &World) -> f32 {
        self.cross_section.area() * self.material.youngs_modulus / self.length(world)
    }

    pub fn length(&self, world: &World) -> f32 {
        let (p1, p2) = self.get_points(world);
        p1.pos.distance(&p2.pos)
    }

    pub fn angle(&self, world: &World) -> f32 {
        let (p1, p2) = self.get_points(world);
        let dx = p1.pos.0 - p2.pos.0;
        let dy = p1.pos.1 - p2.pos.1;

        (dy / dx).atan()
    }

    pub fn other_point(&self, p: usize) -> usize {
        if p == self.points.0 {
            self.points.1
        } else {
            self.points.0
        }
    }
}
