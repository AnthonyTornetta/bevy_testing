use crate::block_renderer;
use crate::block_renderer::DefaultBlockMeshCreator;
use crate::blocks::block::{Block, Side};

pub struct GrassBlockCreator;

impl GrassBlockCreator
{
    pub fn create(id: u16) -> Block
    {
        Block {
            id,
            mesh_creator: Box::new(DefaultBlockMeshCreator::new(GrassBlockUVs {})),
        }
    }
}

struct GrassBlockUVs;

impl block_renderer::HasUVs for GrassBlockUVs
{
    fn u_min(&self, side: Side) -> f32
    {
        match side
        {
            Side::TOP => block_renderer::U_WIDTH,
            Side::BOTTOM => block_renderer::U_WIDTH,
            _ => 0.0,
        }
    }

    fn v_min(&self, side: Side) -> f32
    {
        match side
        {
            Side::BOTTOM => 0.0,
            _ => block_renderer::V_HEIGHT,
        }
    }
}
