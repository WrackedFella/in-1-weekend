extern crate cgmath;
use cgmath::Vector3;
use crate::*;

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Vector3<f32>,
    pub lower_left_corner: Vector3<f32>,
    pub horizontal: Vector3<f32>,
    pub vertical: Vector3<f32>,
    pub lens_radius: f32,
    pub u: Vector3<f32>,
    pub v: Vector3<f32>,
    pub w: Vector3<f32>
}

impl Camera {
    pub fn new(look_from: Vector3<f32>, look_at: Vector3<f32>, v_up: Vector3<f32>, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = vfov*std::f32::consts::PI/180f32;
        let half_height = (theta/2f32).tan();
        let half_width = aspect * half_height;
        let temp_w: Vector3<f32> = unit_vector(look_from - look_at);
        let temp_u: Vector3<f32> = unit_vector(v_up.cross(temp_w));
        let temp_v: Vector3<f32> = temp_w.cross(temp_u);
        
        Camera {
            lower_left_corner: look_from-half_width*focus_dist*temp_u-half_height*focus_dist*temp_v-focus_dist*temp_w,
            horizontal: 2f32*half_width*focus_dist*temp_u,
            vertical: 2f32*half_width*focus_dist*temp_v,
            origin: look_from,
            lens_radius: aperture / 2f32,
            w: temp_w,
            u: temp_u,
            v: temp_v
        }
    }
    pub fn get_ray(self, s: f32, t: f32) -> Ray {
        let rd: Vector3<f32> = self.lens_radius*random_in_unit_disk();
        let offset = self.u*rd.x+self.v*rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset)
    }
}