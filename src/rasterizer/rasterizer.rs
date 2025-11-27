use glm::Vec3;
use crate::rasterizer::image::Image;
use crate::rasterizer::vertex::Triangle;
pub struct Rasterizer <T: Clone> {
    width: u32,
    height: u32,
    triangles: Vec<Triangle>,
    fragment_shader: fn(Vec3, T) -> Vec3,
    vertex_shader: fn(Vec3, T) -> Vec3,
}

impl<T> Rasterizer<T> where T: Clone{
    pub fn new(width: u32, height: u32) -> Self {
        Rasterizer {
            width: width,
            height: height,
            triangles: Vec::new(),
            fragment_shader: |color, state| color,
            vertex_shader: |pos, state| pos,
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

        for triangle in &self.triangles {
            let up_scr = vertex_to_screen_space(triangle.vertices[0], self.width, self.height);
            let mid_scr = vertex_to_screen_space(triangle.vertices[1], self.width, self.height);
            let down_scr = vertex_to_screen_space(triangle.vertices[2], self.width, self.height);

            let (up, mid, down) = sort_vertices_by_y_screen(up_scr, mid_scr, down_scr);

            let (upx, upy, upz) = up;
            let (midx, midy, midz) = mid;
            let (downx, downy, downz) = down;

            // up side
            for y in upy as i32..=midy as i32 {
                if y < 0 || y >= self.height as i32 { continue; }

                let t_segment = if (midy - upy).abs() > 0.001 {
                    (y as f32 - upy) / (midy - upy)
                } else { 0.5 };
                
                let t_total = if (downy - upy).abs() > 0.001 {
                    (y as f32 - upy) / (downy - upy)
                } else { 0.5 };

                let left_x = upx + (midx - upx) * t_segment;
                let right_x = upx + (downx - upx) * t_total;

                let (start_x, end_x) = if left_x < right_x {
                    (left_x as i32, right_x as i32)
                } else {
                    (right_x as i32, left_x as i32)
                };

                for x in start_x..=end_x {
                    if x < 0 || x >= self.width as i32 { continue; }
                    let color = (self.fragment_shader)(Vec3::new(x as f32, y as f32, 0.0), state.clone());
                    image.set_pixel(x as u32, y as u32, color);
                }
            }

            // down side  
            for y in midy as i32..=downy as i32 {
                if y < 0 || y >= self.height as i32 { continue; }

                let t_segment = if (downy - midy).abs() > 0.001 {
                    (y as f32 - midy) / (downy - midy)
                } else { 0.5 };
                
                let t_total = if (downy - upy).abs() > 0.001 {
                    (y as f32 - upy) / (downy - upy)
                } else { 0.5 };

                let left_x = midx + (downx - midx) * t_segment;
                let right_x = upx + (downx - upx) * t_total;

                let (start_x, end_x) = if left_x < right_x {
                    (left_x as i32, right_x as i32)
                } else {
                    (right_x as i32, left_x as i32)
                };

                for x in start_x..=end_x {
                    if x < 0 || x >= self.width as i32 { continue; }
                    let color = (self.fragment_shader)(Vec3::new(x as f32, y as f32, 0.0), state.clone());
                    image.set_pixel(x as u32, y as u32, color);
                }
            }
        }
    
        image
    }
}

fn sort_vertices_by_y_screen(a: (f32, f32, f32), b: (f32, f32, f32), c: (f32, f32, f32)) -> ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32)) {
        let mut vertices = [a, b, c];
        
        if vertices[0].1 > vertices[1].1 {
            vertices.swap(0, 1);
        }
        if vertices[1].1 > vertices[2].1 {
            vertices.swap(1, 2);
        }
        if vertices[0].1 > vertices[1].1 {
            vertices.swap(0, 1);
        }
        
        (vertices[0], vertices[1], vertices[2])
    }

fn vertex_to_screen_space(vertex: Vec3, width: u32, height: u32) -> (f32, f32, f32) {
    let x = (vertex.x + 1.0) * 0.5 * width as f32;
    let y = (1.0 - vertex.y) * 0.5 * height as f32; 
    let z = vertex.z;
    (x, y, z)
}
fn abs<T>(a: T) -> T 
where 
    T: PartialOrd + From<u8> + std::ops::Neg<Output = T> + Copy,
{
    if a < T::from(0) {
        -a
    } else {
        a
    }
}