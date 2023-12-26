use std::f32::consts::PI;

#[derive(Debug)]
pub enum CrossSection {
    Area(f32),
    Circular(f32),
    Rectangular(f32, f32),
}

impl CrossSection {
    pub fn area(&self) -> f32 {
        let a = match self {
            Self::Area(a) => *a,
            Self::Circular(r) => PI * r.powi(2),
            Self::Rectangular(l1, l2) => l1 * l2,
        };
        println!("{:?} {}", self, a);

        a
    }
}
