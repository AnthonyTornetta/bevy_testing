use crate::block_renderer::EmptyMeshCreator;
use crate::blocks::block::Block;

pub struct AirBlockCreator;

impl AirBlockCreator
{
    pub fn create(id: u16) -> Block
    {
        Block {
            id,
            mesh_creator: Box::new(EmptyMeshCreator::new()),
        }
    }
}
