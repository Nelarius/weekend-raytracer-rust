use crate::{ray::Ray, vec3::Vec3};

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}

// TODO: is this trait really needed? there is no generic code
impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    n: (1.0 / self.radius) * (hit_point - self.center),
                });
            }

            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(HitRecord {
                    t: temp,
                    p: hit_point,
                    n: (1.0 / self.radius) * (hit_point - self.center),
                });
            }
        }
        None
    }
}

pub struct World {
    spheres: Vec<Sphere>,
}

impl World {
    pub fn new(spheres: Vec<Sphere>) -> World {
        World { spheres }
    }
}

impl Hitable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut maybe_hit: Option<HitRecord> = None;
        for sphere in self.spheres.iter() {
            if let Some(hit) = sphere.hit(&ray, t_min, t_max) {
                closest_so_far = if hit.t < closest_so_far {
                    maybe_hit = Some(hit);
                    hit.t
                } else {
                    closest_so_far
                };
            }
        }
        maybe_hit
    }
}
