use crate::hittables::hittable::Hittable;
use std::io::Read;
use crate::hittables::triangle::Triangle;
use crate::vec3::{Vec3, Color};
use std::sync::Arc;
use crate::material::Material;

pub fn read_stl(file: String) -> Vec<Arc<dyn Hittable>> {
    let mut file = std::fs::File::open(file).expect("Cannot open the file");

    let mut list_of_chunks = Vec::new();

    let mut string: [u8; 80] = [0; 80];
    file.by_ref().take(80).read_exact(&mut string);
    let mut dier: Vec<u8> = Vec::new();
    file.by_ref().take(4).read_to_end(&mut dier);
    // let count = u32::from_be_bytes(dier.as_slice());
    let mut finished = false;
    while !finished {
        for i in 0..5 {
            let chunk_size = if i == 4 { 2 } else { 12 };
            if i == 4 || i == 0 {
                let mut vec = Vec::new();
                file.by_ref().take(chunk_size).read_to_end(&mut vec);
                continue;
            }
            let mut chunk = Vec::with_capacity(chunk_size as usize);
            let n = file.by_ref().take(chunk_size as u64).read_to_end(&mut chunk).expect("Test");
            chunk.reverse();
            if n == 0 { finished = true; }
            list_of_chunks.push(chunk);
            if n < chunk_size as usize { finished = true; }
        }
    }
    let mut vector: Vec<Arc<dyn Hittable>> = Vec::new();
    let mut i = 0;
    while i < list_of_chunks.len() - 4 {
        let mut x = list_of_chunks[i].as_slice();
        let a = get_vec3(&x);
        x = list_of_chunks[i + 1].as_slice();
        let b = get_vec3(&x);
        x = list_of_chunks[i + 2].as_slice();
        let c = get_vec3(&x);
        vector.push(Arc::from(Triangle {
            a,
            b,
            c,
            material: Arc::from(Metal { albedo: Color { e: [0.8, 0.6, 0.2] }, fuzz: 0.1 }),
        }));

        i += 3;
    }

    return vector;
}

fn get_vec3(x: &&[u8]) -> Vec3 {
    let mut array: [u8; 4] = Default::default();
    array.copy_from_slice(&x[0..4]);
    let a = f32::from_be_bytes(array);
    array.copy_from_slice(&x[4..8]);
    let b = f32::from_be_bytes(array);
    array.copy_from_slice(&x[8..12]);
    let c = f32::from_be_bytes(array);
    return Vec3 { e: [a as f64, b as f64, c as f64] };
}