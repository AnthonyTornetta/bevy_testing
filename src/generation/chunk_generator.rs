use crate::blocks::block::Block;
use crate::blocks::{BLOCK_DIRT, BLOCK_GRASS, BLOCK_STONE};
use crate::chunk::{HEIGHT, LENGTH, WIDTH};
use crate::{Chunk, Commands, Entity, NeedsGenerated};
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct ChunkGeneratorPlugin;

impl Plugin for ChunkGeneratorPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system(chunk_generation_system);
    }
}

fn generate(chunk: &mut Chunk, entity: Entity, commands: &mut Commands)
{
    commands.entity(entity).remove::<NeedsGenerated>();

    let perlin = Perlin::new();

    for z in 0..LENGTH
    {
        for x in 0..WIDTH
        {
            let max_y = HEIGHT as i32 - 3
                + f64::round(
                    3.0 * perlin.get([
                        (x as i32 + chunk.x) as f64 * 0.1,
                        (z as i32 + chunk.z) as f64 * 0.1,
                    ]),
                ) as i32;

            for y in 0..max_y
            {
                let b: &Block;

                if y == max_y - 1
                {
                    b = &BLOCK_GRASS;
                }
                else if y > max_y - 5
                {
                    b = &BLOCK_DIRT;
                }
                else
                {
                    b = &BLOCK_STONE;
                }

                chunk.set_block(x, y as u16, z, b, entity, commands);
            }
        }
    }
}

fn chunk_generation_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Chunk, With<NeedsGenerated>)>,
)
{
    for (ent, mut chunk, _) in query.iter_mut()
    {
        generate(&mut chunk, ent, &mut commands);
    }
}
