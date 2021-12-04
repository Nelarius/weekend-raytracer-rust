#![deny(
    clippy::pedantic,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style
)]

mod camera;
mod hitable;
mod material;
mod ray;
mod renderer;
mod vec3;

use camera::Camera;
use hitable::{Sphere, World};
use material::Material;
use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use vec3::Vec3;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn main() {
    println!("starting raytracing now!");
    let mut window = Window::new(
        "Raytracer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // hitable spheres
    let mut spheres = vec![
        Sphere::new(
            Vec3::new(0.0, -1000.0, -1.0),
            1000.0,
            Material::lambertian(Vec3::new(0.5, 0.5, 0.5)),
        ),
        Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Material::dielectric(1.5)),
        Sphere::new(
            Vec3::new(-4.0, 1.0, 0.0),
            1.0,
            Material::lambertian(Vec3::new(0.4, 0.2, 0.1)),
        ),
        Sphere::new(
            Vec3::new(4.0, 1.0, 0.0),
            1.0,
            Material::metal(Vec3::new(0.7, 0.6, 0.5), 0.0),
        ),
    ];

    let sphere_offset = Vec3::new(4.0, 0.2, 0.0);
    for a in -5..5 {
        for b in -5..5 {
            let a = a as f32;
            let b = b as f32;
            let center = Vec3::new(a + 0.9 * random::<f32>(), 0.2, b + 0.9 * random::<f32>());
            let choose_mat = random::<f32>();
            if (center - sphere_offset).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    spheres.push(Sphere::new(
                        center,
                        0.2,
                        Material::lambertian(Vec3::new(
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                            random::<f32>() * random::<f32>(),
                        )),
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    spheres.push(Sphere::new(
                        center,
                        0.2,
                        Material::metal(
                            Vec3::new(
                                random::<f32>() * random::<f32>(),
                                random::<f32>() * random::<f32>(),
                                random::<f32>() * random::<f32>(),
                            ),
                            0.5 * random::<f32>(),
                        ),
                    ))
                } else {
                    spheres.push(Sphere::new(center, 0.2, Material::dielectric(1.5)));
                }
            }
        }
    }

    let lookfrom = Vec3::new(16.0, 2.0, 4.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let aspect_ratio = (WIDTH as f32) / (HEIGHT as f32);

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),     // vup
        15.0,                         // vfov
        aspect_ratio,                 // aspect ratio
        0.2,                          // aperture
        (lookfrom - lookat).length(), // focus distance
    );

    let world = World::new(spheres);

    let buffer = renderer::render(WIDTH, HEIGHT, camera, world);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way.
        window.update_with_buffer(buffer.as_slice()).unwrap();
    }
}
