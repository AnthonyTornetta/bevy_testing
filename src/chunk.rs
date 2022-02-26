use crate::blocks;
use crate::blocks::block;
use crate::blocks::BLOCK_AIR;
use bevy::prelude::*;
use std::mem;

pub const WIDTH: u16 = 16;
pub const HEIGHT: u16 = 16;
pub const LENGTH: u16 = 16;

#[derive(Component)]
pub struct Dirty;

#[derive(Component)]
pub struct NeedsGenerated;

#[derive(Component)]
pub struct Chunk
{
    blocks: [block::BlockID; (WIDTH * HEIGHT * LENGTH) as usize],
    neighbors: [Option<Box<Chunk>>; 6],

    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[inline]
fn flatten(x: u16, y: u16, z: u16) -> usize
{
    (x + WIDTH * (y + HEIGHT * z)) as usize
}

impl Chunk
{
    pub fn new(x: i32, y: i32, z: i32) -> Self
    {
        Chunk {
            blocks: [blocks::BLOCK_AIR.id; (WIDTH * HEIGHT * LENGTH) as usize],
            neighbors: [None, None, None, None, None, None],
            x,
            y,
            z,
        }
    }

    pub fn set_neighbor(&mut self, neighbor_position: usize, neighbor: Option<Box<Chunk>>)
    {
        self.neighbors[neighbor_position] = neighbor;
    }

    #[inline]
    pub fn has_block_u16(&self, x: u16, y: u16, z: u16) -> bool
    {
        self.within_blocks_u16(x, y, z)
            && self.blocks[flatten(x as u16, y as u16, z as u16)] != BLOCK_AIR.id
    }

    #[inline]
    pub fn has_block_i16(&self, x: i16, y: i16, z: i16) -> bool
    {
        self.within_blocks_i16(x, y, z)
            && self.blocks[flatten(x as u16, y as u16, z as u16)] != BLOCK_AIR.id
    }

    #[inline]
    pub fn within_blocks_u16(&self, x: u16, y: u16, z: u16) -> bool
    {
        x < WIDTH && y < HEIGHT && z < LENGTH
    }

    #[inline]
    pub fn within_blocks_i16(&self, x: i16, y: i16, z: i16) -> bool
    {
        x >= 0 && x < WIDTH as i16 && y >= 0 && y < HEIGHT as i16 && z >= 0 && z < LENGTH as i16
    }

    #[inline]
    pub fn block_at(&self, x: u16, y: u16, z: u16) -> &'static block::Block
    {
        blocks::from_id(self.blocks[flatten(x, y, z)])
    }

    #[inline]
    pub fn set_block(
        &mut self,
        x: u16,
        y: u16,
        z: u16,
        new_block: &'static block::Block,
        entity: Entity,
        commands: &mut Commands,
    )
    {
        let mut ent = commands.entity(entity);

        // Prevents duplicate "Dirty" components from being added
        ent.remove::<Dirty>();

        if self.blocks[flatten(x, y, z)] != new_block.id
        {
            ent.insert(Dirty {});
        }

        self.blocks[flatten(x, y, z)] = new_block.id;
    }
}
