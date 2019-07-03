use crate::camera::Camera;
use crate::hitable::World;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::prelude::*;
use rayon::prelude::*;

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
    (0..width * height)
        .into_par_iter()
        .map_init(
            || thread_rng(),
            |mut rng, screen_pos| {
                let mut c = Vec3::new(0.0, 0.0, 0.0);
                let i = height - 1 - screen_pos / width;
                let j = screen_pos % width;
                for _ in 0..NUM_SAMPLES {
                    let u = ((j as f32) + rng.gen::<f32>()) / (width as f32);
                    let v = ((i as f32) + rng.gen::<f32>()) / (height as f32);
                    let r = camera.make_ray(&mut rng, u, v);
                    c += color(r, &world, &mut rng, 0);
                }
                c = (1.0 / NUM_SAMPLES as f32) * c;
                let ir = (255.99 * c.x.sqrt()) as u32;
                let ig = (255.99 * c.y.sqrt()) as u32;
                let ib = (255.99 * c.z.sqrt()) as u32;

                to_bgra(ir, ig, ib)
            },
        )
        .collect()
}
