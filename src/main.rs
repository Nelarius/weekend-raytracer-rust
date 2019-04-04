mod hitable;
mod ray;
mod vec3;

use hitable::{Hitable, Sphere, World};
use minifb::{Key, Window, WindowOptions};
use ray::Ray;
use vec3::Vec3;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;

fn color(r: &Ray, world: &World) -> Vec3 {
    // TODO: how to get max float??
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
    let mut window = Window::new(
        "Raytracer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    //640 by 320
    let lower_left_corner = Vec3 {
        x: -1.6,
        y: -0.8,
        z: -1.0,
    };
    let horizontal = Vec3 {
        x: 3.2,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Vec3 {
        x: 0.0,
        y: 1.6,
        z: 0.0,
    };
    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    // list of spheres
    let spheres = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    let world = World::new(spheres);

    for j in 0..WIDTH {
        for i in 0..HEIGHT {
            let u = (j as f32) / (WIDTH as f32);
            let v = (i as f32) / (HEIGHT as f32);
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let c = color(&r, &world);

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
