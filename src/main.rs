use std::borrow::{BorrowMut};
use std::f64::INFINITY;
use std::fs::File;
use std::io::Write;
use std::sync::Arc;
use std::sync::mpsc::channel;

use threadpool::ThreadPool;

use crate::camera::create_camera;
use crate::color::write_color;
use crate::from_stl::read_stl;
use crate::hittables::bvh::initiate_bvh;
use crate::hittables::hittable::{Hittable, HittableTrait};
use crate::lights::light::Light;
use crate::lights::light_list::LightList;
use crate::material::{Diffuse, Material};
use crate::ray::{create_ray, Ray};
use crate::utils::random_double;
use crate::vec3::{Color, create_vec_3, Vec3};

mod vec3;
mod camera;
mod utils;
mod ray;
mod hittables;
mod color;
pub mod material;
mod lights;
mod from_stl;


fn color_at(r: &Ray, world: Arc<Hittable>, light_list: &LightList, depth: i32) -> Color {
    if depth == 0 {
        return Color { e: [0.0, 0.0, 0.0] };
    }

    let option = world.clone().hit(r, 0.0001, INFINITY);
    if !option.is_none() {
        let rec = option.unwrap();
        let mut scattered: Ray = create_ray(Vec3 { e: [0.0, 0.0, 0.0] }, Vec3 { e: [0.0, 0.0, 0.0] });
        let mut attenuation: Color = Vec3 { e: [0.0, 0.0, 0.0] };
        if rec.material.scatter(r, &rec, &mut attenuation, &mut scattered) {
            let mut return_value = attenuation * color_at(&scattered, world.clone(), light_list, depth - 1);
            light_list.get_color(&rec, world.clone(), return_value.borrow_mut());
            return return_value;
        }
        return Color { e: [0.0, 0.0, 0.0] };
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color { e: [1.0, 1.0, 1.0] } * (1.0 - t) + Color { e: [0.5, 0.7, 1.0] } * t;
}


fn main() {
    let mut file = File::create("resources/renders/output.ppm").expect("Could not open the output file.");

    // let aspect_ratio: f64 = 16.0 / 9.0;
    let aspect_ratio = 1.0;
    let width: i32 = 600;
    let height: i32 = (width as f64 / aspect_ratio) as i32;

    let look_from = create_vec_3(70.0, -80.0, -10.0);
    let look_at = create_vec_3(50.0, 0.0, 10.0);
    let vup = create_vec_3(1.0, 0.0, 0.0);

    let cam = create_camera(look_from, look_at, vup, 100.0, aspect_ratio, 3.0);

    let samples_per_pixel = 110;
    let depth = 15;

    writeln!(file, "P3\n{} {}\n255\n", width, height).expect("Cannot write to file.");

    let mut vec = read_stl("resources/stl/bunny.stl".parse().unwrap(), Arc::new(Material::Diffuse { diffuse: Diffuse { albedo: Color { e: [0.7, 0.6, 0.2] } } }));
    let world_box = initiate_bvh(&mut vec);

    let mut light_list = LightList { lights: Vec::new(), ambience: Vec3 { e: [0.7, 0.7, 0.7] } };
    light_list.add(Arc::from(Light {
        position: Vec3 { e: [0.0, 18.0, 0.0] },
        color: Vec3 { e: [0.3, 0.3, 0.3] },
        size: 0.5,
    }));

    let n_workers = 11;
    let n_jobs = 11;
    let pool = ThreadPool::new(n_workers);

    let (tx, rx) = channel();
    for job in 0..n_jobs {
        let tx = tx.clone();

        let cloned_light_list = light_list.clone();
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

                        pixel_color = pixel_color + color_at(&r, cloned_world.clone(), &cloned_light_list, depth);
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
    println!("Finished!");
}
