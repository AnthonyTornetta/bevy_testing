use crate::base_renderable::CanCreateSubMesh;

pub type BlockID = u16;

pub struct Block
{
    pub id: BlockID,

    pub mesh_creator: Box<dyn CanCreateSubMesh + Send + Sync + 'static>,
}

impl PartialEq for Block
{
    fn eq(&self, other: &Self) -> bool
    {
        self.id == other.id
    }
}

#[derive(Clone, Copy)]
pub enum Side
{
    LEFT,
    RIGHT,
    TOP,
    BOTTOM,
    FRONT,
    BACK,
}

impl Block
{
    pub fn new(id: u16, mesh_creator: Box<dyn CanCreateSubMesh + Send + Sync + 'static>) -> Self
    {
        Block { id, mesh_creator }
    }
}
