use std::borrow::BorrowMut;
use std::io::{BufRead, BufReader};
use std::str::SplitWhitespace;
use std::sync::Arc;

use crate::hittables::hittable::Hittable;
use crate::hittables::triangle::Triangle;
use crate::material::Material;
use crate::vec3::Vec3;

pub fn read_obj(file: String, material: Arc<Material>) -> Vec<Arc<Hittable>> {
    let mut file = std::fs::File::open(file).expect("Cannot open the file");
    let reader = BufReader::new(file);

    let mut vertices = Vec::new();
    let mut texture_coordinates = Vec::new();
    let mut faces: Vec<Arc<Hittable>> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        println!("{}", vertices.len());
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        match words.next() {
            Some("v") => {
                let mut vertex = Vec3::new();
                for i in 0..3 {
                    vertex.e[i] = parse_next(words.borrow_mut());
                }
                vertices.push(vertex);
            }
            Some("vt") => {
                let mut vertex = (
                    parse_next(words.borrow_mut()),
                    parse_next(words.borrow_mut()),
                );
                texture_coordinates.push(vertex);
            }
            Some("f") => {
                let (a, texture_a) =
                    get_face_part(&mut vertices, &mut texture_coordinates, &mut words);
                let (b, texture_b) =
                    get_face_part(&mut vertices, &mut texture_coordinates, &mut words);
                let (c, texture_c) =
                    get_face_part(&mut vertices, &mut texture_coordinates, &mut words);
                let mut texture_coordinates = None;
                if texture_a != None {
                    let mut x = [(0.0, 0.0); 3];
                    x[0] = texture_a.unwrap();
                    x[1] = texture_b.unwrap();
                    x[2] = texture_c.unwrap();
                    texture_coordinates = Some(x);
                }
                faces.push(Arc::new(Hittable::Triangle {
                    triangle: Triangle::new_texture_coordinates(
                        a,
                        b,
                        c,
                        texture_coordinates,
                        material.clone(),
                    ),
                }));
            }
            _ => {}
        }
    }
    return faces;
}

fn get_face_part(
    vertices: &mut Vec<Vec3>,
    texture_coordinates: &mut Vec<(f64, f64)>,
    words: &mut SplitWhitespace,
) -> (Vec3, Option<(f64, f64)>) {
    let x: Vec<&str> = words.next().unwrap().split("/").collect();
    let vertex;
    let mut texture_coordinate = None;
    match x.len() {
        1 => vertex = vertices[x[0].parse::<usize>().unwrap() - 1],
        _ => {
            vertex = vertices[x[0].parse::<usize>().unwrap() - 1];
            texture_coordinate = Some(texture_coordinates[x[0].parse::<usize>().unwrap() - 1]);
        }
    }
    return (vertex, texture_coordinate);
}

fn parse_next(iteration: &mut SplitWhitespace) -> f64 {
    return iteration.next().unwrap().parse::<f64>().unwrap();
}
