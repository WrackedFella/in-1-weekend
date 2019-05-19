extern crate raytracing;
extern crate cgmath;
extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use cgmath::Vector3;
use rand::{thread_rng, Rng};

use raytracing::*;
use raytracing::actors::Sphere;
use raytracing::camera::Camera;
use raytracing::materials::*;

// fn color(mut r: Ray, world: &[&Hittable], depth: i8) -> Vector3<f32> {
//     let mut rec = HittableRecord {
//             t: 0f32,
//             p: Vector3::new(0f32,0f32,0f32),
//             normal: Vector3::new(0f32,0f32,0f32)
//             mat_ptr: &Lambertian::new(Vector3::new(0f32,0f32,0f32))
//         };
//     let mut hit_anything: bool = false;
//     let mut closest_so_far: f32 = std::f32::MAX;

//     world.iter().for_each(|h| { 
//         let mut temp_rec: HittableRecord = rec;
//         if h.hit(r, 0.001f32, closest_so_far, &mut temp_rec) {
//             hit_anything = true;
//             closest_so_far = temp_rec.t;
//             rec = temp_rec;
//         }
//     });
    
//     if hit_anything {
//         let mut scattered = Ray::new(Vector3::new(0f32,0f32,0f32),Vector3::new(0f32,0f32,0f32));
//         let mut attentuation = Vector3::new(0f32,0f32,0f32);
        
//         if depth < 50 { //&& rec.mat_ptr.scatter(r, rec, attentuation, scattered) {
//             return multiply_vectors(attentuation,color(scattered, world, depth+1));
//         } else {
//             return Vector3::new(0f32,0f32,0f32);
//         }
//     } else {
//         let unit_direction: Vector3<f32> = unit_vector(r.direction());
//         let t: f32 = 0.5f32*(unit_direction.y+1f32);
//         return (1.0f32-t)*Vector3::new(1.0f32,1.0f32,1.0f32) + t*Vector3::new(0.5f32,0.7f32,1.0f32);
//     }
// }

fn color(mut r: Ray, world: &[&Hittable], depth: i16) -> Vector3<f32> {
    let mut rec = HittableRecord {
            t: 0f32,
            p: Vector3::new(0f32,0f32,0f32),
            normal: Vector3::new(0f32,0f32,0f32),
            mat_ptr: &Lambertian::new(Vector3::new(0f32,0f32,0f32))
        };
    let mut hit_anything: bool = false;
    let mut closest_so_far: f32 = std::f32::MAX;
    let mut result = Vector3::new(0f32,0f32,0f32);
    
    world.iter().for_each(|h| { 
        let hit_test = h.hit(r, 0.001f32, closest_so_far);
        match hit_test {
            Some(x) => {
                hit_anything = true;
                closest_so_far = x.t;
                rec = x;
            },
            None => {
            }
        };
    });
    if hit_anything {
        let scatter_test = rec.mat_ptr.scatter(r, rec);
        match scatter_test {
            Some(x) => {
                if depth < 500 {
                    return multiply_vectors(x.attenuation,color(x.scattered, world, depth+1));
                }
            },
            None => {
            }
        }
        return Vector3::new(0f32,0f32,0f32);
    } else {
        let unit_direction: Vector3<f32> = unit_vector(r.direction());
        let t: f32 = 0.5f32*(unit_direction.y+1f32);
        return (1.0f32-t)*Vector3::new(1.0f32,1.0f32,1.0f32) + t*Vector3::new(0.5f32,0.7f32,1.0f32);
    }
    return result;
}

fn main() -> std::io::Result<()> {
    let nx: u8 = 200;
    let ny: u8 = 100;
    let ns: u8 = 100;

    let mut file_contents = format!("P3\n{} {}\n255\n", nx, ny);
    let mut rng = thread_rng();

    let m1 = Lambertian::new(Vector3::new(0.8f32,0.3f32,0.3f32));
    let m2 = Lambertian::new(Vector3::new(0.8f32,0.8f32,0.0f32));
    let m3 = Metal::new(Vector3::new(0.8f32,0.6f32,0.2f32), 1f32);
    let m4 = Dielectric::new(1.5f32);

    let s1 = Sphere::new(Vector3::new(0f32,0f32,-1f32), 0.5f32, &m1);
    let s2 = Sphere::new(Vector3::new(0f32,-100.5f32,-1f32), 100f32, &m2);
    let s3 = Sphere::new(Vector3::new(1f32,0f32,-1f32), 0.5f32, &m3);
    let s4 = Sphere::new(Vector3::new(-1f32,0f32,-1f32), 0.5f32, &m4);

    let world: Vec<&Hittable> = vec![&s1, &s2, &s3, &s4];

    let cam = Camera::new();
    for j in (1..(ny-1)).rev() {
        for i in 0..nx {
            let mut col: Vector3<f32> = Vector3::new(0f32,0f32,0f32);            
            for s in 0..ns {
                let u: f32 = (rng.gen::<f32>() + i as f32) / nx as f32;
                let v: f32 = (rng.gen::<f32>() + j as f32) / ny as f32;
                let mut r = cam.get_ray(u, v);
                let _p = r.point_at_parameter(2.0);
                col = col + color(r, &world, 0);
            }
            col = col / ns as f32;
            col = Vector3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let ir = 255.99 * col.x;
            let ig = 255.99 * col.y;
            let ib = 255.99 * col.z;
            
            file_contents = file_contents + &format!("{} {} {}\n", ir, ig, ib);
        }
    }
    
    let mut file = File::create("hello_world.ppm")?;
    file.write_all(file_contents.as_bytes())?;
    Ok(())
}
