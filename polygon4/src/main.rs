mod framebuffer;
mod bmp;
mod color;
mod line;

use framebuffer::FrameBuffer;
use color::Color;
use line::draw_line;

// Agregar esta función para rellenar el polígono con agujeros
fn fill_polygon(fb: &mut FrameBuffer, points: &[(i32, i32)], color: &Color, holes: &[&[(i32, i32)]]) {
    let mut nodes = Vec::new();
    let mut min_y = points[0].1;
    let mut max_y = points[0].1;

    // Encuentra el mínimo y el máximo de Y
    for point in points {
        if point.1 < min_y {
            min_y = point.1;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }

    // Escanear línea por línea desde min_y hasta max_y
    for y in min_y..=max_y {
        nodes.clear();

        // Construir una lista de nodos para el polígono principal
        let mut j = points.len() - 1;
        for i in 0..points.len() {
            if (points[i].1 < y && points[j].1 >= y) || (points[j].1 < y && points[i].1 >= y) {
                let x = points[i].0 + (y - points[i].1) * (points[j].0 - points[i].0) / (points[j].1 - points[i].1);
                nodes.push(x);
            }
            j = i;
        }

        // Construir listas de nodos para los agujeros y eliminarlos de los nodos del polígono principal
        for hole in holes {
            let mut hole_nodes = Vec::new();
            let mut j = hole.len() - 1;
            for i in 0..hole.len() {
                if (hole[i].1 < y && hole[j].1 >= y) || (hole[j].1 < y && hole[i].1 >= y) {
                    let x = hole[i].0 + (y - hole[i].1) * (hole[j].0 - hole[i].0) / (hole[j].1 - hole[i].1);
                    hole_nodes.push(x);
                }
                j = i;
            }
            hole_nodes.sort();
            for hole_node in hole_nodes {
                if let Some(pos) = nodes.iter().position(|&x| x == hole_node) {
                    nodes.remove(pos);
                }
            }
        }

        // Ordenar nodos
        nodes.sort();

        // Rellenar entre pares de nodos
        for n in (0..nodes.len()).step_by(2) {
            if n + 1 < nodes.len() {
                for x in nodes[n]..=nodes[n + 1] {
                    fb.set_pixel(x as usize, y as usize, color);
                }
            }
        }
    }
}

fn main() {
    let mut fb = FrameBuffer::new(800, 600);

    let points1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    // Rellenar el primer polígono con color amarillo
    fill_polygon(&mut fb, &points1, &Color::YELLOW, &[]);

    // Dibujar la orilla del primer polígono con color blanco
    for i in 0..points1.len() {
        let (x0, y0) = points1[i];
        let (x1, y1) = points1[(i + 1) % points1.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    let points2 = [
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    // Rellenar el segundo polígono con color azul
    fill_polygon(&mut fb, &points2, &Color::BLUE, &[]);

    // Dibujar la orilla del segundo polígono con color blanco
    for i in 0..points2.len() {
        let (x0, y0) = points2[i];
        let (x1, y1) = points2[(i + 1) % points2.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    let points3 = [
        (377, 249), (411, 197), (436, 249),
    ];

    // Rellenar el tercer polígono con color rojo
    fill_polygon(&mut fb, &points3, &Color::RED, &[]);

    // Dibujar la orilla del tercer polígono con color blanco
    for i in 0..points3.len() {
        let (x0, y0) = points3[i];
        let (x1, y1) = points3[(i + 1) % points3.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    let points4 = [
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36),
        (676, 37), (660, 52), (750, 145), (761, 179), (672, 192),
        (659, 214), (615, 214), (632, 230), (580, 230), (597, 215),
        (552, 214), (517, 144), (466, 180),
    ];

    let points5 = [
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    // Rellenar el cuarto polígono con color verde y agujero (polígono 5)
    fill_polygon(&mut fb, &points4, &Color::GREEN, &[&points5]);

    // Dibujar la orilla del cuarto polígono con color blanco
    for i in 0..points4.len() {
        let (x0, y0) = points4[i];
        let (x1, y1) = points4[(i + 1) % points4.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    // Dibujar la orilla del agujero (polígono 5) con color blanco
    for i in 0..points5.len() {
        let (x0, y0) = points5[i];
        let (x1, y1) = points5[(i + 1) % points5.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    fb.save_as_bmp("polygon_4.bmp");
}
