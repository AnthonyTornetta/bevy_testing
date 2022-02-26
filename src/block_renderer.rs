use crate::blocks::block;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_resource::PrimitiveTopology;

use crate::base_renderable;
use crate::base_renderable::CanCreateSubMesh;
use crate::blocks::block::Side;

pub const U_WIDTH: f32 = 0.5;
pub const V_HEIGHT: f32 = 0.5;

const DEFAULT_FRONT_INFO: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([-0.5, -0.5, 0.5], [0., 0., 1.0], [0., 1.0]),
    ([0.5, -0.5, 0.5], [0., 0., 1.0], [1.0, 1.0]),
    ([0.5, 0.5, 0.5], [0., 0., 1.0], [1.0, 0.0]),
    ([-0.5, 0.5, 0.5], [0., 0., 1.0], [0., 0.0]),
];

const DEFAULT_FRONT_INDICES: [u32; 6] = [0, 1, 2, 2, 3, 0];

const DEFAULT_BACK_INFO: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([-0.5, 0.5, -0.5], [0., 0., -1.0], [0., 1.0]),
    ([0.5, 0.5, -0.5], [0., 0., -1.0], [1.0, 1.0]),
    ([0.5, -0.5, -0.5], [0., 0., -1.0], [1.0, 0.0]),
    ([-0.5, -0.5, -0.5], [0., 0., -1.0], [0., 0.0]),
];

const DEFAULT_BACK_INDICES: [u32; 6] = [4, 5, 6, 6, 7, 4];

const DEFAULT_RIGHT_INFO: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([0.5, -0.5, -0.5], [1.0, 0., 0.], [1.0, 1.0]),
    ([0.5, 0.5, -0.5], [1.0, 0., 0.], [1.0, 0.0]),
    ([0.5, 0.5, 0.5], [1.0, 0., 0.], [0.0, 0.0]),
    ([0.5, -0.5, 0.5], [1.0, 0., 0.], [0.0, 1.0]),
];

const DEFAULT_RIGHT_INDICES: [u32; 6] = [8, 9, 10, 10, 11, 8];

const DEFAULT_LEFT_INFO: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([-0.5, -0.5, 0.5], [-1.0, 0., 0.], [1.0, 1.0]),
    ([-0.5, 0.5, 0.5], [-1.0, 0., 0.], [1.0, 0.0]),
    ([-0.5, 0.5, -0.5], [-1.0, 0., 0.], [0.0, 0.0]),
    ([-0.5, -0.5, -0.5], [-1.0, 0., 0.], [0.0, 1.0]),
];

const DEFAULT_LEFT_INDICES: [u32; 6] = [12, 13, 14, 14, 15, 12];

const DEFAULT_TOP_INFO: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([0.5, 0.5, -0.5], [0., 1.0, 0.], [1.0, 0.]),
    ([-0.5, 0.5, -0.5], [0., 1.0, 0.], [0., 0.]),
    ([-0.5, 0.5, 0.5], [0., 1.0, 0.], [0., 1.0]),
    ([0.5, 0.5, 0.5], [0., 1.0, 0.], [1.0, 1.0]),
];

const DEFAULT_TOP_INDICES: [u32; 6] = [16, 17, 18, 18, 19, 16];

const DEFAULT_BOTTOM_INFO: [([f32; 3], [f32; 3], [f32; 2]); 4] = [
    ([0.5, -0.5, 0.5], [0., -1.0, 0.], [1.0, 0.]),
    ([-0.5, -0.5, 0.5], [0., -1.0, 0.], [0., 0.]),
    ([-0.5, -0.5, -0.5], [0., -1.0, 0.], [0., 1.0]),
    ([0.5, -0.5, -0.5], [0., -1.0, 0.], [1.0, 1.0]),
];

const DEFAULT_BOTTOM_INDICES: [u32; 6] = [20, 21, 22, 22, 23, 20];

