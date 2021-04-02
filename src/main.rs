extern crate sdl2;

use std::f64::INFINITY;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use threadpool::ThreadPool;

use utils::math_utils::random_double;

use crate::camera::{Camera, create_camera};
use crate::color::write_color;
use crate::hittables::hittable::{Hittable, HittableTrait};
use crate::material::Material;
use crate::opengl::opengl::draw_window;
use crate::optimizations::bvh::Bvh;
use crate::parsers::from_stl::read_stl;
use crate::ray::Ray;
use crate::textures::texture::Texture;
use crate::vec3::{Color, create_vec_3, Vec3};

mod opengl;


mod camera;
mod color;
mod hittables;
pub mod material;
mod noises;
mod optimizations;
mod parsers;
mod ray;
mod textures;
mod utils;
mod vec3;


const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: i32 = 400;
const HEIGHT: i32 = (WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: usize = 100;
const DEPTH: i32 = 10;

fn color_at(r: &Ray, world: Arc<Hittable>, depth: i32) -> Color {
    if depth == 0 {
        return Color { e: [0.0, 0.0, 0.0] };
    }

    let option = world.clone().hit(r, 0.0001, INFINITY);
    if !option.is_none() {
        let rec = option.unwrap();
        return rec
            .material
            .scatter(r, &rec, depth, world.clone())
            .unwrap_or(Color { e: [0.0, 0.0, 0.0] });
    }

    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    return Color { e: [1.0, 1.0, 1.0] } * (1.0 - t) + Color { e: [0.5, 0.7, 1.0] } * t;
}

fn main() {
    let look_from = create_vec_3(0.0, 0.0, 2.0);
    let look_at = create_vec_3(0.0, 0.0, 0.0);
    let vup = create_vec_3(0.0, 1.0, 0.0);

    let cam = create_camera(look_from, look_at, vup, 100.0, ASPECT_RATIO, 2.0);

    let mut vec = read_stl(
        "resources/stl/stl.stl".parse().unwrap(),
        Arc::new(Material::Metal {
            albedo: Texture::Solid {
                color: Color { e: [1.0, 0.0, 0.0] },
            },
            fuzz: 1.0,
            emission: Vec3 { e: [0.0, 0.0, 0.0] },
        }),
    );

    let world_box = Bvh::new_morton(&mut vec);

    render_scene_to_file(cam, world_box);
}

fn render_scene_to_file(cam: Camera, world_box: Arc<Hittable>) {
    let n_workers = 11;
    let pool = ThreadPool::new(n_workers);

    let before_render = Instant::now();
    let mut result = initiate_result_mutex(WIDTH, HEIGHT);
    launch_window_thread(&mut result);

    for job in 0..HEIGHT {
        let world_box = world_box.clone();
        let result = result.clone();
        pool.execute(move || {
            let mut row = Vec::new();
            let x = HEIGHT - job - 1;
            println!("Current line: {}", x);
            for j in 0..WIDTH {
                let mut pixel_color = Color { e: [0.0, 0.0, 0.0] };
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (j as f64 + random_double(0.0, 1.0)) / (WIDTH as f64 - 1.0);
                    let v = (x as f64 + random_double(0.0, 1.0)) / (HEIGHT as f64 - 1.0);
                    let r = cam.get_ray(u, v);

                    pixel_color = pixel_color + color_at(&r, world_box.clone(), DEPTH);
                }
                row.push(pixel_color);
            }
            result.lock().unwrap()[job as usize] = row;
        });
    }
    pool.join();

    write_data(&mut result);
    let time_2 = before_render.elapsed().as_secs();
    println!("Time for render: {}", time_2);

    println!("Finished!");
}

fn initiate_result_mutex(width: i32, height: i32) -> Arc<Mutex<Vec<Vec<Vec3>>>> {
    let result = Arc::new(Mutex::new(Vec::new()));
    for i in 0..height {
        result.lock().unwrap().push(Vec::new());
        for _ in 0..width {
            result.lock().unwrap()[i as usize].push(Vec3::new());
        }
    }
    result
}

fn launch_window_thread(result: &mut Arc<Mutex<Vec<Vec<Vec3>>>>) {
    let result = result.clone();
    thread::spawn(move || draw_window(result.clone(), WIDTH, HEIGHT, SAMPLES_PER_PIXEL));
}

fn write_data(result: &mut Arc<Mutex<Vec<Vec<Vec3>>>>) {
    let mut file =
        File::create("resources/renders/output.ppm").expect("Could not open the output file.");
    writeln!(file, "P3\n{} {}\n255\n", WIDTH, HEIGHT).expect("Cannot write to file.");
    let x = result.lock().unwrap();
    for i in 0..HEIGHT as usize {
        for j in 0..WIDTH as usize {
            write_color(&mut file, x[i][j], SAMPLES_PER_PIXEL);
        }
    }
}
