use std::f64::consts::PI;

pub fn deg_to_rad(deg: f64) -> f64 {
    return deg * PI / 180.0;
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}

pub fn random_double(min: f64, max: f64) -> f64 {
    return min + (max - min) * rand::random::<f64>();
}

pub fn binary_split(vec: &Vec<u64>, first: usize, last: usize) -> usize {
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
