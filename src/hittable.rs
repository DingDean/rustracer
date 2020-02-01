use crate::{ray::Ray, util, vec3::Vec3};

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub t: f64,
    /// the intersection vector
    pub p: Vec3,
    /// normal
    pub n: Vec3,
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(x: f64, y: f64, z: f64, r: f64) -> Sphere {
        Sphere {
            center: Vec3::new(x, y, z),
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.squared_length();
        let b = 2.0 * r.direction.dot(oc);
        let c = oc.squared_length() - self.radius * self.radius;

        let descriminant = b * b - 4.0 * a * c;
        if !(descriminant > 0.0) {
            None
        } else {
            let maybe_t = {
                let t1 = (-b - descriminant.sqrt()) / (2.0 * a);
                let t2 = (-b + descriminant.sqrt()) / (2.0 * a);
                if util::is_between(t1, t_min, t_max) {
                    Some(t1)
                } else if util::is_between(t2, t_min, t_max) {
                    Some(t2)
                } else {
                    None
                }
            };
            match maybe_t {
                Some(t) => {
                    let p = r.point_at_parameter(t);
                    let n = (p - self.center) / self.radius;
                    Some(HitRecord { t, p, n })
                }
                None => None,
            }
        }
    }
}

pub struct World {
    meshes: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> World {
        World { meshes: vec![] }
    }

    pub fn add(&mut self, mesh: Box<dyn Hittable>) {
        self.meshes.push(mesh);
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut record: Option<HitRecord> = None;

        for mesh in &self.meshes {
            if let Some(temp) = mesh.hit(r, t_min, closest_so_far) {
                closest_so_far = temp.t;
                record = Some(temp);
            }
        }
        record
    }
}
