pub mod randomizer;
pub mod camera;
pub mod sphere;
pub mod hittable;
pub mod ray;
pub mod color;
pub mod vec3;

use ray::Ray;
use color::Color;

//use vec3::Vec3 as Point3;

pub fn ray_color(r: &Ray, object: &impl hittable::Hittable, recursion_depth: i8) -> Color {
    if recursion_depth <= 0 {
        return Color::new_black();
    }
    if let Some(hit) = object.hit(r, 0.001, f64::INFINITY) {
        let target = hit.p + hit.normal + vec3::Vec3::random_in_unit_sphere();
        //let target = hit.p + hit.normal + vec3::Vec3::random_unit_vector();
        return ray_color(&Ray::new(hit.p, target - hit.p), object, recursion_depth - 1) * 0.5;
    }
    let unit_dir = r.direction().unit_vec();
    let t = 0.5 * (unit_dir.y() + 1.0);
    Color::new(1.0, 1.0, 1.0)*(1.0 - t) + Color::new(0.5, 0.7, 1.0)*t
}

fn main() {
    // Image
    const ASPECT_RATIO : f64 = 16.0 / 9.0;
    const IMG_W : usize = 400;
    const IMG_H : usize = (IMG_W as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL : u32 = 100;
    const MAX_DEPTH : i8 = 50;

    // World
    let mut world = hittable::HittableList::new();
    world.add(sphere::Sphere::new(vec3::Vec3::new(0.0, 0.0, -1.0), 0.5));
    world.add(sphere::Sphere::new(vec3::Vec3::new(0.0, -100.5, -1.0), 100.0));

    // Camera
    let cam = camera::Camera::new();

    if std::path::Path::new("image.ppm").exists() {
        eprintln!("Removing image.ppm");
        let _ = std::fs::remove_file("image.ppm");
    }
    // PPM render
    println!("P3");
    println!("{IMG_W} {IMG_H}");
    println!("255");
    
    eprintln!("Processing image...");
    eprint!("[____________________]");
    for w in (0..IMG_H).rev() {
        eprint!("\r[");
        let five_percents = (((IMG_H - w) as f64) / (IMG_H as f64) * 100.0) as u8 / 5;
        for i in 0..20 {
            if five_percents > i {
                eprint!("#");
            } else {
                eprint!("_");
            }
        }
        eprint!("]");
        
        for h in 0..IMG_W {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (h as f64 + randomizer::rand_f64()) / ((IMG_W - 1) as f64);
                let v = (w as f64 + randomizer::rand_f64()) / ((IMG_H - 1) as f64);
                let r = cam.get_ray(u, v);
                
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            //let pixel_color = ray_color(&r, &sphere);
            pixel_color.write_color(SAMPLES_PER_PIXEL);
        }
    }
    eprintln!();
    eprintln!("Image processed");
}
