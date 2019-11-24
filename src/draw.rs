use opengl_graphics::GlGraphics;
use graphics::Context;

pub trait Draw {
    fn draw(&self, context: &Context, gl: &mut GlGraphics);
}