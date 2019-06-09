use crate::{ray::Ray, vec3::random_in_unit_disk, vec3::Vec3};
use rand::prelude::*;

#[derive(Copy, Clone)]
pub struct Camera {
    eye: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let lens_radius = 0.5 * aperture;
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (0.5 * theta).tan();
        let half_width = aspect * half_height;

        let eye = lookfrom;

        let w = (lookfrom - lookat).make_unit_vector();
        let u = vup.cross(w).make_unit_vector();
        let v = w.cross(u);

        let lower_left_corner =
            eye - half_width * focus_dist * u - half_height * focus_dist * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            eye,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
        }
    }

    pub fn make_ray(&self, rng: &mut ThreadRng, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = rd.x * self.u + rd.y * self.v;
        let lens_pos = self.eye + offset;
        Ray::new(
            lens_pos,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - lens_pos,
        )
    }
}
