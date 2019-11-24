use vector::{ V2f32 };
use draw::Draw;
use opengl_graphics::GlGraphics;
use graphics::{ Context, Transformed };

#[derive(Clone)]
#[derive(Copy)]
pub struct Block {
    draw_position: V2f32, // position where block is drawn
    pixel_size: f32 // size of the square in pixels
}

impl Block {
    // interpolate current position towards goal position
    // speed_factor[0, 1], interpolation speed factor
    pub fn interpolate_towards(&mut self, goal: &V2f32, speed_factor: f32) {
        self.draw_position += (goal - &self.draw_position) * speed_factor;
    }

    pub fn new(draw_position: V2f32, pixel_size: f32) -> Block {
        return Self{draw_position: draw_position, pixel_size: pixel_size};
    }
}

impl Draw for Block {
    fn draw(&self, context: &Context, gl: &mut GlGraphics, scale: f32) {
        let transform = context.transform;
        let transform = transform.trans(
            (scale * self.draw_position.x) as f64,
            (scale * self.draw_position.y) as f64
        );

        let square = graphics::rectangle::square(0.0, 0.0, (self.pixel_size * scale) as f64);

        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        graphics::rectangle(BLUE, square, transform, gl);
    }
}