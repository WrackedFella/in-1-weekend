extern crate cgmath;

use cgmath::Vector3;

pub trait Hitable {
    fn hit(&mut self, r: Ray, t_min: f32, t_max: f32, rec: HitableRecord) -> bool;
}

#[derive(Copy, Clone)]
pub struct Ray {
    a: Vector3<f32>,
    b: Vector3<f32>
}

#[derive(Copy, Clone)]
pub struct HitableRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>
}

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32
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

impl Sphere {
    pub fn new(cen: Vector3<f32>, r: f32) -> Sphere {
        Sphere {
            center: cen,
            radius: r
        }
    }
}

impl Hitable for Sphere {
    fn hit(&mut self, mut r: Ray, t_min: f32, t_max: f32, mut rec: HitableRecord) -> bool {
        let oc: Vector3<f32> = r.origin() - &self.center;
        let a: f32 = cgmath::dot(r.direction(), r.direction());
        let b: f32 = cgmath::dot(oc, r.direction());
        let c: f32 = cgmath::dot(oc, oc) - self.radius*self.radius;
        let discriminant: f32 = b*b - a*c;
        if discriminant > 0f32 {
            let mut temp = (-b - (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - &self.center) / self.radius;
                return true;
            }
            temp = (-b + (b*b-a*c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - &self.center) / self.radius;
                return true;
            }
        }
        return false;
    }
}
