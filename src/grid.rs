use draw;
use graphics;
use opengl_graphics::GlGraphics;
use graphics::{ Context, Transformed };

pub struct Grid {
	pub data : [[bool;Self::width() as usize];Self::height() as usize]
}

impl Grid {
    pub const fn width() -> u32 {
        return 10;
    }

    pub const fn height() -> u32 {
        return 20;
    }

    pub const fn cell_pixelsize() -> u32 {
        return 32;
    }

    pub const fn pixelwidth() -> u32 {
        return Self::width() * Self::cell_pixelsize();
    }

    pub const fn pixelheight() -> u32 {
        return Self::height() * Self::cell_pixelsize();
    }

    pub const fn new() -> Grid {
        return Grid{ data: [[false;Self::width() as usize];Self::height() as usize] }
    }
}

impl draw::Draw for Grid {
    fn draw(&self, context: &Context, gl: &mut GlGraphics, scale: f32) {
        for y in 0..Self::height() {
            for x in 0..Self::width() {
                let transform = context.transform;
                let transform = transform.trans(
                    (Self::cell_pixelsize() as f32 * scale * x as f32) as f64,
                    (Self::cell_pixelsize() as f32 * scale * y as f32) as f64
                );

                let square = graphics::rectangle::square(0.0, 0.0, (Self::cell_pixelsize() as f32 * scale) as f64);

                const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
                if self.data[y as usize][x as usize] {
                    graphics::rectangle(BLUE, square, transform, gl);
                }
                //graphics::line(color: types::Color, radius: types::Radius, line: L, transform: math::Matrix2d, g: &mut G)
            }
        }
    }
}