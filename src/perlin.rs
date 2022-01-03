use crate::utilities::{self, random_int};
use cliffy::{Vec3, Vector};

const POINT_COUNT: usize = 256;
pub struct Perlin {
    ran_vec: Vec<Vec3>,
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
}

impl Perlin {
    pub fn new() -> Self {
        let mut ran_vec = Vec::new();
        for _ in 0..POINT_COUNT {
            ran_vec.push(utilities::random_vec3_between(-1.0, 1.0));
        }

        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();

        Self {
            ran_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Vec3) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let ind_x = ((i + di as i32) & 255) as usize;
                    let ind_y = ((j + dj as i32) & 255) as usize;
                    let ind_z = ((k + dk as i32) & 255) as usize;
                    c[di][dj][dk] = self.ran_vec
                        [(self.perm_x[ind_x] ^ self.perm_y[ind_y] ^ self.perm_z[ind_z]) as usize];
                }
            }
        }

        Self::trilinear_interp(&c, u, v, w)
    }

    fn trilinear_interp(c: &[[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as f32;
                    let fj = j as f32;
                    let fk = k as f32;
                    let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                    accum += (fi * uu + (1.0 - fi) * (1.0 - uu))
                        * (fj * vv + (1.0 - fj) * (1.0 - vv))
                        * (fk * ww + (1.0 - fk) * (1.0 - ww))
                        * c[i][j][k].dot(weight_v);
                }
            }
        }

        accum
    }

    fn generate_perm() -> Vec<u32> {
        let mut p = Vec::new();

        for i in 0..POINT_COUNT {
            p.push(i as u32);
        }

        Self::permute(&mut p, POINT_COUNT);

        p
    }

    fn permute(p: &mut Vec<u32>, n: usize) {
        for i in (0..n).rev() {
            let target = random_int(0, i as i32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }

    pub fn turb(&self, p: &Vec3, depth: u32) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for i in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}
