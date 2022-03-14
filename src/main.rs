#[macro_use]
extern crate lazy_static;

mod blocks;
mod chunk;

mod base_renderable;
mod block_renderer;
mod chunk_renderer;

mod chunk_mesh_updater;
mod epic_plugin;
mod generation;

use crate::base_renderable::CanCreateMesh;
use crate::chunk::{Chunk, HEIGHT, LENGTH, NeedsGenerated, WIDTH};
use crate::generation::chunk_generator;
use bevy::ecs::component::ComponentInfo;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use std::ops::Mul;
use bevy_rapier3d::prelude::*;
use rand::random;
use crate::KeyCode::D;
use crate::nalgebra::Isometry3;

fn main()
{
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_system(print_heights)
        .add_system(camera_movement_system)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration
        {
            gravity: vector![0.0,0.0,0.0],
            ..Default::default()
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(chunk_mesh_updater::ChunkMeshUpdaterPlugin)
        .add_plugin(chunk_generator::ChunkGeneratorPlugin)
        .run();
}

pub fn camera_movement_system(
    mut query: Query<(&Camera, &mut Transform)>,
    mut ev_motion: EventReader<MouseMotion>,
    keys: Res<Input<KeyCode>>,
)
{
    let (_cam, mut transform) = query.single_mut();

    let speed = (keys.pressed(KeyCode::LShift) as i32 * 5 + 1) as f32 * 0.01;

    for x in ev_motion.iter()
    {
        let sens: f32 = 0.001;

        let temp: Quat = Quat::from_euler(
            EulerRot::XYZ,
            -x.delta.y as f32 * sens,
            -x.delta.x as f32 * sens,
            0.0,
        );

        transform.rotation = temp.mul(transform.rotation).normalize();
    }

    transform.translation.x +=
        (keys.pressed(KeyCode::D) as i32 - keys.pressed(KeyCode::A) as i32) as f32 * speed;
    transform.translation.y +=
        (keys.pressed(KeyCode::E) as i32 - keys.pressed(KeyCode::Q) as i32) as f32 * speed;
    transform.translation.z +=
        (keys.pressed(KeyCode::S) as i32 - keys.pressed(KeyCode::W) as i32) as f32 * speed;
}

fn create_colliders(chunk: &Chunk) -> Vec<(Isometry<Real>, SharedShape)>
{
    let mut colliders: Vec<(Isometry<Real>, SharedShape)> = Vec::new();

    // let mut b_width = 0;
    // let mut b_height = 0;
    // let mut b_length = 0;

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
                    // make collider
                    let length = 1 + count / (LENGTH * HEIGHT);
                    let height = 1 + (count - length * count) / HEIGHT;
                    let width = 1 + count - count * height - count * length;

                    let hw = width as f32 / 2.0;
                    let hh = height as f32 / 2.0;
                    let hl = length as f32 / 2.0;

                    colliders.push(
                        (Isometry::translation(start_x + hw, start_y + hh, start_z + hl),
                         ColliderShape::cuboid(hw, hh, hl)));

                    count = 0;
                }
            }
        }
    }

    if count != 0
    {
        // make collider
        let length = 1 + count / (LENGTH * HEIGHT);
        let height = 1 + (count - length * count) / HEIGHT;
        let width = 1 + count - count * height - count * length;

        let hw = width as f32 / 2.0;
        let hh = height as f32 / 2.0;
        let hl = length as f32 / 2.0;

        colliders.push(
            (Isometry::translation(start_x + hw, start_y + hh, start_z + hl),
             ColliderShape::cuboid(hw, hh, hl)));

        count = 0;
    }

        // let mut z = 0;
    // let mut y = 0;
    // let mut x = 0;
    //
    // while z < LENGTH
    // {
    //     while y < HEIGHT
    //     {
    //         while x < WIDTH
    //         {
    //             if chunk.has_block_u16(x, y, z)
    //             {
    //                 let xx = x as f32 - WIDTH as f32 / 2.0;
    //                 let yy = y as f32 - HEIGHT as f32 / 2.0;
    //                 let zz = z as f32 - LENGTH as f32 / 2.0;
    //
    //                 // colliders.push(
    //                 //     (Isometry::translation(xx, yy, zz),
    //                 //      ColliderShape::cuboid(0.5, 0.5, 0.5)));
    //
    //                 if b_width == 0
    //                 {
    //                     start_x = xx;
    //                     start_y = yy;
    //                     start_z = zz;
    //
    //                     b_width = 1;
    //                     b_height = 1;
    //                     b_length = 1;
    //                 }
    //             }
    //             else if b_width != 0
    //             {
    //                 if b_height != 1
    //                 {
    //                     x = -1;
    //                     y -= 1;
    //
    //                     let hw = b_width as f32 / 2.0;
    //                     let hh = b_height as f32 / 2.0;
    //                     let hl = b_length as f32 / 2.0;
    //
    //                     colliders.push(
    //                         (Isometry::translation(start_x + hw, start_h + hh, start_z + hl),
    //                          ColliderShape::cuboid(hw, hh, hl)));
    //
    //                     b_width = 0;
    //                     b_height = 0;
    //                     b_length = 0;
    //                 }
    //                 else
    //                 {
    //                     let hw = b_width as f32 / 2.0;
    //
    //                     colliders.push(
    //                         (Isometry::translation(start_x + hw, start_h + 0.5, start_z + 0.5),
    //                          ColliderShape::cuboid(hw, 0.5, 0.5)));
    //
    //                     b_width = 0;
    //                     b_height = 0;
    //                     b_length = 0;
    //                 }
    //             }
    //
    //             x += 1;
    //         }
    //
    //         y += 1;
    //     }
    //
    //     z += 1;
    // }

    colliders
}

/// sets up a scene with textured entities
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    let texture_handle = asset_server.load("block.png");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..Default::default()
    });

    for z in -4..0
    {
        for x in 0..4
        {
            let chunk = Chunk::new(x * 16, 0, z * 16);

            let xf: f32 = x as f32 * 16.0;
            let yf: f32 = 0.0;
            let zf: f32 = z as f32 * 16.0;

            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(chunk.create_mesh()),
                    material: material_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(xf, yf, zf),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(chunk)
                .insert(NeedsGenerated {})
                .insert_bundle(RigidBodyBundle {
                    position: [xf, yf, zf].into(),
                    velocity: RigidBodyVelocity {
                        linvel: [rand::random::<f32>() * 6.0 - 3.0,
                            rand::random::<f32>() * 6.0 - 3.0,
                            rand::random::<f32>() * 6.0 - 3.0].into(),
                        angvel: [0.0, 0.0, 0.0].into()
                    }.into(),
                    ..Default::default()
                })
                .insert(ColliderPositionSync::Discrete); // Updates Bevy's transform w/ rapier's transform
        }
    }

    // commands.spawn_bundle(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
    //     ..Default::default()
    // });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 10240.0,
            shadows_enabled: true,
            color: Color::Rgba {
                red: 1.0,
                blue: 1.0,
                green: 1.0,
                alpha: 1.0,
            },
            ..Default::default()
        },
        transform: Transform::from_xyz(8.0, 20.0, 8.0),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(3.0, 5.0, 8.0), //.looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
