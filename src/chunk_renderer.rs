use crate::base_renderable::{CanCreateMesh, CanCreateSubMesh};
use crate::chunk;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

fn apply_info(
    cur_index: i32,
    mesh_data: &Vec<([f32; 3], [f32; 3], [f32; 2])>,
    indices: &Vec<u32>,
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices_congreg: &mut Vec<u32>,
) -> i32
{
    let mut next_max_index = cur_index;

    for index in indices
    {
        indices_congreg.push((*index as i32 + cur_index + 1) as u32);
        next_max_index = std::cmp::max(*index as i32 + cur_index + 1, next_max_index);
    }

    for (position, normal, uv) in mesh_data
    {
        positions.push(*position);
        normals.push(*normal);
        uvs.push(*uv);
    }

    next_max_index
}

impl CanCreateMesh for chunk::Chunk
{
    fn create_mesh(&self) -> Mesh
    {
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();

        let mut indices_congreg = Vec::new();

        let mut cur_index: i32 = -1;

        for z in 0..chunk::LENGTH
        {
            for y in 0..chunk::HEIGHT
            {
                for x in 0..chunk::WIDTH
                {
                    if !self.has_block_u16(x, y, z)
                    {
                        continue;
                    }

                    let here = &self.block_at(x, y, z).mesh_creator;
                    let location = Vec3::new(x as f32, y as f32, z as f32);

                    if !self.has_block_i16(x as i16 - 1, y as i16, z as i16)
                    {
                        let indices = here.left_indices();
                        let mesh_data = here.left_mesh_data(&location);

                        cur_index = apply_info(
                            cur_index,
                            &mesh_data,
                            &indices,
                            &mut positions,
                            &mut normals,
                            &mut uvs,
                            &mut indices_congreg,
                        );
                    }

                    if !self.has_block_u16(x + 1, y, z)
                    {
                        let indices = here.right_indices();
                        let mesh_data = here.right_mesh_data(&location);

                        cur_index = apply_info(
                            cur_index,
                            &mesh_data,
                            &indices,
                            &mut positions,
                            &mut normals,
                            &mut uvs,
                            &mut indices_congreg,
                        );
                    }

                    if !self.has_block_i16(x as i16, y as i16 - 1, z as i16)
                    {
                        let indices = here.bottom_indices();
                        let mesh_data = here.bottom_mesh_data(&location);

                        cur_index = apply_info(
                            cur_index,
                            &mesh_data,
                            &indices,
                            &mut positions,
                            &mut normals,
                            &mut uvs,
                            &mut indices_congreg,
                        );
                    }

                    if !self.has_block_u16(x, y + 1, z)
                    {
                        let indices = here.top_indices();
                        let mesh_data = here.top_mesh_data(&location);

                        cur_index = apply_info(
                            cur_index,
                            &mesh_data,
                            &indices,
                            &mut positions,
                            &mut normals,
                            &mut uvs,
                            &mut indices_congreg,
                        );
                    }

                    if !self.has_block_i16(x as i16, y as i16, z as i16 - 1)
                    {
                        let indices = here.back_indices();
                        let mesh_data = here.back_mesh_data(&location);

                        cur_index = apply_info(
                            cur_index,
                            &mesh_data,
                            &indices,
                            &mut positions,
                            &mut normals,
                            &mut uvs,
                            &mut indices_congreg,
                        );
                    }

                    if !self.has_block_u16(x, y, z + 1)
                    {
                        let indices = here.front_indices();
                        let mesh_data = here.front_mesh_data(&location);

                        cur_index = apply_info(
                            cur_index,
                            &mesh_data,
                            &indices,
                            &mut positions,
                            &mut normals,
                            &mut uvs,
                            &mut indices_congreg,
                        );
                    }
                }
            }
        }

        let indices = Indices::U32(indices_congreg);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(indices));
        mesh
    }
}
