mod camera;
mod hittable;
mod materials;
mod ray;
mod scenes;
mod util;
mod vec3;

use camera::Camera;
use hittable::World;
use minifb::{Key, Window, WindowOptions};
use num_cpus;
use rand::prelude::*;
use ray::Ray;
use std::sync::Arc;
use std::time;
use vec3::Vec3;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn color(r: &Ray, world: &World, depth: u32, rng: &mut ThreadRng) -> Vec3 {
    if let Some(record) = world.hit(r, 0.001, std::f64::MAX) {
        if depth >= 50 {
            return Vec3::zeros();
        }
        if let Some(scatter) = record.material.scatter(r, &record, rng) {
            scatter.attenuation * color(&scatter.ray, world, depth + 1, rng)
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
    let timer = time::Instant::now();
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

    // let world = scenes::simple::load();
    let world = scenes::sphere_sea::load();

    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let eye = Vec3::new(13.0, 2.0, 3.0);
    // let eye = Vec3::zeros();
    let lookto = Vec3::new(0.0, 0.0, 0.0);
    // let focus_dist = (eye - lookto).length();
    let camera = Camera::new(
        eye,
        lookto,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.5,
        10.0,
    );

    // split the image into num_rows rows each with height row_height
    let (row_height, num_row) = {
        let num_cores = num_cpus::get();
        let threads_per_core = 2;
        let num_row = num_cores * threads_per_core;
        let row_height = HEIGHT / num_row;
        (row_height, num_row)
    };
    println!(
        "The image would be rendered with {} threads each with height {}.",
        num_row, row_height
    );

    let (sender, receiver) = std::sync::mpsc::channel();
    let world_ptr = Arc::new(world);
    let camera_ptr = Arc::new(camera);
    for i in 0..num_row {
        let upper_bound = HEIGHT - i * row_height;
        let lower_bound = {
            let y = HEIGHT - (i + 1) * row_height;
            if y < row_height {
                0
            } else {
                y
            }
        };

        let sx = sender.clone();
        let world_arc = world_ptr.clone();
        let camera_arc = camera_ptr.clone();
        let ns = 100;
        let task_number = i;
        std::thread::spawn(move || {
            let mut buffer: Vec<u32> = Vec::new();
            let mut rng = thread_rng();
            for j in (lower_bound..upper_bound).rev() {
                for i in 0..WIDTH {
                    let mut c = Vec3::zeros();
                    for _ in 0..ns {
                        let v: f64 = (j as f64 + rng.gen_range(0.0, 1.0)) / HEIGHT as f64;
                        let u: f64 = (i as f64 + rng.gen_range(0.0, 1.0)) / WIDTH as f64;
                        let ray = camera_arc.get_ray(u, v, &mut rng);
                        let lc = color(&ray, &world_arc, 0, &mut rng);
                        c += lc;
                    }
                    c /= ns as f64;
                    // gamma correction
                    c = Vec3::new(c.x.sqrt(), c.y.sqrt(), c.z.sqrt());
                    buffer.push(util::from_u8_rgb(c));
                }
            }
            sx.send((task_number, buffer)).unwrap();
        });
    }

    let mut buffs: Vec<Vec<u32>> = vec![vec![]; num_row];
    for (i, buf) in receiver.iter().take(num_row) {
        buffs[i] = buf;
    }

    let buffer = buffs.concat();
    println!("Image rendered in {}s", timer.elapsed().as_secs());

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
