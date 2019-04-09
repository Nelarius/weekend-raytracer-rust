use crate::{
    hitable::HitRecord,
    ray::Ray,
    vec3::{random_in_unit_sphere, Vec3},
};

#[derive(Copy, Clone)]
pub struct Scatter {
    pub attenuation: Vec3,
    pub ray: Ray,
}

impl Scatter {
    pub fn new(attenuation: Vec3, ray: Ray) -> Scatter {
        Scatter { attenuation, ray }
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Scatter;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Scatter {
        let target = hit.p + hit.n + random_in_unit_sphere();
        let attenuation = self.albedo;
        let scattered_ray = Ray::new(hit.p, target - hit.p);
        Scatter::new(attenuation, scattered_ray)
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Scatter {
        let reflected = ray.direction.reflect(hit.n);
        let attenuation = self.albedo;
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        Scatter::new(attenuation, scattered)
    }
}
