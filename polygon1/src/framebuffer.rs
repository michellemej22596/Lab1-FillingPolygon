use std::fs::File;
use std::io::Write;
use crate::color::Color;

pub struct FrameBuffer {
    width: usize,
    height: usize,
    buffer: Vec<u8>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![0; width * height * 3]; // Cada píxel tiene 3 bytes (RGB)
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

    pub fn save_as_bmp(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();

        // Encabezado BMP
        let file_size = 54 + self.buffer.len();
        let reserved = 0;
        let data_offset = 54;

        let header = [
            0x42, 0x4D, // BM
            (file_size & 0xFF) as u8, ((file_size >> 8) & 0xFF) as u8,
            ((file_size >> 16) & 0xFF) as u8, ((file_size >> 24) & 0xFF) as u8,
            (reserved & 0xFF) as u8, ((reserved >> 8) & 0xFF) as u8,
            ((reserved >> 16) & 0xFF) as u8, ((reserved >> 24) & 0xFF) as u8,
            (data_offset & 0xFF) as u8, ((data_offset >> 8) & 0xFF) as u8,
            ((data_offset >> 16) & 0xFF) as u8, ((data_offset >> 24) & 0xFF) as u8,
        ];

        // Información del encabezado
        let header_info = [
            40, 0, 0, 0, // Tamaño del encabezado de información
            (self.width & 0xFF) as u8, ((self.width >> 8) & 0xFF) as u8,
            ((self.width >> 16) & 0xFF) as u8, ((self.width >> 24) & 0xFF) as u8,
            (self.height & 0xFF) as u8, ((self.height >> 8) & 0xFF) as u8,
            ((self.height >> 16) & 0xFF) as u8, ((self.height >> 24) & 0xFF) as u8,
            1, 0,       // Número de planos
            24, 0,      // Bits por píxel
            0, 0, 0, 0, // Compresión
            0, 0, 0, 0, // Tamaño de imagen (sin compresión)
            0, 0, 0, 0, // Resolución horizontal (opcional)
            0, 0, 0, 0, // Resolución vertical (opcional)
            0, 0, 0, 0, // Colores en la paleta (opcional)
            0, 0, 0, 0, // Colores importantes (opcional)
        ];

        file.write_all(&header).unwrap();
        file.write_all(&header_info).unwrap();

        // Escribir el buffer (los píxeles deben escribirse en orden inverso de filas)
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let index = (y * self.width + x) * 3;
                file.write_all(&self.buffer[index..index + 3]).unwrap();
            }
        }
    }
}
