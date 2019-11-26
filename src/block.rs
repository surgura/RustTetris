use vector::{ V2f32 };
use draw::Draw;
use opengl_graphics::GlGraphics;
use graphics::{ Context, Transformed };
use interpolate;

#[derive(Clone)]
#[derive(Copy)]
pub struct Block {
    draw_position: V2f32, // position where block is drawn
    interpolate_data: interpolate::InterpolateData,
    pixel_size: f32 // size of the square in pixels
}

impl Block {
    // interpolate current position towards goal position
    // speed_factor[0, 1], interpolation speed factor
    pub fn update_interpolation(&mut self, dt: f32) {
        interpolate::interpolate(&mut self.interpolate_data, &mut self.draw_position, dt)
    }

    pub fn set_goal_position(&mut self, goal: V2f32, animation_time: f32) {
        interpolate::set_goal(&mut self.interpolate_data, goal, animation_time);
    }

    pub fn new(draw_position: V2f32, pixel_size: f32) -> Block {
        return Self{
            draw_position: draw_position,
            interpolate_data: interpolate::InterpolateData::new(),
            pixel_size: pixel_size,
        };
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