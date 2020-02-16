use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;

use super::material::{Materialable, Scatter};
use rand::prelude::*;

pub struct Metal {
    fuzzy: f64,
    albedo: Vec3,
}

impl Metal {
    pub fn new(a: f64, b: f64, c: f64, fuzzy: f64) -> Box<Metal> {
        Box::new(Metal {
            fuzzy: if fuzzy < 1.0 { fuzzy } else { 1.0 },
            albedo: Vec3::new(a, b, c),
        })
    }
}

impl Materialable for Metal {
    fn scatter(&self, r: &Ray, hit: &HitRecord, rng: &mut ThreadRng) -> Option<Scatter> {
        let v = r.direction.make_unit_vector();
        let reflected = util::reflect(v, hit.n);
        let ray = Ray::new(
            hit.p,
            reflected + self.fuzzy * util::random_in_unit_sphere(rng),
        );
        if ray.direction.dot(hit.n) > 0.0 {
            Some(Scatter {
                ray,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
