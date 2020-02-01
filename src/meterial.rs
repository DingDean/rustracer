use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scatter {
    ray: Ray,
    attenuation: Vec3,
}

pub trait Materialable {
    fn scatter(&self, r: &Ray, hit_records: Vec<HitRecord>) -> Option<Scatter>;
}
