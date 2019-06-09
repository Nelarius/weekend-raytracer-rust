use crate::camera::Camera;
use crate::hitable::{Hitable, World};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

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

pub fn render(width: usize, height: usize, camera: Camera, world: World) -> Vec<u32> {
    let world = Arc::new(world);
    let buffer: Arc<Mutex<Vec<u32>>> = Arc::new(Mutex::new(vec![0; width * height]));

    let num_pixels = width * height;

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
        let buffer_ref = buffer.clone();
        let world_ref = world.clone();

        tasks.push(thread::spawn(move || {
            let start_index = ithread * chunk_size;
            let end_index = std::cmp::min(start_index + chunk_size, num_pixels);
            let mut rng = thread_rng();
            let mut local_pixels: Vec<u32> = Vec::with_capacity(end_index - start_index);

            for k in start_index..end_index {
                let mut c = Vec3::new(0.0, 0.0, 0.0);
                let i = k / width;
                let j = k % width;
                for _ in 0..NUM_SAMPLES {
                    let u = ((j as f32) + rng.gen::<f32>()) / (width as f32);
                    let v = ((i as f32) + rng.gen::<f32>()) / (height as f32);
                    let r = camera.make_ray(&mut rng, u, v);
                    c += color(r, &world_ref, &mut rng, 0);
                }
                c = (1.0 / NUM_SAMPLES as f32) * c;
                let ir = (255.99 * c.x.sqrt()) as u32;
                let ig = (255.99 * c.y.sqrt()) as u32;
                let ib = (255.99 * c.z.sqrt()) as u32;

                local_pixels.push(to_bgra(ir, ig, ib));
            }

            let mut buffer = buffer_ref.lock().unwrap();

            for (pos, pixel) in local_pixels.iter().enumerate() {
                let k = start_index + pos;
                let i = k / width;
                let j = k % width;

                buffer[to_buffer_index(i, j, width, height)] = *pixel;
            }
        }));
    }

    for task in tasks {
        task.join().expect("Unable to join thread");
    }

    let buffer = Arc::try_unwrap(buffer).expect("The lock still has multiple owners");
    buffer.into_inner().expect("Locking the mutex failed")
}
