
pub mod actors;

pub mod camera;

extern crate cgmath;
use cgmath::Vector3;

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f32, t_max: f32, rec: &mut HittableRecord) -> bool;
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
    pub normal: Vector3<f32>
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