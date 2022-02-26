use crate::block_renderer;
use crate::block_renderer::DefaultBlockMeshCreator;
use crate::blocks::block::{Block, Side};

pub struct StoneBlockCreator;

impl StoneBlockCreator
{
    pub fn create(id: u16) -> Block
    {
        Block {
            id,
            mesh_creator: Box::new(DefaultBlockMeshCreator::new(StoneUVs {})),
        }
    }
}

struct StoneUVs;

impl block_renderer::HasUVs for StoneUVs
{
    fn u_min(&self, _side: Side) -> f32
    {
        0.0
    }

    fn v_min(&self, _side: Side) -> f32
    {
        0.0
    }
}
