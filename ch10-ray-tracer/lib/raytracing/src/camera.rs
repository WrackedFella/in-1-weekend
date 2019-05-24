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
    pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, v_up: Vector3<f32>, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov*std::f32::consts::PI/180f32;
        let half_height = (theta/2f32).tan();
        let half_width = aspect * half_height;
        let w: Vector3<f32> = unit_vector(look_from - look_at);
        let u: Vector3<f32> = unit_vector(v_up.cross(w));
        let v: Vector3<f32> = w.cross(u);

        Camera {
            lower_left_corner: look_from-half_width*u-half_height*v-w,
            horizontal: 2f32*half_width*u,
            vertical: 2f32*half_width*v,
            origin: look_from
        }
    }
    pub fn get_ray(self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}