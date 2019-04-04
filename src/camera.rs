use crate::{ray::Ray, vec3::Vec3};

#[derive(Copy, Clone)]
pub struct Camera {
    eye: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(eye: Vec3, lower_left_corner: Vec3, horizontal: Vec3, vertical: Vec3) -> Camera {
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
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }
}
