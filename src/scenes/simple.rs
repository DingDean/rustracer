use crate::hittable::{Sphere, World};
use crate::materials;

pub fn load() -> World {
    let mut world = World::new();

    world.add(Sphere::new(
        0.0,
        0.0,
        -1.0,
        0.5,
        materials::Lambertian::new(0.8, 0.3, 0.3),
    ));
    world.add(Sphere::new(
        0.0,
        -1000.0,
        0.0,
        999.5,
        materials::Lambertian::new(0.46, 0.53, 0.54),
    ));

    world.add(Sphere::new(
        1.0,
        0.0,
        -1.0,
        0.5,
        materials::Metal::new(0.8, 0.6, 0.2, 0.3),
    ));

    world.add(Sphere::new(
        -1.0,
        0.0,
        -1.0,
        0.5,
        materials::Dielectrics::new(1.5),
    ));

    world
}
