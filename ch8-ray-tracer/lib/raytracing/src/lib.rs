
pub mod actors;

pub mod camera;

extern crate cgmath;
extern crate rand;
use rand::{thread_rng, Rng};
use cgmath::Vector3;

pub fn multiply_vectors(v1: Vector3<f32>, v2: Vector3<f32>) -> Vector3<f32> {
    //println!("{} {} {}", v1.x, v1.y, v1.z);
    let test = Vector3::new(v1.x*v2.x, v1.y*v2.y, v1.z*v2.z);
    //println!("{} {} {}", test.x, test.y, test.z);
    return test;
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
    let mut p = Vector3::new(0f32,0f32,0f32);
    while vector_length_squared(p) >= 1.0 {
        p = 2f32*Vector3::new(rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>()) - Vector3::new(1f32,1f32,1f32);
    }
    return p;
}

pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2f32*cgmath::dot(v,n)*n
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HittableRecord) -> bool;
}

pub trait Scatterable {
    fn scatter(&self, r_in: Ray, rec: HittableRecord, attenuation: Vector3<f32>, scattered: Ray) -> bool;
}

#[derive(Copy, Clone)]
pub struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>
}

#[derive(Copy, Clone)]
pub struct HittableRecord<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub mat_ptr: &'a (Scatterable + 'a)
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
    fn scatter(&self, r_in: Ray, rec: HittableRecord, mut attenuation: Vector3<f32>, mut scattered: Ray) -> bool {
        let target: Vector3<f32> = rec.p + rec.normal + random_in_unit_sphere();
        scattered = Ray::new(rec.p, target-rec.p);
        attenuation = self.albedo;
        // println!("Albedo {} {} {}", self.albedo.x, self.albedo.y, self.albedo.z);
        return true;
    }
}

pub struct Metal {
    pub albedo: Vector3<f32>
}

impl Metal {
    pub fn new(a: Vector3<f32>) -> Metal {
        println!("Metal {} {} {}", a.x, a.y, a.z);
        Metal {
            albedo: a
        }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, mut r_in: Ray, rec: HittableRecord, mut attenuation: Vector3<f32>, mut scattered: Ray) -> bool {
        let reflected: Vector3<f32> = reflect(unit_vector(r_in.direction()), rec.normal);
        scattered = Ray::new(rec.p, reflected);
        println!("Albedo {} {} {}", self.albedo.x, self.albedo.y, self.albedo.z);
        attenuation = self.albedo;
        return cgmath::dot(scattered.direction(), rec.normal) > 0f32;
    }
}
