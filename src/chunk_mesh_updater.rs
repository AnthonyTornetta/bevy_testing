use crate::base_renderable::CanCreateMesh;
use crate::chunk::Dirty;
use crate::{Chunk, WIDTH, HEIGHT, LENGTH};
use bevy::ecs::component::{ComponentId, ComponentInfo};
use bevy::prelude::*;
use bevy::render::primitives::Aabb;
use bevy_rapier3d::prelude::*;

pub struct ChunkMeshUpdaterPlugin;

fn add_collider(start_x: f32, start_y: f32, start_z: f32, count: u16, colliders: &mut Vec<(Isometry<Real>, SharedShape)>)
{
    let length_size = LENGTH * HEIGHT;
    let height_size = HEIGHT;

    // make collider
    let length = count / (length_size);
    let height = (count - length * length_size) / height_size;
    let width = count - height_size * height - length_size * length;

    let hw = (width + 1) as f32 / 2.0;
    let hh = (height + 1) as f32 / 2.0;
    let hl = (length + 1) as f32 / 2.0;

    colliders.push(
        (Isometry::translation(start_x + hw, start_y + hh, start_z + hl),
         ColliderShape::cuboid(hw, hh, hl)));
}

fn create_colliders(chunk: &Chunk) -> Vec<(Isometry<Real>, SharedShape)>
{
    let mut colliders: Vec<(Isometry<Real>, SharedShape)> = Vec::new();

    let mut start_x: f32 = 0.0;
    let mut start_y: f32 = 0.0;
    let mut start_z: f32 = 0.0;

    let mut count: u16 = 0;

    for z in 0..LENGTH
    {
        for y in 0..HEIGHT
        {
            for x in 0..WIDTH
            {
                if chunk.has_block_u16(x, y, z)
                {
                    if count == 0
                    {
                        start_x = x as f32 - WIDTH as f32 / 2.0;
                        start_y = y as f32 - HEIGHT as f32 / 2.0;
                        start_z = z as f32 - LENGTH as f32 / 2.0;
                    }
                    count += 1;
                }
                else if count != 0
                {
                    add_collider(start_x, start_y, start_z, count, &mut colliders);

                    count = 0;
                }
            }
        }
    }

    if count != 0
    {
        add_collider(start_x, start_y, start_z, count, &mut colliders);
    }

    colliders
}

fn check_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(Entity, &Chunk, &Handle<Mesh>, Option<&Aabb>, With<Dirty>)>,
)
{
    for (entity, chunk, handle, aabb, _) in query.iter_mut()
    {
        let mesh = chunk.create_mesh();

        let _ = meshes.set(handle, mesh);

        let mut ent = commands.entity(entity);

        if !aabb.is_none()
        {
            ent.remove::<Aabb>();

            ent.insert(chunk.create_mesh().compute_aabb().unwrap());
        }

        let colliders = create_colliders(&chunk);
        let len = colliders.len();
        ent.remove_bundle::<ColliderBundle>();

        ent.insert_bundle(ColliderBundle {
            // shape: ColliderShape::cuboid(8.0, 8.0, 8.0).into(),
            shape: ColliderShapeComponent::from(ColliderShape::compound(colliders)),
            collider_type: ColliderType::Solid.into(),
            // position: [xf, yf, zf].into(),
            material: ColliderMaterial { friction: 0.7, restitution: 0.3, ..Default::default() }.into(),
            mass_properties: ColliderMassProps::Density(2.0).into(),
            ..Default::default()
        });

        ent.remove::<Dirty>();

        println!("Created mesh with {} colliders!", len);
    }
}

impl Plugin for ChunkMeshUpdaterPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_system(check_chunks);
    }
}
