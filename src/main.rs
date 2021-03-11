use std::f64::INFINITY;
use std::fs::File;
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::time::Instant;

use threadpool::ThreadPool;

use utils::math_utils::random_double;

use crate::camera::create_camera;
use crate::color::write_color;
use crate::from_stl::read_stl;
use crate::hittables::bvh::{initiate_bvh, surround};
use crate::hittables::hittable::{Hittable, HittableTrait};
use crate::hittables::hittable_list::HittableList;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::morton_code::bvh_morton;
use crate::vec3::{Color, create_vec_3, Vec3};
use crate::textures::texture::Texture;

mod vec3;
mod camera;
mod ray;
mod hittables;
mod color;
pub mod material;
mod from_stl;
mod utils;
mod textures;


fn color_at(r: &Ray, world: Arc<Hittable>, depth: i32) -> Color {
    if depth == 0 {
        return Color { e: [0.0, 0.0, 0.0] };
    }

    let option = world.clone().hit(r, 0.0001, INFINITY);
    if !option.is_none() {
        let rec = option.unwrap();
        return rec.material.scatter(r, &rec, depth, world.clone()).unwrap_or(Color { e: [0.0, 0.0, 0.0] });
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color { e: [1.0, 1.0, 1.0] } * (1.0 - t) + Color { e: [0.5, 0.7, 1.0] } * t;
}


fn main() {
    let aspect_ratio: f64 = 16.0 / 9.0;
    let width: i32 = 1280;
    let height: i32 = (width as f64 / aspect_ratio) as i32;

    let mut look_from = create_vec_3(-5.5, 0.0, 0.0);
    let look_at = create_vec_3(0.0, 0.0, 0.0);
    let vup = create_vec_3(0.0, 1.0, 0.0);

    let mut cam = create_camera(look_from, look_at, vup, 100.0, aspect_ratio, 2.0);

    let samples_per_pixel = 150;
    let depth = 75;



    let mut vec = read_stl("resources/stl/troopers_white.stl".parse().unwrap(), Arc::new(Material::Metal { albedo: Texture::Solid {color: Color { e: [1.0, 1.0, 1.0] }}, fuzz: 0.05, emission: Vec3 { e: [0.0, 0.0, 0.0] } }));
    vec.append(&mut read_stl("resources/stl/troopers_black.stl".parse().unwrap(), Arc::new(Material::Diffuse { albedo: Texture::Solid {color: Color { e: [0.05, 0.05, 0.05] }}, emission: Vec3 { e: [0.0, 0.0, 0.0] } })));
    vec.append(&mut read_stl("resources/stl/troopers_lights.stl".parse().unwrap(), Arc::new(Material::Diffuse { albedo: Texture::Solid { color: Color{e: [0.82, 0.23, 0.23] }}, emission: Vec3 { e: [8.2, 2.3, 2.3] } })));

    // let world_box = aac(&vec, &surround(&vec.clone()));
    let before_bvh = Instant::now();
    let world_box = bvh_morton(&mut vec);
    // let world_box = initiate_bvh(&mut vec);
    let time = before_bvh.elapsed().as_secs();

    let mut file = File::create("resources/renders/output.ppm").expect("Could not open the output file.");
    writeln!(file, "P3\n{} {}\n255\n", width, height).expect("Cannot write to file.");
    let n_workers = 11;
    let n_jobs = 11;
    let pool = ThreadPool::new(n_workers);


    let before_render = Instant::now();
    let (tx, rx) = channel();
    for job in 0..n_jobs {
        let tx = tx.clone();

        let cloned_world = world_box.clone();
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

                        pixel_color = pixel_color + color_at(&r, cloned_world.clone(), depth);
                    }
                    vector.push(pixel_color);
                }
            }
            tx.send(vector).expect("Could not send the result from the thread.");
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
    file.flush();

    let time_2 = before_render.elapsed().as_secs();
    println!("Time for BVH: {}", time);
    println!("Time for render: {}", time_2);

    println!("Finished!");
}

