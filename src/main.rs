mod draw;
mod grid;
mod falling_block;
mod vector;
mod update;
mod block;
mod interpolate;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate cgmath;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use draw::Draw;
use update::Update;

fn main() {
    let scale: f32 = 1.0;
    let mut game_grid = grid::Grid::new();
    //game_grid.data[2][2] = Some(block::Block::new(V2f32::new(64.0, 64.0), 32.0));
    //game_grid.data[2][2].as_mut().unwrap().set_goal_position(V2f32::new(32.0, 32.0), 2.0);

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
        "Tetris",
        [
            grid::Grid::pixelwidth() as u32,
            grid::Grid::pixelheight() as u32
        ]
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut gl = GlGraphics::new(opengl);

    let mut falling_block = falling_block::FallingBlock::new(&mut game_grid);

    let mut events = Events::new(EventSettings::new());
    while let Some(event) = events.next(&mut window) {
        if let Some(renderargs) = event.render_args() {
            gl.draw(renderargs.viewport(), |context, mut gl| {
                // clear screen.
                const BACKGROUND_COLOR: [f32; 4] = [0.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0, 1.0];
                graphics::clear(BACKGROUND_COLOR, gl);

                // draw grid
                game_grid.draw(&context, &mut gl, scale);
            });
        }

        if let Some(updateargs) = event.update_args() {
            falling_block.update(&mut game_grid, updateargs.dt as f32);
            game_grid.update(updateargs.dt as f32);
        }
    }
}