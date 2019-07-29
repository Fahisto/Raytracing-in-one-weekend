use ppm::*;
use std::fs::*;
use std::io::*;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;
    const COEFF: f32 = 255.99;

    let mut file_handle = File::create("my_ppm.ppm").expect("Failed to create file");
    let mut contents = format!("P3\r\n{} {}\r\n255\r\n", nx, ny);

    file_handle
        .write(&contents.as_bytes())
        .expect("Failed to write contents");

    let lower_left_corner = Vec3(-2.0, -1.0, -1.0);
    let hor = Vec3(4.0, 0.0, 0.0);
    let ver = Vec3(0.0, 2.0, 0.0);
    let orig = Vec3::zero();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let ray = Ray::new(orig, lower_left_corner + hor * u + ver * v);

            let color = ppm::color(ray);
            //println!("{}", color);

            let ir: i32 = (COEFF * color.0) as i32;
            let ig: i32 = (COEFF * color.1) as i32;
            let ib: i32 = (COEFF * color.2) as i32;

            contents = format!("{} {} {}\r\n", ir, ig, ib);
            file_handle
                .write(&contents.as_bytes())
                .expect("Failed to write contents");
        }
    }
}
