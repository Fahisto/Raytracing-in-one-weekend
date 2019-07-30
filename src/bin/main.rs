use ppm::*;
use ppm::{Hitable, HitableList, Sphere};
use rand::*;
use std::fs::*;
use std::io::*;

fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;
    let ns: i32 = 100;
    const COEFF: f32 = 255.99;

    let mut file_handle = File::create("my_ppm.ppm").expect("Failed to create file");
    let mut contents = format!("P3\r\n{} {}\r\n255\r\n", nx, ny);

    file_handle
        .write(&contents.as_bytes())
        .expect("Failed to write contents");

    let hitable_list = HitableList::new_filled_list(vec![
        Sphere::new(Vec3(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3(0.0, -100.5, -1.0), 100.0),
    ]);
    let camera = Camera::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut color = Vec3::zero();
            for _ in 0..ns {
                let rand1: f32 = rand::thread_rng().gen();
                let rand2: f32 = rand::thread_rng().gen();
                let u = ((i as f32) + rand1) / nx as f32;
                let v = ((j as f32) + rand2) / ny as f32;
                let ray = camera.get_ray(u, v);
                let point = ray.point_at_parameter(2.0);
                color += ppm::color(ray, &hitable_list);
                //println!("{}", color);
            }
            color /= ns as f32;
            color = Vec3(color.0.sqrt(), color.1.sqrt(), color.2.sqrt());
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
