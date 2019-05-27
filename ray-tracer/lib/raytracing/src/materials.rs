use rand::{thread_rng, Rng};
use cgmath::Vector3;
use crate::*;

#[derive(Copy, Clone)]
pub enum MaterialType {
    Lambertian {
        albedo: Vector3<f32>
    },
    Metal {
        albedo: Vector3<f32>,
        fuzz: f32
    },
    Dielectric {
        ref_indx: f32
    }
}

impl Scatterable for MaterialType {
    fn scatter(&self, mut r_in: Ray, rec: HittableRecord) -> Option<ScatterRecord> {
        match &self {
            MaterialType::Lambertian { albedo } => {
                let target: Vector3<f32> = rec.p + rec.normal + random_in_unit_sphere();
                return Some(ScatterRecord {
                    attenuation: *albedo,
                    scattered: Ray::new(rec.p, target-rec.p)
                });
            },
            MaterialType::Metal { albedo, fuzz } => {
                let reflected: Vector3<f32> = reflect(unit_vector(r_in.direction()), rec.normal);
                let mut s = Ray::new(rec.p, reflected+*fuzz*random_in_unit_sphere());
                let x = cgmath::dot(s.direction(), rec.normal);
                if x > 0f32 {
                    return Some(ScatterRecord {
                        attenuation: *albedo,
                        scattered: s
                    });
                }
                return None;
            },
            MaterialType::Dielectric { ref_indx } => {
                let outward_normal: Vector3<f32>;
                let ni_over_nt: f32;
                let cosine: f32;
                if cgmath::dot(r_in.direction(), rec.normal) > 0f32 {
                    outward_normal = -rec.normal;
                    ni_over_nt = *ref_indx;
                    cosine = *ref_indx * cgmath::dot(r_in.direction(), rec.normal) / vector_length(r_in.direction());
                } else {
                    outward_normal = rec.normal;
                    ni_over_nt = 1.0f32 / *ref_indx;
                    cosine = -cgmath::dot(r_in.direction(), rec.normal) / vector_length(r_in.direction());
                }
                
                let refraction_test = refract(r_in.direction(), outward_normal, ni_over_nt);
                let reflect_prob: f32;
                let reflected = reflect(r_in.direction(), rec.normal);
                let scatter: Ray;
                let mut refracted = Vector3::new(0f32,0f32,0f32);

                match refraction_test {
                    Some(x) => {
                        reflect_prob = schlick(cosine, *ref_indx);
                        refracted = x;
                    },
                    None => {
                        reflect_prob = 1.0f32;
                    }
                }
                let mut rng = thread_rng();
                if rng.gen::<f32>() < reflect_prob {
                    // println!("Reflect {} {} {}", reflected.x, reflected.y, reflected.z);
                    scatter = Ray::new(rec.p, reflected);
                } else {
                    // println!("Refracted {} {} {}", refracted.x, refracted.y, refracted.z);
                    scatter = Ray::new(rec.p, refracted);
                }
                return Some(ScatterRecord {
                    attenuation: Vector3::new(1f32,1f32,1f32),
                    scattered: scatter
                });
            }
        }
    }
}
