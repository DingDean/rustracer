use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;

use super::material::{Materialable, Scatter};

pub struct Dielectrics {
    /// reflected index
    ref_index: f64,
}

impl Dielectrics {
    pub fn new(ref_index: f64) -> Box<Dielectrics> {
        Box::new(Dielectrics { ref_index })
    }
}

fn schlick(cosine: f64, ref_index: f64) -> f64 {
    let mut r0 = (1.0 - ref_index) / (1.0 + ref_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Materialable for Dielectrics {
    fn scatter(&self, r: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let (normal, ni_over_nt, cosine) = {
            if r.direction.dot(hit.n) > 0.0 {
                (
                    -hit.n,
                    self.ref_index,
                    self.ref_index * r.direction.dot(hit.n) / r.direction.length(),
                )
            } else {
                (
                    hit.n,
                    1.0 / self.ref_index,
                    -r.direction.dot(hit.n) / r.direction.length(),
                )
            }
        };

        let reflected = Some(Scatter {
            ray: Ray::new(hit.p, util::reflect(r.direction, normal)),
            attenuation: Vec3::all(1.0),
        });

        let (refracted, prob) =
            if let Some(refract) = util::refract(r.direction, normal, ni_over_nt) {
                (
                    Some(Scatter {
                        ray: Ray::new(hit.p, refract),
                        attenuation: Vec3::all(1.0),
                    }),
                    schlick(cosine, self.ref_index),
                )
            } else {
                (None, 1.0)
            };

        if util::random_double() < prob {
            reflected
        } else {
            refracted
        }
    }
}
