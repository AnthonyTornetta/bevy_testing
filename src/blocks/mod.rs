use crate::blocks::air_block::AirBlockCreator;
use crate::blocks::block::{Block, BlockID};
use crate::blocks::dirt_block::DirtBlockCreator;
use crate::blocks::grass_block::GrassBlockCreator;
use crate::blocks::stone_block::StoneBlockCreator;

mod air_block;
pub mod block;
mod dirt_block;
mod grass_block;
mod stone_block;
//
// static mut MASTER_BLOCK_ID: BlockID = 0;
//
// // will only ever be called on the same thread.  Therefore, it is assumed that it is completely safe
// fn next_id() -> BlockID
// {
//     unsafe {
//         let res = MASTER_BLOCK_ID;
//
//         MASTER_BLOCK_ID += 1;
//
//         res
//     }
// }

lazy_static! {
    pub static ref BLOCK_AIR: Block = AirBlockCreator::create(0);
    pub static ref BLOCK_STONE: Block = StoneBlockCreator::create(1);
    pub static ref BLOCK_GRASS: Block = GrassBlockCreator::create(2);
    pub static ref BLOCK_DIRT: Block = DirtBlockCreator::create(3);
    pub static ref BLOCKS: [&'static block::Block; 4] =
        [&BLOCK_AIR, &BLOCK_STONE, &BLOCK_GRASS, &BLOCK_DIRT];
}

pub fn from_id(id: BlockID) -> &'static Block
{
    &BLOCKS[id as usize]
}
