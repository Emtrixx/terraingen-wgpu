use cgmath::prelude::*;
use cgmath::{Matrix4, Point3, Vector3};

use crate::noise::{Noise2D, PerlinNoise};
use crate::vertex::Vertex;

// struct Quad {
//     vertices: [Vertex; 4],
//     indices: [u16; 6],
// }

pub struct Plane {
    pub width: f32,
    pub height: f32,
    pub width_segments: u32,
    pub height_segments: u32,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Plane {
    pub fn new(width: f32, height: f32, width_segments: u32, height_segments: u32) -> Self {
        let segment_width = width / width_segments as f32;
        let segment_height = height / height_segments as f32;

        let arr_width = width_segments + 1;
        let arr_height = height_segments + 1;

        let mut vertices: Vec<Vertex> = Vec::new();
        for y in 0..arr_height {
            let y_pos = -height / 2.0 + y as f32 * segment_height;
            for x in 0..arr_width {
                let x_pos = -width / 2.0 + x as f32 * segment_width;
                vertices.push(Vertex {
                    position: [x_pos, 0.0, y_pos],
                    color: [0.5, 0.0, 0.5],
                });
            }
        }

        let mut indices: Vec<u32> = Vec::new();
        for y in 0..height_segments {
            for x in 0..width_segments {
                let a = x + y * arr_width;
                let b = x + 1 + y * arr_width;
                let c = x + (y + 1) * arr_width;
                let d = x + 1 + (y + 1) * arr_width;
                indices.extend([b, a, c, b, c, d]);
            }
        }

        Self {
            width,
            height,
            width_segments,
            height_segments,
            vertices,
            indices,
        }
    }

    pub fn apply_heightmap(&mut self) {
        // Recreating the vertices array with the heightmap. Could be improved by just updating the height and color values.

        // let mut positions: Vec<[f32; 3]> = Vec::new();
        let perlin = PerlinNoise::new(0, 256, 0.5, 1.0, 3, 0.5, 2.0);

        let segment_width = self.width / self.width_segments as f32;
        let segment_height = self.height / self.height_segments as f32;

        let arr_width = self.width_segments + 1;
        let arr_height = self.height_segments + 1;

        let mut vertices: Vec<Vertex> = Vec::new();
        for y in 0..arr_height {
            let y_pos = -self.height / 2.0 + y as f32 * segment_height;
            for x in 0..arr_width {
                let x_pos = -self.width / 2.0 + x as f32 * segment_width;
                let height = perlin.get_noise(x_pos, y_pos);

                let color = get_color(height);
                // positions.push([x_pos, 0.0, y_pos]);
                vertices.push(Vertex {
                    position: [x_pos, height * 3.0, y_pos],
                    color: color,
                });
            }
        }

        self.vertices = vertices;
    }
}

fn get_color(height: f32) -> [f32; 3] {
    let mut color = [1.0, 1.0, 1.0];
    if height < 0.0 {
        color = [0.0, 0.0, 1.0];
    } else if height < 0.1 {
        color = [0.0, 1.0, 0.0];
    } else if height < 0.2 {
        color = [1.0, 1.0, 0.0];
    } else if height < 0.3 {
        color = [1.0, 0.5, 0.0];
    } else if height < 0.4 {
        color = [1.0, 0.0, 0.0];
    }
    color
}
