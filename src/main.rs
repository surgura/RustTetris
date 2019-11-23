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

fn draw_grid(graphics: &mut GlGraphics, args: &RenderArgs, game_grid: &grid::Grid) {
    use graphics::*;

    const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];


    graphics.draw(args.viewport(), |c, gl| {
        // Clear the screen.
        clear(GREEN, gl);
        for y in 0..game_grid.data.len() {
            for x in 0..game_grid.data.len() {
                let transform = c.transform;
                let transform = transform.trans(10.0 + 42.0 * x as f64, 10.0 + 42.0 * y as f64);

                let square = rectangle::square(0.0, 0.0, 32.0);

                // Draw a box rotating around the middle of the screen.
                rectangle(BLUE, square, transform, gl);
            }
        }
    });
}

fn main() {
    let game_grid = grid::Grid{ data: [[false;4];4] } ;

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
        if let Some(render) = event.render_args() {
            draw_grid(&mut gl, &render, &game_grid);
        }

        /*
        if let Some(u) = e.update_args() {
            app.update(&u);
        }*/
    }
}