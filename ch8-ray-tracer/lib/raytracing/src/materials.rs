extern crate cgmath;
use cgmath::Vector3;
use crate::*;

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Vector3<f32>
}

impl Lambertian {
    pub fn new(a: Vector3<f32>) -> Lambertian {
        Lambertian {
            albedo: a
        }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HittableRecord) -> Option<ScatterData> {
        let target: Vector3<f32> = rec.p + rec.normal + random_in_unit_sphere();
        Some(ScatterData {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, target-rec.p)
        })
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vector3<f32>
}

impl Metal {
    pub fn new(a: Vector3<f32>) -> Metal {
        Metal {
            albedo: a
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, mut r_in: Ray, rec: HittableRecord) -> Option<ScatterData> {
        let reflected: Vector3<f32> = reflect(unit_vector(r_in.direction()), rec.normal);
        let a = self.albedo;
        let mut s = Ray::new(rec.p, reflected);
        let x = cgmath::dot(s.direction(), rec.normal);
        if x > 0f32 {
            return Some(ScatterData {
                attenuation: a,
                scattered: s
            });
        }
        return None;
    }
}
