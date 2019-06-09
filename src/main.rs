mod camera;
mod hitable;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hitable::{Hitable, Sphere, World};
use material::Material;
use minifb::{Key, Window, WindowOptions};
use rand::prelude::*;
use ray::Ray;
use std::sync::{Arc, Mutex};
use std::thread;
use vec3::Vec3;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const NUM_THREADS: usize = 16;
const NUM_SAMPLES: u32 = 16;
const MAX_DEPTH: u32 = 8;

fn color(r: Ray, world: &World, rng: &mut ThreadRng, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(&r, 0.001, std::f32::MAX) {
        if depth < MAX_DEPTH {
            let scatter = match hit.material {
                Material::Dielectric(d) => d.scatter(r, hit, rng),
                Material::Lambertian(l) => l.scatter(r, hit, rng),
                Material::Metal(m) => m.scatter(r, hit, rng),
            };
            return scatter.attenuation * color(scatter.ray, world, rng, depth + 1);
        } else {
            return Vec3::zeros();
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

    let world = Arc::new(World::new(spheres));

    let buffer = {
        let buffer: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(vec![0; WIDTH * HEIGHT]));

        let num_pixels = WIDTH * HEIGHT;

        let chunk_size = {
            let n = num_pixels / NUM_THREADS;
            let rem = num_pixels % NUM_THREADS;
            if rem > 0 {
                n + 1
            } else {
                n
            }
        };

        let mut tasks = Vec::new();

        for ithread in 0..NUM_THREADS {
            let world_ref = world.clone();
            let buffer_ref = buffer.clone();

            tasks.push(thread::spawn(move || {
                let start_index = ithread * chunk_size;
                let end_index = std::cmp::min(start_index + chunk_size, num_pixels);
                let mut rng = thread_rng();
                let mut local_pixels: Vec<u32> = Vec::with_capacity(end_index - start_index);

                for k in start_index..end_index {
                    let mut c = Vec3::new(0.0, 0.0, 0.0);
                    let i = k / WIDTH;
                    let j = k % WIDTH;
                    for _ in 0..NUM_SAMPLES {
                        let u = ((j as f32) + rng.gen::<f32>()) / (WIDTH as f32);
                        let v = ((i as f32) + rng.gen::<f32>()) / (HEIGHT as f32);
                        let r = camera.make_ray(&mut rng, u, v);
                        c += color(r, &world_ref, &mut rng, 0);
                    }
                    c = (1.0 / NUM_SAMPLES as f32) * c;
                    // uses gamma corrected color values
                    // gamma correction uses factor 1/2
                    let ir = (255.99 * c.x.sqrt()) as u32;
                    let ig = (255.99 * c.y.sqrt()) as u32;
                    let ib = (255.99 * c.z.sqrt()) as u32;

                    local_pixels.push(to_bgra(ir, ig, ib));
                }

                let mut buffer = buffer_ref.lock().unwrap();

                for (pos, pixel) in local_pixels.iter().enumerate() {
                    let k = start_index + pos;
                    let i = k / WIDTH;
                    let j = k % WIDTH;

                    buffer[to_buffer_index(i, j, WIDTH, HEIGHT)] = *pixel;
                }
            }));
        }

        for task in tasks {
            task.join().expect("Unable to join thread");
        }

        let buffer = Arc::try_unwrap(buffer).expect("The lock still has multiple owners");
        buffer.into_inner().expect("Locking the mutex failed")
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // We unwrap here as we want this code to exit if it fails.
        // Real applications may want to handle this in a different way.
        window.update_with_buffer(buffer.as_slice()).unwrap();
    }
}
