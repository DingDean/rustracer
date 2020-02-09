use crate::vec3::Vec3;
use rand::prelude::*;

pub fn is_between(t: f64, min: f64, max: f64) -> bool {
    t < max && t > min
}

pub fn random_double() -> f64 {
    let mut rng: ThreadRng = thread_rng();
    let x: f64 = rng.gen_range(0.0, 1.0);
    x
}

pub fn random_in_unit_disk() -> Vec3 {
    let mut rng: ThreadRng = thread_rng();
    loop {
        let x: f64 = rng.gen_range(0.0, 1.0);
        let y: f64 = rng.gen_range(0.0, 1.0);
        let z: f64 = 0.0;
        let p = 2.0 * Vec3::new(x, y, z) - Vec3::new(1.0, 1.0, 0.0);
        if p.length() < 1.0 {
            break p;
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut rng: ThreadRng = thread_rng();
    loop {
        let x: f64 = rng.gen_range(0.0, 1.0);
        let y: f64 = rng.gen_range(0.0, 1.0);
        let z: f64 = rng.gen_range(0.0, 1.0);
        let p = 2.0 * Vec3::new(x, y, z) - Vec3::all(1.0);
        if p.length() < 1.0 {
            break p;
        }
    }
}

pub fn from_u8_rgb(rgb: Vec3) -> u32 {
    let r = (255.0 * rgb.x) as u32;
    let g = (255.0 * rgb.y) as u32;
    let b = (255.0 * rgb.z) as u32;
    (r << 16) | (g << 8) | b
}

pub fn lerp(from: Vec3, to: Vec3, t: f64) -> Vec3 {
    (1.0 - t) * from + t * to
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

/// Compute the refracted ray based on incident ray `v` and the normal `n`
/// where `ni_over_nt` is the ratio of reflected indexes
pub fn refract(v: Vec3, n: Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.make_unit_vector();
    let dt = uv.dot(n);
    let descriminant = 1.0 - ni_over_nt * (1.0 - dt * dt);
    if descriminant > 0.0 {
        Some(ni_over_nt * (uv - dt * n) - descriminant.sqrt() * n)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_between_test() {
        assert!(is_between(1.0, 0.0, 2.0));
    }

    #[test]
    fn reflect_test() {
        let incoming_ray = Vec3::new(1.0, -1.0, 0.0);
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let reflected_ray = reflect(incoming_ray, normal);
        assert_eq!(reflected_ray, Vec3::new(1.0, 1.0, 0.0));
    }
}
