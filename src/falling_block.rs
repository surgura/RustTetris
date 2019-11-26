use vector::V2u32;
use vector::V2f32;
use block::Block;
use grid::Grid;

pub struct FallingBlock {
    blocks: Vec<V2u32>, // indices on grid of blocks that are part of this shape
    fall_progression: f32
}

impl FallingBlock {
    pub fn new(grid: &mut Grid) -> FallingBlock {
        let mut block = Block::new(
            V2f32::new(128.0, 128.0),
            32.0
        );
        block.set_goal_position(V2f32::new(128.0, 128.0), 0.0);
        grid.data[4][4] = Some(block);
        let mut blocks = Vec::<V2u32>::new();
        blocks.push(V2u32::new(4,4));

        return FallingBlock{
            blocks: blocks,
            fall_progression: 0.0
        };
    }

    pub fn update(&mut self, grid: &mut Grid, dt: f32) {
        let speed: f32 = 1.0f32 * dt;
        self.fall_progression += speed;

        let downcount: u32 = self.fall_progression as u32;
        self.fall_progression -= downcount as f32;

        for _ in 0..downcount {
            for block in &mut self.blocks {
                let block_data: &mut Block = grid.data[block.y as usize][block.x as usize].as_mut().unwrap();
                block_data.set_goal_position(V2f32::new(block.x as f32 * 32.0, block.y as f32 * 32.0 + 32.0), 0.3);
                grid.data[block.y as usize + 1][block.x as usize] = grid.data[block.y as usize][block.x as usize];
                grid.data[block.y as usize][block.x as usize] = None;
                block.y += 1;
            }
        }
    }
}