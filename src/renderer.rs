use crate::camera::Camera;
use crate::hitable::World;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;
use scoped_threadpool::Pool;
use std::sync::Arc;

const NUM_THREADS: usize = 16;
const NUM_SAMPLES: u32 = 128;
const MAX_DEPTH: u32 = 16;

fn color(r: Ray, world: &World, rng: &mut ThreadRng, depth: u32) -> Vec3 {
    if let Some(hit) = world.hit(&r, 0.001, std::f32::MAX) {
        if depth < MAX_DEPTH {
            let scatter = hit.material.scatter(r, hit, rng);
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

pub fn render(width: usize, height: usize, camera: Camera, world: World) -> Vec<u32> {
    let num_pixels = width * height;
    let chunk_size = num_pixels / NUM_THREADS;

    let world = Arc::new(world);
    let mut buffer: Vec<u32> = vec![0; num_pixels];
    let mut pool = Pool::new(NUM_THREADS as u32);

    pool.scoped(|scope| {
        for (ichunk, chunk) in buffer.chunks_mut(chunk_size).enumerate() {
            let world_ref = Arc::clone(&world);
            scope.execute(move || {
                let mut rng = thread_rng();
                let start_index = ichunk * chunk_size;

                for k in 0..chunk.len() {
                    let mut c = Vec3::new(0.0, 0.0, 0.0);
                    let screen_pos = start_index + k;
                    let i = height - 1 - screen_pos / width;
                    let j = screen_pos % width;
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

                    chunk[k] = to_bgra(ir, ig, ib);
                }
            });
        }
    });

    buffer
}
