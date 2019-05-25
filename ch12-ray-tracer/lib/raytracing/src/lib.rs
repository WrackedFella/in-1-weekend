
extern crate cgmath;
extern crate rand;
use rand::{thread_rng, Rng};
use cgmath::Vector3;

pub mod actors;
pub mod camera;

// Reflection function
pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1f32-ref_idx) / (1f32+ref_idx);
    r0 = r0*r0;
    return r0+(1f32-r0)*(1f32-cosine).powf(5f32);
}

pub fn refract(v: Vector3<f32>, n: Vector3<f32>, ni_over_nt: f32) -> Option<Vector3<f32>> {
    let uv = unit_vector(v);
    let dt = cgmath::dot(uv, n);
    let discriminant = 1.0f32 - ni_over_nt*ni_over_nt*(1f32-dt*dt);
    if discriminant > 0f32 {
        return Some(ni_over_nt*(uv-n*dt) - n*(discriminant.sqrt()));
    } else {
        return None;
    }
}

pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2f32*cgmath::dot(v,n)*n
}

pub fn multiply_vectors(v1: Vector3<f32>, v2: Vector3<f32>) -> Vector3<f32> {
    Vector3::new(v1.x*v2.x, v1.y*v2.y, v1.z*v2.z)
}

pub fn unit_vector(v: Vector3<f32>) -> Vector3<f32> {
    v / vector_length(v)
}

pub fn vector_length_squared(v: Vector3<f32>) -> f32 {
    v.x*v.x + v.y*v.y + v.z*v.z
}

pub fn vector_length(v: Vector3<f32>) -> f32 {
    let length_squared: f32 = v.x*v.x + v.y*v.y + v.z*v.z;
    length_squared.sqrt()
}

pub fn random_in_unit_sphere() -> Vector3<f32> {
    let mut rng = thread_rng();
    let mut p = Vector3::new(std::f32::MAX, std::f32::MAX, std::f32::MAX);
    while vector_length_squared(p) >= 1.0 {
        p = 2f32*Vector3::new(rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>()) - Vector3::new(1f32,1f32,1f32);
    }
    return p;
}

pub fn random_in_unit_disk() -> Vector3<f32> {
    let mut rng = thread_rng();
    let mut p: Vector3<f32>;
    loop {
        p = 2f32*Vector3::new(rng.gen::<f32>(),rng.gen::<f32>(),0f32) - Vector3::new(1f32,1f32,0f32);
        if cgmath::dot(p,p) < 1f32 {
            break;
        }
    }
    return p;
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HittableRecord>;
}

pub trait Scatterable {
    fn scatter(&self, r_in: Ray, rec: HittableRecord) -> Option<ScatterRecord>;
}

#[derive(Copy, Clone)]
pub struct RefractRecord {
    pub attenuation: Vector3<f32>,
    pub scattered: Ray
}

#[derive(Copy, Clone)]
pub struct ScatterRecord {
    pub attenuation: Vector3<f32>,
    pub scattered: Ray
}

#[derive(Copy, Clone)]
pub struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>
}

#[derive(Copy, Clone)]
pub struct HittableRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub mat_ptr: MaterialType
}

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
                let mut outward_normal = Vector3::new(0f32,0f32,0f32);
                let mut ni_over_nt = 0f32;
                let mut cosine = 0f32;
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
                let mut reflect_prob = 0f32;
                let reflected = reflect(r_in.direction(), rec.normal);
                let mut refracted = Vector3::new(0f32,0f32,0f32);
                let mut scatter = Ray::new(Vector3::new(0f32,0f32,0f32), Vector3::new(0f32,0f32,0f32));

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

impl Ray {
    pub fn new(a_in: Vector3<f32>, b_in: Vector3<f32>) -> Ray {
        Ray {
            a: a_in,
            b: b_in
        }
    }

    pub fn origin(&mut self) -> Vector3<f32> { self.a }
    pub fn direction(&mut self) -> Vector3<f32> { self.b }
    
    pub fn point_at_parameter(&mut self, t :f32) -> Vector3<f32> {
        &self.a + t*&self.b
    }
}
