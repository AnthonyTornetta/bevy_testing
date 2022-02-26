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
use crate::chunk::{Chunk, NeedsGenerated};
use crate::generation::chunk_generator;
use bevy::ecs::component::ComponentInfo;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use std::ops::Mul;

fn main()
{
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(camera_movement_system)
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

    for z in -16..0
    {
        for x in 0..16
        {
            let chunk = Chunk::new(x * 16, 0, z * 16);
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(chunk.create_mesh()),
                    material: material_handle.clone(),
                    transform: Transform {
                        translation: Vec3::new(x as f32 * 16.0, 0.0, z as f32 * 16.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(chunk)
                .insert(NeedsGenerated {});
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
