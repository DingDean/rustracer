use crate::hittable::{Sphere, World};
use crate::materials::{Dielectrics, Lambertian, Metal};
use crate::util::random_double as random;
use crate::vec3::Vec3;
use rand::prelude::*;

pub fn load() -> World {
    let mut world = World::new();

    // Add the ground
    world.add(Sphere::new(
        0.0,
        -1000.0,
        0.0,
        1000.0,
        Lambertian::new(0.5, 0.5, 0.5),
    ));
    // Add three large sphere
    world.add(Sphere::new(0.0, 1.0, 0.0, 1.0, Dielectrics::new(1.5)));
    world.add(Sphere::new(
        -4.0,
        1.0,
        0.0,
        1.0,
        Lambertian::new(0.4, 0.2, 0.1),
    ));
    world.add(Sphere::new(
        4.0,
        1.0,
        0.0,
        1.0,
        Metal::new(0.7, 0.6, 0.5, 0.0),
    ));

    let tp = Vec3::new(4.0, 0.2, 0.0);
    let mut rng = thread_rng();
    for i in -11..11 {
        for j in -11..11 {
            let random_mat = random(&mut rng);
            let center = Vec3::new(
                i as f64 + 0.9 * random(&mut rng),
                0.2,
                j as f64 + 0.9 * random(&mut rng),
            );
            if (center - tp).length() <= 0.9 {
                continue;
            }
            let mesh = if random_mat < 0.8 {
                Sphere::new(
                    center.x,
                    center.y,
                    center.z,
                    0.2,
                    Lambertian::new(
                        random(&mut rng) * random(&mut rng),
                        random(&mut rng) * random(&mut rng),
                        random(&mut rng) * random(&mut rng),
                    ),
                )
            } else if random_mat < 0.95 {
                Sphere::new(
                    center.x,
                    center.y,
                    center.z,
                    0.2,
                    Metal::new(
                        0.5 * (1.0 + random(&mut rng)),
                        0.5 * (1.0 + random(&mut rng)),
                        0.5 * (1.0 + random(&mut rng)),
                        0.5 * random(&mut rng),
                    ),
                )
            } else {
                Sphere::new(center.x, center.y, center.z, 0.2, Dielectrics::new(1.5))
            };

            world.add(mesh);
        }
    }

    world
}
