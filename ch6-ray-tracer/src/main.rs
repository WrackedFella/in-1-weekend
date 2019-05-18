extern crate raytracing;
extern crate cgmath;

use raytracing::*;
use raytracing::actors::Sphere;
use std::fs::File;
use std::io::prelude::*;
use cgmath::Vector3;

fn unit_vector(v: Vector3<f32>) -> Vector3<f32> {
    v / vector_length(v)
}

fn vector_length(v: Vector3<f32>) -> f32 {
    let length_squared: f32 = v.x*v.x + v.y*v.y + v.z*v.z;
    length_squared.sqrt()
}

fn color(mut r: Ray, world: &[&Hittable]) -> Vector3<f32> {
    let mut rec = HittableRecord {
            t: 0f32,
            p: Vector3::new(0f32,0f32,0f32),
            normal: Vector3::new(0f32,0f32,0f32)
        };
    let mut hit_anything: bool = false;
    let mut closest_so_far: f32 = std::f32::MAX;

    world.iter().for_each(|h| { 
        let mut temp_rec: HittableRecord = rec;
        if h.hit(r, 0f32, closest_so_far, &mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            rec = temp_rec;
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
    let nx: u8 = 200;
    let ny: u8 = 100;
    let mut j: u8 = ny - 1;
    let mut file_contents = format!("P3\n{} {}\n255\n", nx, ny);
    let lower_left_corner = Vector3::new(-2.0f32, -1.0f32, -1.0f32);
    let horizontal = Vector3::new(4.0f32, 0.0f32, 0.0f32);
    let vertical = Vector3::new(0.0f32, 2.0f32, 0.0f32);
    let origin = Vector3::new(0.0f32, 0.0f32, 0.0f32);

    let s1 = Sphere::new(Vector3::new(0f32,0f32,-1f32), 0.5f32);
    let s2 = Sphere::new(Vector3::new(0f32,-100.5f32,-1f32), 100f32);
    let world: Vec<&Hittable> = vec![&s1, &s2];

    loop {
        let mut i: u8 = 0;
        while i < nx {
            let u: f32 = i as f32 / nx as f32;
            let v: f32 = j as f32 / ny as f32;
            let mut r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);

            let p = r.point_at_parameter(2.0);
            let col = color(r, &world);
            let ir = (255.99 * col.x) as u8;
            let ig = (255.99 * col.y) as u8;
            let ib = (255.99 * col.z) as u8;
            
            file_contents = file_contents + &format!("{} {} {}\n", ir, ig, ib);
            i = i + 1;
        }
        if j == 0 {
            break;
        }
        j = j - 1;
    }
    
    let mut file = File::create("hello_world.ppm")?;
    file.write_all(file_contents.as_bytes())?;
    Ok(())
}
