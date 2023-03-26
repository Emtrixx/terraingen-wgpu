use cgmath::{num_traits::ToPrimitive, InnerSpace, Vector2};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub trait Noise2D {
    fn get_noise(&self, x: f32, y: f32) -> f32;
}

pub struct PerlinNoise {
    pub seed: u32,
    pub size: usize,
    pub frequency: f32,
    pub amplitude: f32,
    pub octaves: u32,
    pub gain: f32,
    pub lacunarity: f32,
    permutation: Vec<usize>,
    rand_vectors: Vec<[f32; 2]>,
}

impl PerlinNoise {
    pub fn new(
        seed: u32,
        size: usize,
        frequency: f32,
        amplitude: f32,
        octaves: u32,
        gain: f32,
        lacunarity: f32,
    ) -> Self {
        let mut permutation: Vec<usize> = (0..size).collect();
        permutation.shuffle(&mut thread_rng());
        permutation.append(&mut permutation.clone());

        let mut rand_vectors: Vec<[f32; 2]> = Vec::new();
        for _ in 0..size {
            let angle = rand::random::<f32>() * 360.0;
            let x = angle.to_radians().cos();
            let y = angle.to_radians().sin();
            rand_vectors.push([x, y]);
        }

        Self {
            seed,
            size,
            frequency,
            amplitude,
            octaves,
            gain,
            lacunarity,
            permutation,
            rand_vectors,
        }
    }

    fn generate(&self, input: [f32; 2]) -> f32 {
        // Corner Vectors
        // TODO i32 -> u32?
        let x = (input[0].floor() % 255.).to_usize().unwrap();
        let y = (input[1].floor() % 255.).to_usize().unwrap();
        let x0 = input[0] - x as f32;
        let y0 = input[1] - y as f32;

        let top_right = [x0 - 1.0, y0 - 1.0];
        let top_left = [x0, y0 - 1.0];
        let bottom_right = [x0 - 1.0, y0];
        let bottom_left = [x0, y0];

        let value_top_right = self.permutation[self.permutation[x + 1] + y + 1];
        let value_top_left = self.permutation[self.permutation[x] + y + 1];
        let value_bottom_right = self.permutation[self.permutation[x + 1] + y];
        let value_bottom_left = self.permutation[self.permutation[x] + y];

        // let dot_top_right = get_constant_vector(value_top_right).dot(top_right.into());
        // let dot_top_left = get_constant_vector(value_top_left).dot(top_left.into());
        // let dot_bottom_right = get_constant_vector(value_bottom_right).dot(bottom_right.into());
        // let dot_bottom_left = get_constant_vector(value_bottom_left).dot(bottom_left.into());
        let dot_top_right = self
            .get_random_vector(value_top_right)
            .dot(top_right.into());
        let dot_top_left = self.get_random_vector(value_top_left).dot(top_left.into());
        let dot_bottom_right = self
            .get_random_vector(value_bottom_right)
            .dot(bottom_right.into());
        let dot_bottom_left = self
            .get_random_vector(value_bottom_left)
            .dot(bottom_left.into());

        let u = smooth_step(x0);
        let v = smooth_step(y0);

        let result = lerp(
            lerp(dot_bottom_left, dot_bottom_right, u),
            lerp(dot_top_left, dot_top_right, u),
            v,
        );

        return result;
    }

    fn get_random_vector(&self, v: usize) -> Vector2<f32> {
        return self.rand_vectors[v].into();
    }
}

impl Noise2D for PerlinNoise {
    fn get_noise(&self, x: f32, y: f32) -> f32 {
        let mut total = 0.0;
        let mut frequency = self.frequency;
        let mut amplitude = self.amplitude;
        let mut max_value = 0.0;

        for _ in 0..self.octaves {
            total += self.generate([x * frequency, y * frequency]) * amplitude;
            max_value += amplitude;
            amplitude *= self.gain;
            frequency *= self.lacunarity;
        }

        total / max_value
    }
}

fn get_constant_vector(v: usize) -> Vector2<f32> {
    let h = v & 3;
    if h == 0 {
        return Vector2::new(1.0, 1.0);
    } else if h == 1 {
        return Vector2::new(-1.0, 1.0);
    } else if h == 2 {
        return Vector2::new(-1.0, -1.0);
    } else {
        return Vector2::new(1.0, -1.0);
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + t * (b - a)
}

fn smooth_step(t: f32) -> f32 {
    ((6. * t - 15.) * t + 10.) * t * t * t
}
