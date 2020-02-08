use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Materialable {
    fn scatter(&self, r: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

