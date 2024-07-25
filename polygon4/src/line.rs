use crate::framebuffer::FrameBuffer;
use crate::color::Color;

// Dibujar una línea entre dos puntos (x0, y0) y (x1, y1) usando el algoritmo de Bresenham
pub fn draw_line(fb: &mut FrameBuffer, x0: i32, y0: i32, x1: i32, y1: i32, color: &Color) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x0 = x0;
    let mut y0 = y0;

    loop {
        fb.set_pixel(x0 as usize, y0 as usize, color);
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}