extern crate raytracing;
extern crate cgmath;
extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use cgmath::Vector3;
use rand::prelude::*;

use raytracing::*;
use raytracing::actors::Sphere;
use raytracing::camera::Camera;

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
    let ns: u8 = 100;
    let mut s: u8 = 0;

    let mut file_contents = format!("P3\n{} {}\n255\n", nx, ny);
    
    let s1 = Sphere::new(Vector3::new(0f32,0f32,-1f32), 0.5f32);
    let s2 = Sphere::new(Vector3::new(0f32,-100.5f32,-1f32), 100f32);
    let world: Vec<&Hittable> = vec![&s1, &s2];
    let cam = Camera::new();

    let mut j: u8 = ny - 1;
    loop {
        let mut i: u8 = 0;
        while i < nx {
            let mut col = Vector3::new(0f32,0f32,0f32);            
            while s < ns {
                let rand1: f32 = rand::random();
                let rand2: f32 = rand::random();
                
                let u: f32 = (rand1 * i as f32) / nx as f32;
                let v: f32 = (rand2 * j as f32) / ny as f32;
                let mut r = cam.get_ray(u, v);
                let _p = r.point_at_parameter(2.0);
                col = col + color(r, &world);
                s = s + 1;
                println!("{}, {}, {}", col.x, col.y, col.z); // has values
            }
            println!("{}, {}, {}", col.x, col.y, col.z); // No values
            col = col / ns as f32;
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
