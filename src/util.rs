use crate::vec3::Vec3;
use rand::prelude::*;

pub fn is_between(t: f64, min: f64, max: f64) -> bool {
    t < max && t > min
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_between_test() {
        assert!(is_between(1.0, 0.0, 2.0));
    }
}
