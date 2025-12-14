use glm::Vec3;
use crate::rasterizer::image::Image;
use crate::rasterizer::vertex::Triangle;

pub struct Rasterizer<T: Clone> {
    width: u32,
    height: u32,
    triangles: Vec<Triangle>,
    fragment_shader: fn(Vec3, T) -> Vec3,
    vertex_shader: fn(Vec3, T) -> Vec3,
}

impl<T> Rasterizer<T> where T: Clone {
    pub fn new(width: u32, height: u32) -> Self {
        Rasterizer {
            width,
            height,
            triangles: Vec::new(),
            fragment_shader: |color, _| color,
            vertex_shader: |pos, _| pos,
        }
    }

    pub fn add_triangle(mut self, triangle: Triangle) -> Self {
        self.triangles.push(triangle);
        self
    }

    pub fn set_vertex_shader(mut self, shader: fn(Vec3, T) -> Vec3) -> Self {
        self.vertex_shader = shader;
        self
    }

    pub fn set_fragment_shader(mut self, shader: fn(Vec3, T) -> Vec3) -> Self {
        self.fragment_shader = shader;
        self
    }

    pub fn rasterize(&mut self, state: T) -> Image {
        for triangle in &mut self.triangles {
            triangle.vertices[0] = (self.vertex_shader)(triangle.vertices[0], state.clone());
            triangle.vertices[1] = (self.vertex_shader)(triangle.vertices[1], state.clone());
            triangle.vertices[2] = (self.vertex_shader)(triangle.vertices[2], state.clone());
        }

        let mut image = Image::new(self.width, self.height);

        // depth buffer (may be return image :3 )
        let mut depth_buffer = vec![f32::INFINITY; (self.width * self.height) as usize];

        // epsilon (for float eq)
        let eps = 1e-6;

        for triangle in &self.triangles {
            // Преобразуем в экранные координаты
            let v0 = vertex_to_screen_space(triangle.vertices[0], self.width, self.height);
            let v1 = vertex_to_screen_space(triangle.vertices[1], self.width, self.height);
            let v2 = vertex_to_screen_space(triangle.vertices[2], self.width, self.height);

            let (x0, y0, z0) = v0;
            let (x1, y1, z1) = v1;
            let (x2, y2, z2) = v2;

            // area of triangle
            let area = (x1 - x0) * (y2 - y0) - (x2 - x0) * (y1 - y0);
            
            // Backface culling: if area <= 0 triangle look to us
            if area <= eps { continue; }
            
            // in future we will have to divide to area for normalizing, and now we just one time inverting area for performace
            let inv_area = 1.0 / area;

            // Bounding box ( triangle into minimal box )
            let min_x = (x0.min(x1).min(x2).max(0.0)).floor() as i32;
            let max_x = (x0.max(x1).max(x2).min(self.width as f32 - 1.0)).ceil() as i32;
            let min_y = (y0.min(y1).min(y2).max(0.0)).floor() as i32;
            let max_y = (y0.max(y1).max(y2).min(self.height as f32 - 1.0)).ceil() as i32;
            
            // iterating all pixels in bounding box
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    // edge function (is dot next to or behind line)
                    let w0 = (x1 - x as f32) * (y2 - y as f32) - (x2 - x as f32) * (y1 - y as f32);
                    let w1 = (x2 - x as f32) * (y0 - y as f32) - (x0 - x as f32) * (y2 - y as f32);
                    let w2 = (x0 - x as f32) * (y1 - y as f32) - (x1 - x as f32) * (y0 - y as f32);

                    // if w0, w1 and w2 greater then zero (in triangle)
                    if w0 >= -eps && w1 >= -eps && w2 >= -eps {
                        // barycentric coordinates
                        let alpha = w0 * inv_area;
                        let beta = w1 * inv_area;
                        let gamma = w2 * inv_area;
                        
                        // interpolating Z (barycentric coords also qualifier of distance to vertex in triangle)
                        let z = alpha * z0 + beta * z1 + gamma * z2;
                        
                        // checking z buffer (checking that we more approached to camera to render)
                        // now it really useless
                        let idx = (y as u32 * self.width + x as u32) as usize;
                        if z < depth_buffer[idx] {
                            depth_buffer[idx] = z;
                            
                            // painting (ALREADY!!!!)
                            let final_color = (self.fragment_shader)(
                                Vec3::new(x as f32, y as f32, z),
                                state.clone()
                            );
                            
                            image.set_pixel(x as u32, y as u32, final_color);
                        }
                    }
                }
            }
        }

        image
    }
}

fn vertex_to_screen_space(vertex: Vec3, width: u32, height: u32) -> (f32, f32, f32) {
    let x = (vertex.x + 1.0) * 0.5 * width as f32;
    let y = (1.0 - vertex.y) * 0.5 * height as f32;
    let z = vertex.z;
    (x, y, z)
}
