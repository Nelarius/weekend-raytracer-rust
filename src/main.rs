mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hitable::{Hitable, MaterialRecord, Sphere, World};
use material::{Dielectric, Lambertian, Material, Metal};
use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use ray::Ray;
use vec3::Vec3;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const NUM_SAMPLES: i32 = 32;
const MAX_DEPTH: i32 = 50;

fn color(r: Ray, world: &World, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(&r, 0.001, std::f32::MAX) {
        if depth < MAX_DEPTH {
            let scatter = match hit.material {
                MaterialRecord::Dielectric(d) => d.scatter(&r, &hit),
                MaterialRecord::Lambertian(l) => l.scatter(&r, &hit),
                MaterialRecord::Metal(m) => m.scatter(&r, &hit),
            };
            return if let Some(s) = scatter {
                s.attenuation * color(s.ray, &world, depth + 1)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            };
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = r.direction.make_unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn to_bgra(r: u32, g: u32, b: u32) -> u32 {
    255 << 24 | r << 16 | g << 8 | b
}

fn to_buffer_index(i: usize, j: usize, width: usize, height: usize) -> usize {
    // flip height from bottom to top
    ((height - 1 - i) * width) + j
}

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

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // list of spheres
    let spheres = vec![
        Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            MaterialRecord::Lambertian(Lambertian {
                albedo: Vec3::new(0.1, 0.2, 0.6),
            }),
        ),
        Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            MaterialRecord::Lambertian(Lambertian {
                albedo: Vec3::new(0.8, 0.8, 0.0),
            }),
        ),
        Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            MaterialRecord::Metal(Metal {
                albedo: Vec3::new(0.8, 0.6, 0.2),
                fuzz: 0.2,
            }),
        ),
        Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            MaterialRecord::Dielectric(Dielectric {
                refraction_index: 1.5,
            }),
        ),
        Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.48,
            MaterialRecord::Dielectric(Dielectric {
                refraction_index: 1.5,
            }),
        ),
    ];

    let lookfrom = Vec3::new(-2.0, 1.0, 1.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let aspect_ratio = (WIDTH as f32) / (HEIGHT as f32);
    // let camera = Camera::new(90.0, aspect_ratio);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),     // vup
        30.0,                         // vfov
        aspect_ratio,                 // aspect ratio
        1.0,                          // aperture
        (lookfrom - lookat).length(), // focust distance
    );

    let world = World::new(spheres);

    let mut rng = rand::thread_rng();

    for j in 0..WIDTH {
        for i in 0..HEIGHT {
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..NUM_SAMPLES {
                let u = ((j as f32) + rng.gen::<f32>()) / (WIDTH as f32);
                let v = ((i as f32) + rng.gen::<f32>()) / (HEIGHT as f32);
                let r = camera.make_ray(u, v);
                c += color(r, &world, 0);
            }
            c = (1.0 / NUM_SAMPLES as f32) * c;

            // uses gamma corrected color values
            // gamma correction uses factor 1/2
            let ir = (255.99 * c.x.sqrt()) as u32;
            let ig = (255.99 * c.y.sqrt()) as u32;
            let ib = (255.99 * c.z.sqrt()) as u32;

            buffer[to_buffer_index(i, j, WIDTH, HEIGHT)] = to_bgra(ir, ig, ib);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way.
        window.update_with_buffer(buffer.as_slice()).unwrap();
    }
}
