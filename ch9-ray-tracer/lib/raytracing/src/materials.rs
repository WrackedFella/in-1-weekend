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
    fn scatter(&self, r_in: Ray, rec: HittableRecord) -> Option<ScatterRecord> {
        let target: Vector3<f32> = rec.p + rec.normal + random_in_unit_sphere();
        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered: Ray::new(rec.p, target-rec.p)
        })
    }
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzz: f32
}

impl Metal {
    pub fn new(a: Vector3<f32>, f: f32) -> Metal {
        Metal {
            albedo: a,
            fuzz: f
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, mut r_in: Ray, rec: HittableRecord) -> Option<ScatterRecord> {
        let reflected: Vector3<f32> = reflect(unit_vector(r_in.direction()), rec.normal);
        let mut s = Ray::new(rec.p, reflected+self.fuzz*random_in_unit_sphere());
        let x = cgmath::dot(s.direction(), rec.normal);
        if x > 0f32 {
            return Some(ScatterRecord {
                attenuation: self.albedo,
                scattered: s
            });
        }
        return None;
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    pub ref_indx: f32
}

impl Dielectric {
    pub fn new(ri: f32) -> Dielectric {
        Dielectric {
            ref_indx: ri
        }
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, mut r_in: Ray, rec: HittableRecord) -> Option<ScatterRecord> {
        let mut outward_normal = Vector3::new(0f32,0f32,0f32);
        let mut ni_over_nt = 0f32;
        let reflected = reflect(r_in.direction(), rec.normal);
        let refracted = Vector3::new(0f32,0f32,0f32);
        if cgmath::dot(r_in.direction(), rec.normal) > 0f32 {
            outward_normal = -rec.normal;
            ni_over_nt = self.ref_indx;
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0f32 / self.ref_indx;
        }
        let refraction_test = refract(r_in.direction(), outward_normal, ni_over_nt);
        match refraction_test {
            Some(x) => {
                Some(ScatterRecord {
                    attenuation: Vector3::new(1f32,1f32,1f32),
                    scattered: Ray::new(rec.p, refracted)
                })
            },
            None => {
                Some(ScatterRecord {
                    attenuation: Vector3::new(1f32,1f32,1f32),
                    scattered: Ray::new(rec.p, reflected)
                })
            }
        }
    }
}
