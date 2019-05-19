
extern crate cgmath;
extern crate rand;
use rand::{thread_rng, Rng};
use cgmath::Vector3;

pub mod actors;
pub mod camera;
pub mod materials;

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
    //println!("{} {} {}", p.x, p.y, p.z);
    return p;
}

pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2f32*cgmath::dot(v,n)*n
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<HittableRecord>;
}

pub trait Scatterable {
    fn scatter(&self, r_in: Ray, rec: HittableRecord) -> Option<ScatterData>;
}

#[derive(Copy, Clone)]
pub struct ScatterData {
    pub attenuation: Vector3<f32>,
    pub scattered: Ray
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