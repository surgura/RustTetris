mod draw;
mod grid;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use draw::Draw;

fn main() {
    let game_grid = grid::Grid::new();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "Tetris",
        [200, 200]
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(renderargs) = event.render_args() {
            gl.draw(renderargs.viewport(), |context, gl| {
                // clear screen.
                const GREEN: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
                graphics::clear(GREEN, gl);

                // draw grid
                game_grid.draw(&context, gl);
            });
        }

        /*
        if let Some(u) = e.update_args() {
            app.update(&u);
        }*/
    }
}