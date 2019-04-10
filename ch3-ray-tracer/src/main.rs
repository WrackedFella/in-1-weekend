extern crate raytracing;
extern crate cgmath;

use raytracing::Ray;
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

fn color(mut r: Ray) -> Vector3<f32> {
    let unit_direction: Vector3<f32> = unit_vector(r.direction());
    let t: f32 = 0.5f32*(unit_direction.y+1.0f32);
    ((1.0f32-t)*Vector3::new(1.0f32, 1.0f32, 1.0f32)) + (t*Vector3::new(0.5f32, 0.7f32, 1.0f32))
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
    while j >= 0.0 {
        let mut i: f32 = 0.0;
        while i < nx {
            let u: f32 = i / nx;
            let v: f32 = j / ny;
            let uh: Vector3<f32> = u*horizontal;
            let vv: Vector3<f32> = v*vertical;
            let r = Ray::new(origin, lower_left_corner + uh + vv);
            let mut shadow_r = Ray::new(origin, lower_left_corner + uh + vv);
            let col = color(r);
            let ir: f32 = 255.99 * col.x;
            let ig: f32 = 255.99 * col.y;
            let ib: f32 = 255.99 * col.z;
            
                
            if i == 10.0 && j == 20.0 {
                println!("Value of u: {}", u);
                println!("Value of v: {}", v);
                println!("Value of u*horizontal x: {}, y: {}", uh.x, uh.y);
                println!("Value of v*vertical x: {}, y: {}", vv.x, vv.y);
                let x = lower_left_corner + uh + vv;
                println!("Value of ray destination x: {}, y: {}", x.x, x.y);
                let unit_direction: Vector3<f32> = unit_vector(shadow_r.direction());
                println!("Value of Unit Vector x: {}, y: {}", unit_direction.x, unit_direction.y);
            }

            file_contents = file_contents + &format!("{} {} {}\n", ir, ig, ib);
            i = i + 1.0f32;
        }
        j = j - 1.0f32;
    }
    
    let mut file = File::create("hello_world.ppm")?;
    file.write_all(file_contents.as_bytes())?;
    Ok(())
}
