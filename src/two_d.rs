pub mod beam_elements;

#[derive(Debug)]
pub struct Vector(pub f32, pub f32);

impl Vector {
    pub fn distance(&self, other: &Self) -> f32 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }

    pub fn from_mag_angle(magnitude: f32, angle: f32) -> Self {
        Self (magnitude * angle.cos(), magnitude * angle.sin())
    }
}
