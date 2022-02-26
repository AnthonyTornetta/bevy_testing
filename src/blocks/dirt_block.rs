use crate::block_renderer;
use crate::block_renderer::DefaultBlockMeshCreator;
use crate::blocks::block::{Block, Side};

pub struct DirtBlockCreator;

impl DirtBlockCreator
{
    pub fn create(id: u16) -> Block
    {
        Block {
            id,
            mesh_creator: Box::new(DefaultBlockMeshCreator::new(DirtUVs {})),
        }
    }
}

struct DirtUVs;

impl block_renderer::HasUVs for DirtUVs
{
    fn u_min(&self, _side: Side) -> f32
    {
        block_renderer::U_WIDTH
    }

    fn v_min(&self, _side: Side) -> f32
    {
        0.0
    }
}
