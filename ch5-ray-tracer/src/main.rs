extern crate raytracing;
extern crate cgmath;

use raytracing::*;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use cgmath::Vector3;

fn unit_vector(v: Vector3<f32>) -> Vector3<f32> {
    v / vector_length(v)
}

fn vector_length(v: Vector3<f32>) -> f32 {
    let length_squared: f32 = v.x*v.x + v.y*v.y + v.z*v.z;
    length_squared.sqrt()
}

fn hit_sphere(center: Vector3<f32>, radius: f32, mut r: Ray) -> f32 {
    let oc = r.origin() - center;
    let a: f32 = cgmath::dot(r.direction(), r.direction());
    let b: f32 = 2.0f32 * cgmath::dot(oc, r.direction());
    let c: f32 = cgmath::dot(oc, oc) - radius*radius;
    let discriminant: f32 = b*b - 4f32*a*c;
    if discriminant < 0f32 {
        return -1.0f32;
    } else {
        return (-b - discriminant.sqrt()) / (2.0f32*a);
    }
}

fn color(mut r: Ray, list: &[Box<Hitable>]) -> Vector3<f32> {
    let mut rec = HitableRecord {
            t: 0f32,
            p: Vector3::new(0f32,0f32,0f32),
            normal: Vector3::new(0f32,0f32,0f32)
        };
    let mut temp_rec = HitableRecord {
        t: 0f32,
        p: Vector3::new(0f32,0f32,0f32),
        normal: Vector3::new(0f32,0f32,0f32)
    };
    let mut hit_anything: bool = false;
    let mut closest_so_far: f32 = std::f32::MAX;
    list.iter().for_each(|h| { 
        let temp_temp_rec: HitableRecord = temp_rec;
        if h.hit(r, 0f32, closest_so_far, temp_temp_rec) {
            hit_anything = true;
            closest_so_far = temp_temp_rec.t;
            rec = temp_temp_rec;
        }
    });
    if hit_anything {
        return 0.5f32*Vector3::new(rec.normal.x+1f32, rec.normal.y+1f32, rec.normal.z+1f32);
    } else {
        let unit_direction: Vector3<f32> = unit_vector(r.direction());
        let t: f32 = 0.5f32*(unit_direction.y+1f32);
        return (1.0f32-t)*Vector3::new(1.0f32,1.0f32,1.0f32) + t*Vector3::new(0.5f32,0.7f32,1.0f32);
    }
        
}

fn main() -> std::io::Result<()> {
    let nx: f32 = 200.0;
    let ny: f32 = 100.0;
    let mut j: f32 = ny - 1.0;
    let mut file_contents = format!("P3\n{} {}\n255\n", nx, ny);
    let lower_left_corner = Vector3::new(-2.0f32, -1.0f32, -1.0f32);
    let horizontal = Vector3::new(4.0f32, 0.0f32, 0.0f32);
    let vertical = Vector3::new(0.0f32, 2.0f32, 0.0f32);
    let origin = Vector3::new(0.0f32, 0.0f32, 0.0f32);

    let mut list = Vec::new();
    list.push(Box::new(Sphere::new(Vector3::new(0f32,0f32,-1f32), 0.5f32)) as Box<Hitable>);
    list.push(Box::new(Sphere::new(Vector3::new(0f32,-100.5f32,-1f32), 0.5f32)) as Box<Hitable>);
    while j >= 0.0 {
        let mut i: f32 = 0.0;
        while i < nx {
            let u: f32 = i / nx;
            let v: f32 = j / ny;
            let uh: Vector3<f32> = u*horizontal;
            let vv: Vector3<f32> = v*vertical;
            let mut r = Ray::new(origin, lower_left_corner + uh + vv);

            let p = r.point_at_parameter(2.0);
            let col = color(r, &list);
            let ir: f32 = 255.99 * col.x;
            let ig: f32 = 255.99 * col.y;
            let ib: f32 = 255.99 * col.z;
            
            file_contents = file_contents + &format!("{} {} {}\n", ir, ig, ib);
            i = i + 1.0f32;
        }
        j = j - 1.0f32;
    }
    
    let mut file = File::create("hello_world.ppm")?;
    file.write_all(file_contents.as_bytes())?;
    Ok(())
}
