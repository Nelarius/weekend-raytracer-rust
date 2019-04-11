use crate::{ray::Ray, vec3::Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    eye: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (0.5 * theta).tan();
        let half_width = aspect * half_height;

        let eye = lookfrom;

        let w = (lookfrom - lookat).make_unit_vector();
        let u = vup.cross(w).make_unit_vector();
        let v = w.cross(u);

        let lower_left_corner = eye - half_width * u - half_height * v - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        Camera {
            eye,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn make_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.eye,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.eye,
        )
    }
}
