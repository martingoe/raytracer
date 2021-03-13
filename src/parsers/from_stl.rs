use crate::hittables::hittable::Hittable;
use crate::hittables::triangle::Triangle;
use crate::material::Material;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

pub fn read_stl(file: String, material: Arc<Material>) -> Vec<Arc<Hittable>> {
    let mut file = std::fs::File::open(file).expect("Cannot open the file");

    let mut string: [u8; 84] = [0; 84];
    file.by_ref()
        .take(84)
        .read_exact(&mut string)
        .expect("Reading did not return a value. STL file must be malformed.");
    let chunks = read_chunks(&mut file);
    let mut vector: Vec<Arc<Hittable>> = Vec::with_capacity(chunks.len());
    let mut i = 0;
    while i < chunks.len() - 4 {
        let mut x = chunks[i].as_slice();
        let a = get_vec3(&x);
        x = chunks[i + 1].as_slice();
        let b = get_vec3(&x);
        x = chunks[i + 2].as_slice();
        let c = get_vec3(&x);
        vector.push(Arc::from(Hittable::Triangle {
            triangle: Triangle::new(a, b, c, material.clone()),
        }));

        i += 3;
    }

    return vector;
}

fn read_chunks(file: &mut File) -> Vec<Vec<u8>> {
    let mut finished = false;
    let mut result = Vec::new();
    while !finished {
        for i in 0..5 {
            let chunk_size = if i == 4 { 2 } else { 12 };
            if i == 4 || i == 0 {
                let mut vec = Vec::new();
                file.by_ref()
                    .take(chunk_size)
                    .read_to_end(&mut vec)
                    .expect("Could not read from the STL file.");
                continue;
            }
            let mut chunk = Vec::with_capacity(chunk_size as usize);
            let n = file
                .by_ref()
                .take(chunk_size as u64)
                .read_to_end(&mut chunk)
                .expect("Could not read from the STL file.");
            chunk.reverse();
            if n == 0 {
                finished = true;
            }

            result.push(chunk);
            if n < chunk_size as usize {
                finished = true;
            }
        }
    }
    return result;
}

fn get_vec3(x: &&[u8]) -> Vec3 {
    let mut array: [u8; 4] = Default::default();
    array.copy_from_slice(&x[0..4]);
    let a = f32::from_be_bytes(array);
    array.copy_from_slice(&x[4..8]);
    let b = f32::from_be_bytes(array);
    array.copy_from_slice(&x[8..12]);
    let c = f32::from_be_bytes(array);
    return Vec3 {
        e: [a as f64, b as f64, c as f64],
    };
}
