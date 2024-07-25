use crate::color::Color;
use std::fs::File;
use std::io::Write;

pub struct FrameBuffer {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
}

fn create_file_header(width: usize, height: usize) -> [u8; 14] {
    let file_size = 14 + 40 + (width * height * 3) as u32;
    [
        0x42, 0x4D, // Signature
        (file_size & 0xFF) as u8, ((file_size >> 8) & 0xFF) as u8, ((file_size >> 16) & 0xFF) as u8, ((file_size >> 24) & 0xFF) as u8, // File size
        0x00, 0x00, 0x00, 0x00, 
        0x36, 0x00, 0x00, 0x00, 
    ]
}

fn create_info_header(width: usize, height: usize) -> [u8; 40] {
    [
        0x28, 0x00, 0x00, 0x00, 
        (width & 0xFF) as u8, ((width >> 8) & 0xFF) as u8, ((width >> 16) & 0xFF) as u8, ((width >> 24) & 0xFF) as u8, // Width
        (height & 0xFF) as u8, ((height >> 8) & 0xFF) as u8, ((height >> 16) & 0xFF) as u8, ((height >> 24) & 0xFF) as u8, // Height
        0x01, 0x00, 
        0x18, 0x00, 
        0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 
        0x13, 0x0B, 0x00, 0x00, 
        0x13, 0x0B, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 
    ]
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height * 3]; 
        Self { width, height, buffer }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) * 3;
            self.buffer[index] = color.r;
            self.buffer[index + 1] = color.g;
            self.buffer[index + 2] = color.b;
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let index = (y * self.width + x) * 3;
        Color {
            r: self.buffer[index],
            g: self.buffer[index + 1],
            b: self.buffer[index + 2],
        }
    }

    pub fn save_as_bmp(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        
        let file_header = create_file_header(self.width, self.height);
        let info_header = create_info_header(self.width, self.height);

        file.write_all(&file_header).unwrap();
        file.write_all(&info_header).unwrap();

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pixel = self.get_pixel(x, y);
                file.write_all(&[pixel.b, pixel.g, pixel.r]).unwrap();
            }
        }
    }
}