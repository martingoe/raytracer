use rand::Rng;

const PERMUTATION: [i32; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

pub struct PerlinNoise {
    p: [i32; 512],
}

fn fade(x: f64) -> f64 {
    x * x * x * (x * (x * 6.0 - 15.0) + 10.0)
}

fn lerp(t: f64, a1: f64, a2: f64) -> f64 {
    a1 + t * (a2 - a1)
}

fn grad(hash: i32, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 15; // CONVERT LO 4 BITS OF HASH CODE

    let u = if h < 8 { x } else { y }; // INTO 12 GRADIENT DIRECTIONS.
    let v = if h < 4 {
        y
    } else {
        if h == 12 || h == 14 {
            x
        } else {
            z
        }
    };

    return (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v });
}

impl PerlinNoise {
    pub(crate) fn new() -> PerlinNoise {
        let mut p = [0; 512];
        for i in 0..256 {
            let i1 = rand::thread_rng().gen_range(0..=255);
            p[i] = PERMUTATION[i1];
            p[256 + i] = PERMUTATION[i1];
        }
        return PerlinNoise { p };
    }
    pub(crate) fn get_value(&self, mut x: f64, mut y: f64, mut z: f64) -> f64 {
        // See https://mrl.cs.nyu.edu/~perlin/paper445.pdf
        let ix = x.floor() as i32 & 255;
        let iy = y.floor() as i32 & 255;
        let iz = z.floor() as i32 & 255;

        x = x - x.floor();
        y = y - y.floor();
        z = z - z.floor();

        let u = fade(x);
        let v = fade(y);
        let w = fade(z);

        let a = (self.p[ix as usize] + iy) as usize;
        let aa = (self.p[a as usize] + iz) as usize;
        let ab = (self.p[a as usize + 1] + iz) as usize;
        let b = (self.p[ix as usize + 1] + iy) as usize;
        let ba = (self.p[b as usize] + iz) as usize;
        let bb = (self.p[b as usize + 1] + iz) as usize;

        return lerp(
            w,
            lerp(
                v,
                lerp(
                    u,
                    grad(self.p[aa], x, y, z),
                    grad(self.p[ba], x - 1.0, y, z),
                ),
                lerp(
                    u,
                    grad(self.p[ab], x, y - 1.0, z),
                    grad(self.p[bb], x - 1.0, y - 1.0, z),
                ),
            ),
            lerp(
                v,
                lerp(
                    u,
                    grad(self.p[aa + 1], x, y, z - 1.0),
                    grad(self.p[ba + 1], x - 1.0, y, z - 1.0),
                ),
                lerp(
                    u,
                    grad(self.p[ab + 1], x, y - 1.0, z - 1.0),
                    grad(self.p[bb + 1], x - 1.0, y - 1.0, z - 1.0),
                ),
            ),
        );
    }
}
