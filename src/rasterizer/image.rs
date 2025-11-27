use glm::Vec3;

pub struct Image {
    width: u32,
    height: u32,
    pub pixels: Vec<Vec3>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            pixels: vec![Vec3::new(0.0, 0.0, 0.0); (height * width) as usize],
        }
    }

    pub fn get_resolution(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn format_to_ppm(&self) -> String {
        let mut result = String::new();
        
        // PPM header
        result.push_str(&format!("P3\n{} {}\n255\n", self.width, self.height));
        
        // Iterate through rows (y), then columns (x)
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                let pixel = &self.pixels[idx];
                
                // Convert from float [0.0, 1.0] to integer [0, 255]
                let r = (pixel.x.clamp(0.0, 1.0) * 255.0).round() as u32;
                let g = (pixel.y.clamp(0.0, 1.0) * 255.0).round() as u32;
                let b = (pixel.z.clamp(0.0, 1.0) * 255.0).round() as u32;
                
                result.push_str(&format!("{} {} {} ", r, g, b));
            }
            result.push('\n');
        }
        
        result
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Vec3) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.pixels[idx] = color;
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Vec3 {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.pixels[idx]
        } else {
            Vec3::new(0.0, 0.0, 0.0)
        }
    }


    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;
        
        let mut file = File::create(filename)?;
        file.write_all(self.format_to_ppm().as_bytes())?;
        Ok(())
    }
}