use bevy::prelude::*;

pub trait CanCreateSubMesh
{
    fn right_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>;
    fn right_indices(&self) -> Vec<u32>;

    fn left_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>;
    fn left_indices(&self) -> Vec<u32>;

    fn top_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>;
    fn top_indices(&self) -> Vec<u32>;

    fn bottom_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>;
    fn bottom_indices(&self) -> Vec<u32>;

    fn front_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>;
    fn front_indices(&self) -> Vec<u32>;

    fn back_mesh_data(&self, offset: &Vec3) -> Vec<([f32; 3], [f32; 3], [f32; 2])>;
    fn back_indices(&self) -> Vec<u32>;
}

pub trait CanCreateMesh
{
    fn create_mesh(&self) -> Mesh;
}
