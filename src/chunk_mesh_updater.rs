use crate::base_renderable::CanCreateMesh;
use crate::chunk::Dirty;
use crate::Chunk;
use bevy::ecs::component::{ComponentId, ComponentInfo};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;

pub struct ChunkMeshUpdaterPlugin;

fn check_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &Chunk, &Handle<Mesh>, Option<&Aabb>, With<Dirty>)>,
)
{
    for (entity, chunk, handle, aabb, _) in query.iter_mut()
    {
        println!("Checked a chunk!");

        let mesh = chunk.create_mesh();

        let _ = meshes.set(handle, mesh);

        let mut ent = commands.entity(entity);

        if !aabb.is_none()
        {
            ent.remove::<Aabb>();

            ent.insert(chunk.create_mesh().compute_aabb().unwrap());
        }
        ent.remove::<Dirty>();

        println!("Created Mesh!");
    }
}

impl Plugin for ChunkMeshUpdaterPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system(check_chunks);
    }
}
