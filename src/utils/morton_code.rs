use std::sync::Arc;

use crate::hittables::bvh::{BBox, Bvh, surround};
use crate::hittables::hittable::{Hittable, HittableTrait};
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

fn morton_3d(pos: Vec3, b_box: &BBox) -> u64 {
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



pub fn bvh_morton(hittables: &mut Vec<Arc<Hittable>>) -> Arc<Hittable> {
    let b_box = surround(hittables);
    let mut morton_codes: Vec<u64> = Vec::new();
    for i in hittables.iter() {
        morton_codes.push(morton_3d(i.get_mean_pos(), &b_box));
    }
    radix_sort(hittables, &mut morton_codes);
    return gen_tree(&hittables, &morton_codes, 0, morton_codes.len() - 1);
}

fn gen_tree(sorted_hittables: &Vec<Arc<Hittable>>, codes: &Vec<u64>, start: usize, end: usize) -> Arc<Hittable> {
    if end == start {
        return sorted_hittables[start].clone();
    }

    let b_box = surround(&sorted_hittables[start..=end].to_vec());
    if start + 1 == end {
        return Arc::from(Hittable::Bvh {
            bvh: Bvh {
                bounds: b_box,
                left: sorted_hittables[start].clone(),
                right: sorted_hittables[end].clone(),
            }
        });
    }
    let half = split(&codes, start, end);
    println!("Previous: {}, Now: {}", end - start, half - start);

    let left = gen_tree(sorted_hittables, codes, start, half);
    let right = gen_tree(sorted_hittables, codes, half + 1, end);
    return Arc::from(Hittable::Bvh {
        bvh: Bvh {
            bounds: b_box,
            left,
            right,
        }
    });
}

fn split(vec: &Vec<u64>, first: usize, last: usize) -> usize {
    let first_code = vec[first];
    let last_code = vec[last];
    if first_code == last_code {
        return (first + last) >> 1;
    }
    let common_prefix = (first_code ^ last_code).leading_zeros();

    // Binary search
    let mut split = first;
    let mut step = last - first;
    while {
        step = (step + 1) / 2;
        let new_split = split + step;
        if new_split < last {
            let split_code = vec[new_split];
            let split_prefix = (first_code ^ split_code).leading_zeros();
            if split_prefix > common_prefix {
                split = new_split;
            }
        }
        step > 1
    } {}


    return split;
}

fn get_max(vec: &Vec<u64>) -> u64 {
    let mut max = vec[0];
    for i in vec.iter() {
        max = *i.max(&max);
    }
    return max;
}

fn count_sort(values: &Vec<Arc<Hittable>>, codes: &Vec<u64>, exp: u64) -> (Vec<Arc<Hittable>>, Vec<u64>) {
    let mut count = [0; 10];
    let mut dup_values = values.clone();
    let mut dup_codes = codes.clone();
    for i in 0..values.len() {
        count[(codes[i] as u64 / exp % 10) as usize] += 1;
    }
    for i in 1..10 {
        count[i] += count[i - 1];
    }
    for i in (0..values.len()).rev() {
        let i1 = (codes[i] / exp) as usize % 10;
        if count[i1] == 0 {
            println!("ie");
        }
        dup_values[(count[i1] as usize) - 1] = values[i].clone();
        dup_codes[count[i1] as usize - 1] = codes[i].clone();
        count[((codes[i] as u64 / exp) % 10) as usize] -= 1;
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
        exp *= 10;
    }
}