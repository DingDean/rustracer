use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;

use super::material::{Materialable, Scatter};

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(a: f64, b: f64, c: f64) -> Box<Lambertian> {
        Box::new(Lambertian {
            albedo: Vec3::new(a, b, c),
        })
    }
}

impl Materialable for Lambertian {
    fn scatter(&self, _r: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let target = hit_record.p + hit_record.n + util::random_in_unit_sphere();
        Some(Scatter {
            ray: Ray::new(hit_record.p, target - hit_record.p),
            attenuation: self.albedo,
        })
    }
}
