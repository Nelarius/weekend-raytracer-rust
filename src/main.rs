mod camera;
mod hitable;
mod ray;
mod vec3;

use camera::Camera;
use hitable::{Hitable, Sphere, World};
use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use ray::Ray;
use vec3::Vec3;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const NUM_SAMPLES: i32 = 32;

fn random_in_unit_sphere() -> Vec3 {
    // TODO random numbers :/
    Vec3::new(1.0, 0.0, 0.0)
}

fn color(r: &Ray, world: &World) -> Vec3 {
    // TODO: how to get max float in rust??
    if let Some(hit) = world.hit(&r, 0.0, 10000.0) {
        Vec3::new(
            0.5 * (hit.n.x + 1.0),
            0.5 * (hit.n.y + 1.0),
            0.5 * (hit.n.z + 1.0),
        )
    } else {
        let unit_direction = r.direction.make_unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
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
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let world = World::new(spheres);

    // 640 by 320
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),    // eye
        Vec3::new(-1.6, -0.8, -1.0), // lower left corner
        Vec3::new(3.2, 0.0, 0.0),    // horizontal
        Vec3::new(0.0, 1.6, 0.0),    // vertical
    );

    let mut rng = rand::thread_rng();

    for j in 0..WIDTH {
        for i in 0..HEIGHT {
            // TODO: add some actual random noise to the samples...
            let mut c = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..NUM_SAMPLES {
                let u = ((j as f32) + rng.gen::<f32>()) / (WIDTH as f32);
                let v = ((i as f32) + rng.gen::<f32>()) / (HEIGHT as f32);
                let r = camera.make_ray(u, v);
                c += color(&r, &world);
            }
            c = (1.0 / NUM_SAMPLES as f32) * c;

            let ir = (255.99 * c.x) as u32;
            let ig = (255.99 * c.y) as u32;
            let ib = (255.99 * c.z) as u32;

            buffer[to_buffer_index(i, j, WIDTH, HEIGHT)] = to_bgra(ir, ig, ib);
        }
    }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way.
        window.update_with_buffer(buffer.as_slice()).unwrap();
    }
}
