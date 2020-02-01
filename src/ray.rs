use super::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Ray {
        Ray {
            origin: a,
            direction: b,
        }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
