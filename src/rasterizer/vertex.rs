use glm::Vec3;

pub struct Triangle {
    pub vertices: [Vec3; 3],
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3]) -> Self {
        Self {
            vertices: vertices,
        }
    }
}