#[inline]
fn apply_offset<T: HasUVs>(
    default_info: [([f32; 3], [f32; 3], [f32; 2]); 4],
    offset: &Vec3,
    has_uvs: &T,
    side: Side,
) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
{
    let mut res = Vec::with_capacity(default_info.len());

    for mut item in default_info
    {
        item.0[0] += offset.x;
        item.0[1] += offset.y;
        item.0[2] += offset.z;

        item.2[0] = has_uvs.u_width(side) * item.2[0] + has_uvs.u_min(side);
        item.2[1] = has_uvs.v_height(side) * item.2[1] + has_uvs.v_min(side);

        res.push(item);
    }

    res
}

pub trait HasUVs
{
    fn u_min(&self, side: block::Side) -> f32;
    fn u_width(&self, _side: block::Side) -> f32
    {
        U_WIDTH
    }
    fn v_min(&self, side: block::Side) -> f32;
    fn v_height(&self, _side: block::Side) -> f32
    {
        V_HEIGHT
    }
}

pub struct DefaultBlockMeshCreator<T: HasUVs>
{
    uv_chooser: T,
}

impl<T: HasUVs> DefaultBlockMeshCreator<T>
{
    pub fn new(uv_chooser: T) -> Self
    {
        DefaultBlockMeshCreator { uv_chooser }
    }
}

impl<T: HasUVs> base_renderable::CanCreateSubMesh for DefaultBlockMeshCreator<T>
{
    fn right_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        apply_offset(DEFAULT_RIGHT_INFO, offset, &self.uv_chooser, Side::RIGHT)
    }

    fn right_indices(&self) -> Vec<u32>
    {
        Vec::from(DEFAULT_FRONT_INDICES)
    }

    fn left_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        apply_offset(DEFAULT_LEFT_INFO, offset, &self.uv_chooser, Side::LEFT)
    }

    fn left_indices(&self) -> Vec<u32>
    {
        Vec::from(DEFAULT_FRONT_INDICES)
    }

    fn top_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        apply_offset(DEFAULT_TOP_INFO, offset, &self.uv_chooser, Side::TOP)
    }

    fn top_indices(&self) -> Vec<u32>
    {
        Vec::from(DEFAULT_FRONT_INDICES)
    }

    fn bottom_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        apply_offset(DEFAULT_BOTTOM_INFO, offset, &self.uv_chooser, Side::BOTTOM)
    }

    fn bottom_indices(&self) -> Vec<u32>
    {
        Vec::from(DEFAULT_FRONT_INDICES)
    }

    fn front_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        apply_offset(DEFAULT_FRONT_INFO, offset, &self.uv_chooser, Side::FRONT)
    }

    fn front_indices(&self) -> Vec<u32>
    {
        Vec::from(DEFAULT_FRONT_INDICES)
    }

    fn back_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        apply_offset(DEFAULT_BACK_INFO, offset, &self.uv_chooser, Side::BACK)
    }

    fn back_indices(&self) -> Vec<u32>
    {
        Vec::from(DEFAULT_FRONT_INDICES)
    }
}

pub struct EmptyMeshCreator;

impl EmptyMeshCreator
{
    pub fn new() -> Self
    {
        EmptyMeshCreator {}
    }
}

impl base_renderable::CanCreateSubMesh for EmptyMeshCreator
{
    fn right_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        Vec::new()
    }

    fn right_indices(&self) -> Vec<u32>
    {
        Vec::new()
    }

    fn left_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        Vec::new()
    }

    fn left_indices(&self) -> Vec<u32>
    {
        Vec::new()
    }

    fn top_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        Vec::new()
    }

    fn top_indices(&self) -> Vec<u32>
    {
        Vec::new()
    }

    fn bottom_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        Vec::new()
    }

    fn bottom_indices(&self) -> Vec<u32>
    {
        Vec::new()
    }

    fn front_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        Vec::new()
    }

    fn front_indices(&self) -> Vec<u32>
    {
        Vec::new()
    }

    fn back_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>
    {
        Vec::new()
    }

    fn back_indices(&self) -> Vec<u32>
    {
        Vec::new()
    }
}
