use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;
use rand::prelude::*;
use std::f64::consts::PI;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let lens_radius = aperture / 2.0;

        let theta = vfov * PI / 180.0;
        let half_height = (0.5 * theta).tan();
        let half_width = aspect * half_height;

        let w = (lookfrom - lookat).make_unit_vector();
        let u = vup.cross(w).make_unit_vector();
        let v = w.cross(u);

        let llc = lookfrom - focus_dist * (half_width * u + half_height * v + w);

        Camera {
            lower_left_corner: llc,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: lookfrom,
            u,
            v,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * util::random_in_unit_disk(rng);
        let offset = rd.x * self.u + rd.y * self.v;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
