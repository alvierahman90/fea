use super::*;

#[derive(Debug)]
pub struct Beam{
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
        let b = Beam {
            id: 0,
            points: (p1, p2),
            material: props.material,
            cross_section: props.cross_section,
        };

        return b;
    }

    fn get_points<'a>(&self, world: &'a World) -> (&'a Point, &'a Point) {
        let p1 = world.points.get(&self.points.0).unwrap();
        let p2 = world.points.get(&self.points.1).unwrap();

        (p1, p2)
    }

    pub fn stiffness(&self, world: &World) -> f32 {
        let s = self.cross_section.area() * self.material.youngs_modulus / self.length(world);

        println!("{:?} {}", self, s);

        s

    }

    pub fn length(&self, world: &World) -> f32 {
        let (p1, p2) = self.get_points(world);
        let l = p1.pos.distance(&p2.pos);

        println!("{:?} {}", self, l);

        l
    }

    pub fn angle(&self, world: &World) -> f32 {
        let (p1, p2) = self.get_points(world);
        let dx = p1.pos.0 - p2.pos.0;
        let dy = p1.pos.1 - p2.pos.1;

        (dy/dx).atan()
    }

    pub fn other_point(&self, p: usize) -> usize {
        if p == self.points.0{
            return self.points.1;
        } else {
            return self.points.0;
        }
    }
}
