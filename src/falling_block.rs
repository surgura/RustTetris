use vector::V2f32;
use draw::Draw;
use update::Update;
use opengl_graphics::GlGraphics;
use graphics::{ Context, Transformed };
use grid::Grid;

pub struct FallingBlock {
    shape: [[bool;Self::width() as usize];Self::height() as usize],
    position: V2f32 // left top position of shape in grid
}

impl FallingBlock {
    pub const fn width() -> u32 {
        return 4;
    }

    pub const fn height() -> u32 {
        return 4;
    }

    pub fn new() -> FallingBlock {
        return FallingBlock{
            shape: [
                [true, false, false, false],
                [false, false, false, false],
                [false, false, false, false],
                [false, false, false, false]
            ],
            position: V2f32{x: 0.0, y: 1.5}
        };
    }
}

impl Draw for FallingBlock {
    fn draw(&self, context: &Context, gl: &mut GlGraphics, scale: f32) {
        for y in 0..Self::height() {
            for x in 0..Self::width() {
                let transform = context.transform;
                let transform = transform.trans(
                    (Grid::cell_pixelsize() as f32 * scale * (x as f32 + self.position.x)) as f64,
                    (Grid::cell_pixelsize() as f32 * scale * (y as f32 + self.position.y)) as f64
                );

                let square = graphics::rectangle::square(0.0, 0.0, (Grid::cell_pixelsize() as f32 * scale) as f64);

                const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
                if self.shape[y as usize][x as usize] {
                    graphics::rectangle(BLUE, square, transform, gl);
                }
            }
        }
    }
}

impl Update for FallingBlock {
    fn update(&mut self, dt: f32) {
        let speed: f32 = 1.0f32 * dt;
        self.position.y += speed;
    }
}