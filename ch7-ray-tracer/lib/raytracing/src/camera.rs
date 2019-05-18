extern crate cgmath;
use cgmath::Vector3;
use crate::*;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vector3::new(-2f32,-1f32,-1f32),
            horizontal: Vector3::new(4f32,0f32,0f32),
            vertical: Vector3::new(0f32,2f32,0f32),
            origin: Vector3::new(0f32,0f32,0f32)
        }
    }
    pub fn get_ray(self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}