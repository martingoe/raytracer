use std::sync::Arc;

use crate::hittables::hittable::Hittable;
use crate::optimizations::bvh::BBox;
use crate::vec3::Vec3;

pub fn get_pos_on_unit_cube(pos: &Vec3, b_box: &BBox) -> Vec3 {
    let x = (pos.x() - b_box.bounds[0].x()) / (b_box.bounds[1].x() - b_box.bounds[0].x());
    let y = (pos.y() - b_box.bounds[0].y()) / (b_box.bounds[1].y() - b_box.bounds[0].y());
    let z = (pos.z() - b_box.bounds[0].z()) / (b_box.bounds[1].z() - b_box.bounds[0].z());
    return Vec3 { e: [x, y, z] };
}

fn expand_bits(int: u64) -> u64 {
    // https://developer.nvidia.com/blog/thinking-parallel-part-iii-tree-construction-gpu/
    let mut v = int;

    v = (v * 0x00010001u64) & 0xFF0000FFu64;
    v = (v * 0x00000101u64) & 0x0F00F00Fu64;
    v = (v * 0x00000011u64) & 0xC30C30C3u64;
    v = (v * 0x00000005u64) & 0x49249249u64;
    return v;
}

pub fn morton_3d(pos: Vec3, b_box: &BBox) -> u64 {
    // https://developer.nvidia.com/blog/thinking-parallel-part-iii-tree-construction-gpu/
    let unit_positions = get_pos_on_unit_cube(&pos, b_box);
    let x = ((unit_positions.x() * 1024.0).max(0.0)).min(1023.0);
    let y = ((unit_positions.y() * 1024.0).max(0.0)).min(1023.0);
    let z = ((unit_positions.z() * 1024.0).max(0.0)).min(1023.0);
    let xx = expand_bits(x as u64);
    let yy = expand_bits(y as u64);
    let zz = expand_bits(z as u64);
    return xx * 4 + yy * 2 + zz;
}

fn get_max(vec: &Vec<u64>) -> u64 {
    let mut max = vec[0];
    for i in vec.iter() {
        max = *i.max(&max);
    }
    return max;
}

fn count_sort(
    values: &Vec<Arc<Hittable>>,
    codes: &Vec<u64>,
    exp: u64,
) -> (Vec<Arc<Hittable>>, Vec<u64>) {
    let mut count = [0; 64];
    let mut dup_values = values.clone();
    let mut dup_codes = codes.clone();
    for i in 0..values.len() {
        count[(codes[i] as u64 / exp % 64) as usize] += 1;
    }
    for i in 1..64 {
        count[i] += count[i - 1];
    }
    for i in (0..values.len()).rev() {
        let i1 = (codes[i] / exp) as usize % 64;
        dup_values[(count[i1] as usize) - 1] = values[i].clone();
        dup_codes[count[i1] as usize - 1] = codes[i].clone();
        count[((codes[i] as u64 / exp) % 64) as usize] -= 1;
    }
    return (dup_values, dup_codes);
}

pub fn radix_sort(values: &mut Vec<Arc<Hittable>>, codes: &mut Vec<u64>) {
    let max = get_max(&codes);
    let mut exp = 1;
    while (max as u64 / exp) > 0 {
        let sort = count_sort(values, codes, exp);
        *values = sort.0.clone();
        *codes = sort.1.clone();
        exp *= 64;
    }
}
