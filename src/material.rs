use crate::{
    hitable::HitRecord,
    ray::Ray,
    vec3::{random_in_unit_sphere, Vec3},
};
use rand::random;

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

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let target = hit.p + hit.n + random_in_unit_sphere();
        let attenuation = self.albedo;
        let scattered_ray = Ray::new(hit.p, target - hit.p);
        Some(Scatter::new(attenuation, scattered_ray))
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let reflected = ray.direction.reflect(hit.n);
        let attenuation = self.albedo;
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        Some(Scatter::new(attenuation, scattered))
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub refraction_index: f32,
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    // ni * sin(i) = nt * sin(t)
    // sin(t) = sin(i) * (ni / nt)
    let uv = v.make_unit_vector();
    let dt = uv.dot(n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n;
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Dielectric {
    pub fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        // if the ray direction and hit normal are in the same half-sphere
        let (outward_normal, ni_over_nt, cosine) = if ray.direction.dot(hit.n) > 0.0 {
            (
                -hit.n,
                self.refraction_index,
                self.refraction_index * ray.direction.dot(hit.n) / ray.direction.length(),
            )
        } else {
            (
                hit.n,
                1.0 / self.refraction_index,
                -ray.direction.dot(hit.n) / ray.direction.length(),
            )
        };

        if let Some(refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
            let refraction_prob = schlick(cosine, self.refraction_index);
            let out_dir = if random::<f32>() < refraction_prob {
                ray.direction.reflect(hit.n)
            } else {
                refracted
            };
            Some(Scatter::new(
                Vec3::new(1.0, 1.0, 1.0),
                Ray::new(hit.p, out_dir),
            ))
        } else {
            Some(Scatter::new(
                Vec3::new(1.0, 1.0, 1.0),
                Ray::new(hit.p, ray.direction.reflect(hit.n)),
            ))
        }
    }
}

#[derive(Copy, Clone)]
pub enum Material {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    Metal(Metal),
}
