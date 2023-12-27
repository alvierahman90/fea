pub mod two_d;

#[derive(Debug)]
pub struct Material {
    pub youngs_modulus: f32,
    pub yield_stress: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Unknown,
    Known(f32),
}
