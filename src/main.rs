/*
pub mod data;
mod projectile;

fn main() {
    println!("Hello, world!");
    projectile::run();
}
*/

use std::ops::Index;
use pixel_canvas::{Canvas, Color, input::MouseState, XY};

fn main() {
    // Configure the window that you want to draw in. You can add an event
    // handler to build interactive art. Input handlers for common use are
    // provided.
    let canvas = Canvas::new(512, 512)
        .title("Tile")
        .state(MouseState::new())
        .input(MouseState::handle_input);
    // The canvas will render for you at up to 60fps.
    canvas.render(|mouse, image| {
        // Modify the `image` based on your state.
        let width = image.width() as usize;
        for (y, row) in image.chunks_mut(width).enumerate() {
            for (_x, pixel) in row.iter_mut().enumerate() {
                *pixel = Color {
                    r: 128,
                    g: 0,
                    b: 0,
                }
            }
        }
        image.fill(Color { r: 255, g: 255, b: 255});
        for x in 1..=20 {
            for y in 1..=20 {
                let p = XY(x+190, y+190);
                image[p] = Color::rgb(255,0,0);
            }
        }
    });
}
