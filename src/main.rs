mod framebuffer;
mod line;
mod bmp;
mod polygon;

use crate::framebuffer::Framebuffer;
use crate::bmp::WriteBmp;
use crate::polygon::Polygon;
use std::io;

fn main() {
    let mut entrada = String::new();

    println!("Ingrese la opción que desea ver:\n 1. Poligino 1\n 2. Polígono 2\n 3. Polígono 3\n 4. Polígono 4\n 5. Polígono 5");
    io::stdin().read_line(&mut entrada).unwrap();
    let opcion: u32 = entrada.trim().parse().unwrap_or(0);

    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    match opcion {
        1 => {
            let poly1 = vec![
                (165, 380), (185, 360), (180, 330), (207, 345), (233, 330), (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
            ];

            framebuffer.draw_polygon(&poly1, 0xFFFFFF, 0x0000FF);
        }

        2 => {
            let poly2 = vec![
                (321, 335), (288, 286), (339, 251), (374, 302)
            ];

            framebuffer.draw_polygon(&poly2, 0xFFFFFF, 0x0000FF);
        }

        3 => {
            let poly3 = vec![
                (377, 249), (411, 197), (436, 249)
            ];

            framebuffer.draw_polygon(&poly3, 0xFFFFFF, 0x0000FF);
        }

        4 => {
            let poly4 = vec![
                (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
                (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
                (597, 215), (552, 214), (517, 144), (466, 180)
            ];

            framebuffer.draw_polygon(&poly4, 0xFFFFFF, 0x0000FF);
        }

        5 => {
            let poly4 = vec![
                (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
                (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
                (597, 215), (552, 214), (517, 144), (466, 180)
            ];

            let poly5 = vec![
                (682,175),
                (708,120),
                (735,148),
                (739,170),
            ];

            framebuffer.draw_polygon(&poly4, 0xFFFFFF, 0x0000FF);
            framebuffer.fill_polygon(&poly5, 0x000000);
            framebuffer.draw_outline(&poly5, 0xFFFFFF);
        }

        _ => println!("Opción inválida"),
    }
    framebuffer.render_buffer("out.bmp").unwrap();
} 
