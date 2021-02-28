use std::f64::INFINITY;
use std::fs::{File};
use std::io::Write;
use std::sync::Arc;
use crate::camera::create_camera;
use crate::color::write_color;
use crate::ray::Ray;
use crate::utils::random_double;
use crate::vec3::{Color, create_vec_3, Vec3};
use crate::hittables::hittable::Hittable;
use crate::hittables::hittable_list::HittableList;
use crate::lights::light_list::LightList;
use std::borrow::BorrowMut;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use crate::from_stl::read_stl;

mod vec3;
mod camera;
mod utils;
mod ray;
mod hittables;
mod color;
pub mod material;
mod lights;
mod from_stl;


fn color_at(r: &Ray, world: &Box<dyn Hittable + Send + Sync>, light_list: &LightList, depth: i32) -> Color {
    if depth == 0 {
        return Color { e: [0.0, 0.0, 0.0] };
    }

    let option = world.hit(r, 0.0001, INFINITY);
    if !option.is_none() {
        let rec = option.unwrap();
        let mut scattered: Ray = Ray { origin: Vec3 { e: [0.0, 0.0, 0.0] }, direction: Vec3 { e: [0.0, 0.0, 0.0] } };
        let mut attenuation: Color = Vec3 { e: [0.0, 0.0, 0.0] };
        if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
            let mut return_value = attenuation * color_at(&scattered, world, light_list, depth - 1);
            light_list.get_color(&rec, world, return_value.borrow_mut());
            return return_value;
        }
        return Color { e: [0.0, 0.0, 0.0] };
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color { e: [1.0, 1.0, 1.0] } * (1.0 - t) + Color { e: [0.5, 0.7, 1.0] } * t;
}


fn main() {
    let mut file = File::create("image.ppm").expect("Could not open the output file.");

    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: i32 = 1280;
    let height: i32 = (width as f64 / aspect_ratio) as i32;

    let look_from = create_vec_3(0.0, 0.0, -1.5);
    let look_at = create_vec_3(0.0, 0.0, 0.0);
    let vup = create_vec_3(0.0, 1.0, 0.0);

    let cam = create_camera(look_from, look_at, vup, 150.0, 16.0 / 9.0, 3.0);

    let samples_per_pixel = 100;
    let depth = 50;

    writeln!(file, "P3\n{} {}\n255\n", width, height).expect("Cannot write to file.");

    let world = HittableList { list: read_stl(String::from("stl.stl")) };

    // let material_ground = Arc::new(Diffuse { albedo: Color { e: [0.8, 0.8, 0.0] } });
    /*let material_left = Arc::new(Diffuse { albedo: Color { e: [0.7, 0.3, 0.8] } });
    let material_right = Arc::new(Metal { albedo: Color { e: [0.8, 0.8, 0.8] }, fuzz: 0.5 });
    let material_center = Arc::new(Metal { albedo: Color { e: [0.8, 0.6, 0.2] }, fuzz: 0.1 });

    let material_dielectric = Arc::new(Dielectric { ir: 1.5, tint: Color { e: [1.0, 0.8, 0.8] } });*/

    // world.add(Arc::new(Sphere { position: Point3 { e: [0.0, -101.0, 0.0] }, radius: 100.0, material: material_ground.clone() }));
    // world.add(Arc::new(Sphere { position: Point3 { e: [0.0, 0.0, 0.0] }, radius: 1.0, material: material_center.clone() }));
    // world.add(Arc::new(Sphere { position: Point3 { e: [-2.0, 0.0, 0.0] }, radius: 1.0, material: material_left.clone() }));
    // world.add(Arc::new(Sphere { position: Point3 { e: [2.0, 0.0, 0.0] }, radius: 1.0, material: material_right.clone() }));
    // world.add(Arc::new(Triangle { a: Point3{e: [0.0, -1.0, -2.5]}, b: Point3{e: [30.0, -1.0, 10.0]}, c: Point3{e: [-30.0, -1.0, 10.0]}, material: material_ground.clone() }));
    // world.add(Arc::new(Triangle { a: Point3{e: [0.0, -1.0, -1.0]}, b: Point3{e: [-3.0, 2.0, -1.0]}, c: Point3{e: [0.0, 2.0, -1.0]}, material: material_dielectric.clone() }));

    let world_box: Arc<Box<dyn Hittable + Send + Sync>> = Arc::new(Box::new(world));

    let light_list = LightList { lights: Vec::new(), ambience: Vec3 { e: [1.0, 1.0, 2.0] } };
    // light_list.add(Arc::from(Light {
    //     position: Vec3 { e: [2.0, 3.0, -4.0] },
    //     color: Vec3 { e: [0.8, 0.8, 0.8] },
    //     size: 1.0,
    // }));

    let n_workers = 10;
    let n_jobs = 10;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for job in 0..n_jobs {
        let tx = tx.clone();

        let cloned_light_list = light_list.clone();
        let cloned_world: Arc<Box<dyn Hittable + Send + Sync>> = world_box.clone();
        pool.execute(move || {
            let mut vector = Vec::new();
            for i in 0..height {
                let x = height - i - 1;
                println!("Current line of thread {}: {}", job, x);
                for j in 0..width {
                    let mut pixel_color = Color { e: [0.0, 0.0, 0.0] };
                    for _ in 0..samples_per_pixel / n_jobs {
                        let u = (j as f64 + random_double(0.0, 1.0)) / (width as f64 - 1.0);
                        let v = (x as f64 + random_double(0.0, 1.0)) / (height as f64 - 1.0);
                        let r = cam.get_ray(u, v);

                        pixel_color = pixel_color + color_at(&r, &cloned_world, &cloned_light_list, depth);
                    }
                    vector.push(pixel_color);
                }
            }
            tx.send(vector);
        });
    }
    pool.join();
    let mut take = rx.iter().take(n_jobs);
    let mut x = take.next().unwrap();
    for _i in 1..n_jobs {
        let nth = take.next().unwrap();
        for j in 0..nth.len() {
            x[j] = x[j] + nth[j];
        }
    }
    for i in 0..x.len() {
        write_color(&mut file, x[i], samples_per_pixel as i32);
    }
    println!("Finished!");
}
