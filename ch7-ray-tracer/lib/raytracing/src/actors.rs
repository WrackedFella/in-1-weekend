extern crate cgmath;
use cgmath::Vector3;
use crate::*;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32
}

impl Sphere {
    pub fn new(cen: Vector3<f32>, r: f32) -> Sphere {
        Sphere {
            center: cen,
            radius: r
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, mut r: Ray, t_min: f32, t_max: f32,  rec: &mut HittableRecord) -> bool {
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
