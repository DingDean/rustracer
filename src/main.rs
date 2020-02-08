mod camera;
mod hittable;
mod materials;
mod ray;
mod util;
mod vec3;

use camera::Camera;
use hittable::{Sphere, World};
use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use ray::Ray;
use vec3::Vec3;

const WIDTH: usize = 200;
const HEIGHT: usize = 100;

fn color(r: &Ray, world: &World, depth: u32) -> Vec3 {
    if let Some(record) = world.hit(r, 0.001, std::f64::MAX) {
        if depth >= 50 {
            return Vec3::zeros();
        }
        if let Some(scatter) = record.material.scatter(r, &record) {
            scatter.attenuation * color(&scatter.ray, world, depth + 1)
        } else {
            Vec3::zeros()
        }
    } else {
        // blue background
        let unit_direction = r.direction.make_unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        util::lerp(Vec3::all(1.0), Vec3::new(0.5, 0.7, 1.0), t)
    }
}

fn main() {
    let mut buffer: Vec<u32> = Vec::new();

    let mut window = Window::new(
        "Test - EST to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut world = World::new();
    world.add(Box::new(Sphere::new(
        0.0,
        0.0,
        -1.0,
        0.5,
        materials::Lambertian::new(0.8, 0.3, 0.3),
    )));
    world.add(Box::new(Sphere::new(
        0.0,
        -100.5,
        -1.0,
        100.0,
        materials::Lambertian::new(0.8, 0.8, 0.0),
    )));

    world.add(Box::new(Sphere::new(
        1.0,
        0.0,
        -1.0,
        0.5,
        materials::Metal::new(0.8, 0.6, 0.2, 0.3),
    )));

    world.add(Box::new(Sphere::new(
        -1.0,
        0.0,
        -1.0,
        0.5,
        materials::Dielectrics::new(1.5),
    )));

    let camera = Camera::default();
    let mut rng = thread_rng();
    let ns = 100;

    for j in (0..HEIGHT).rev() {
        for i in 0..WIDTH {
            let mut c = Vec3::zeros();
            for _ in 0..ns {
                let v: f64 = (j as f64 + rng.gen_range(0.0, 1.0)) / HEIGHT as f64;
                let u: f64 = (i as f64 + rng.gen_range(0.0, 1.0)) / WIDTH as f64;
                let ray = camera.get_ray(u, v);
                let lc = color(&ray, &world, 0);
                c += lc;
            }
            c /= ns as f64;
            // gamma correction
            c = Vec3::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt());
            buffer.push(util::from_u8_rgb(c));
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
