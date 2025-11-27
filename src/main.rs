mod rasterizer;
use rasterizer::{
    vertex::Triangle,
    rasterizer::Rasterizer
};
use glm::Vec3;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

#[derive(Clone)]
struct State {
    time: f32
}

fn fragment_shader(pos: Vec3, state: State) -> Vec3 {
    let depth = (pos.z + 1.0) * 0.5;
    let brightness = 1.0 - depth * 0.8;
    
    Vec3::new(brightness, brightness, brightness)
}

fn vertex_shader(pos: Vec3, state: State) -> Vec3 {
    let angle = state.time;
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    
    let x1 = pos.x * cos_a - pos.z * sin_a;
    let z1 = pos.x * sin_a + pos.z * cos_a;
    
    let angle_x = state.time * 0.5;
    let cos_x = angle_x.cos();
    let sin_x = angle_x.sin();
    
    let y2 = pos.y * cos_x - z1 * sin_x;
    let z2 = pos.y * sin_x + z1 * cos_x;
    
    let angle_z = state.time * 0.3;
    let cos_z = angle_z.cos();
    let sin_z = angle_z.sin();
    
    let x3 = x1 * cos_z - y2 * sin_z;
    let y3 = x1 * sin_z + y2 * cos_z;
    
    Vec3::new(x3, y3, z2)
}

fn main() {
    let p0 = Vec3::new(-0.5, -0.5, -0.5);
    let p1 = Vec3::new(0.5, -0.5, -0.5);
    let p2 = Vec3::new(0.5, 0.5, -0.5);
    let p3 = Vec3::new(-0.5, 0.5, -0.5);
    let p4 = Vec3::new(-0.5, -0.5, 0.5);
    let p5 = Vec3::new(0.5, -0.5, 0.5);
    let p6 = Vec3::new(0.5, 0.5, 0.5);
    let p7 = Vec3::new(-0.5, 0.5, 0.5);
    
    for i in 0..100 {
        let time = i as f32 * 0.1;
        
        let mut rasterizer = Rasterizer::new(WIDTH, HEIGHT);
        
        rasterizer = rasterizer.add_triangle(Triangle::new([p4, p5, p6]));
        rasterizer = rasterizer.add_triangle(Triangle::new([p4, p6, p7]));
        
        rasterizer = rasterizer.add_triangle(Triangle::new([p0, p2, p1]));
        rasterizer = rasterizer.add_triangle(Triangle::new([p0, p3, p2]));
        
        rasterizer = rasterizer.add_triangle(Triangle::new([p1, p2, p6]));
        rasterizer = rasterizer.add_triangle(Triangle::new([p1, p6, p5]));
        
        rasterizer = rasterizer.add_triangle(Triangle::new([p0, p7, p3]));
        rasterizer = rasterizer.add_triangle(Triangle::new([p0, p4, p7]));
        
        rasterizer = rasterizer.add_triangle(Triangle::new([p3, p6, p2]));
        rasterizer = rasterizer.add_triangle(Triangle::new([p3, p7, p6]));
        
        rasterizer = rasterizer.add_triangle(Triangle::new([p0, p1, p5]));
        rasterizer = rasterizer.add_triangle(Triangle::new([p0, p5, p4]));
        
        rasterizer = rasterizer
            .set_vertex_shader(vertex_shader)
            .set_fragment_shader(fragment_shader);
        
        let image = rasterizer.rasterize(State { time });
        let filename = format!("image-{i:03}.ppm");
        image.save_to_file(&filename).expect("Failed to save");
        println!("Generated {}", filename);
    }
}