use draw;
use opengl_graphics::GlGraphics;
use graphics::{ Context, Transformed };
use block;
use update;
use piston::UpdateArgs;
use vector::V2f32;

pub struct Grid {
	pub data : [[Option<block::Block>;Self::width() as usize];Self::height() as usize]
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
        return Grid{ data: [[None;Self::width() as usize];Self::height() as usize] }
    }
}

impl draw::Draw for Grid {
    fn draw(&self, context: &Context, gl: &mut GlGraphics, scale: f32) {
        for y in 0..Self::height() {
            for x in 0..Self::width() {
                if let Some(block) = &self.data[y as usize][x as usize] {
                    block.draw(context, gl, scale);
                }
            }
        }
    }
}

impl update::Update for Grid {
    fn update(&mut self, updateargs: &UpdateArgs) {
        const INTERPOLATE_FACTOR: f32 = 0.5;
        for y in 0..Self::height() {
            for x in 0..Self::width() {
                if let Some(block) = &mut self.data[y as usize][x as usize] {
                    block.interpolate_towards(
                        &V2f32::new(x as f32, y as f32),
                        (INTERPOLATE_FACTOR as f64 * updateargs.dt) as f32
                    )
                }
            }
        }
    }
}