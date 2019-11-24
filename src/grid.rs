use draw;
use graphics;
use opengl_graphics::GlGraphics;
use graphics::{ Context, Transformed };

pub struct Grid {
	pub data : [[bool;Self::width()];Self::height()]
}

impl Grid {
    pub const fn width() -> usize {
        return 10;
    }

    pub const fn height() -> usize {
        return 20;
    }

    pub const fn new() -> Grid {
        return Grid{ data: [[false;Self::width()];Self::height()] }
    }
}

impl draw::Draw for Grid {
    fn draw(&self, context: &Context, gl: &mut GlGraphics) {
        for y in 0..self.data.len() {
            for x in 0..self.data.len() {
                let transform = context.transform;
                let transform = transform.trans(10.0 + 42.0 * x as f64, 10.0 + 42.0 * y as f64);

                let square = graphics::rectangle::square(0.0, 0.0, 32.0);

                const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
                graphics::rectangle(BLUE, square, transform, gl);
            }
        }
    }
}