use vector::{ V2f32 };
use draw::Draw;
use opengl_graphics::GlGraphics;
use graphics::{ Context, Transformed };

#[derive(Clone)]
#[derive(Copy)]
pub struct Block {
    draw_position: V2f32, // position where block is drawn
    animation_timeleft: f32, // seconds left before animation should end
    animation_goal: V2f32,
    pixel_size: f32 // size of the square in pixels
}

impl Block {
    // interpolate current position towards goal position
    // speed_factor[0, 1], interpolation speed factor
    pub fn update_interpolation(&mut self, dt: f64) {
        if dt as f32 >= self.animation_timeleft {
            self.animation_timeleft = 0.0;
            self.draw_position = self.animation_goal;
        } else {
            let distance = self.animation_goal - self.draw_position;
            self.draw_position +=
                2.0f32 * distance / self.animation_timeleft * dt as f32 - 
                distance / (self.animation_timeleft * self.animation_timeleft) * dt as f32 * dt as f32;
            self.animation_timeleft -= dt as f32;
        }
    }

    pub fn set_goal_position(&mut self, goal: V2f32, animation_time: f32) {
        self.animation_timeleft = animation_time;
        self.animation_goal = goal;
    }

    pub fn new(draw_position: V2f32, pixel_size: f32) -> Block {
        return Self{
            draw_position: draw_position,
            pixel_size: pixel_size,
            animation_goal: draw_position,
            animation_timeleft: 0.0
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