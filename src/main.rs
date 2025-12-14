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
    Vec3::new(1.0, 0.0, 0.0)
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
    let mut rasterizer = Rasterizer::new(WIDTH, HEIGHT)
        .set_vertex_shader(vertex_shader)
        .set_fragment_shader(fragment_shader)
        .add_triangle(Triangle::new([Vec3::new(0.0, 0.5, 1.0), Vec3::new(0.5, -0.5, 1.0), Vec3::new(-0.5, -0.5, 1.0)]));
    let image = rasterizer.rasterize(State { time: 0.0 });
    image.save_to_file("image.ppm").expect("Fail");
}
