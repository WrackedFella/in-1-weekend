use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // let hello = "Hello, ";
    // let world = "world!";
    // let file_contents = format!("{}\n{}", hello, world);
    let nx: f32 = 200.0;
    let ny: f32 = 100.0;
    let mut j: f32 = ny - 1.0;
    let mut file_contents = format!("P3\n{} {}\n255\n", nx, ny);
    while j >= 0.0
    {
        let mut i: f32 = 0.0;
        while i < nx
        {
            let r: f32 = i / nx;
            let g: f32 = j / ny; 
            let b: f32 = 0.2;
            let ir: f32 = 255.99 * r;
            let ig: f32 = 255.99 * g;
            let ib: f32 = 255.99 * b;
            
            file_contents = file_contents + &format!("{} {} {}\n", ir, ig, ib);
            i = i + 1.0;
        }
        j = j - 1.0;
    }
    
    let mut file = File::create("hello_world.ppm")?;
    file.write_all(file_contents.as_bytes())?;
    Ok(())
}