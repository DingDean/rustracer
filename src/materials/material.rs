use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Materialable: Send + Sync {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> Option<Scatter>;
}

