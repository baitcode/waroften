mod input;
mod orbiting;
mod level;
mod player;
mod menu;

use bevy::{
    prelude::*,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use bevy_rapier3d::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_replicon::prelude::*;

use belly::prelude::*;

use crate::input::UserInputPlugin;
use crate::orbiting::{OrbitingCameraPlugin, rotate_camera_using_mouse, spawn_camera};
use crate::level::{Level, draw_level};
use crate::player::{PlayerPlugin, Player};
use crate::menu::draw_menu;

// .add_plugin(WorldInspectorPlugin::new())
// .add_plugin(OrbitingCameraPlugin)
// .add_plugin(UserInputPlugin)
// .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
// .add_plugin(RapierDebugRenderPlugin::default())
// .add_plugin(LogDiagnosticsPlugin::default())
// .add_plugin(FrameTimeDiagnosticsPlugin::default())
// .add_plugin(PlayerPlugin)

// .insert_resource(RapierConfiguration {
//     gravity: -Vec3::Z * 9.81,
//     ..Default::default()
// })

// .add_startup_system(world_startup)

// .add_system(spawn_camera)

// .add_system(draw_level)
// .add_system(rotate_camera_using_mouse)

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(BellyPlugin)

        .add_startup_system(draw_menu)

        .run();
}

#[derive(States, Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub enum ClientState {
    #[default]
    MainMenu,
    InGame,
    Loading
}

fn world_startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player::default(),
        // Target::default(), moved into player? good or bad?
        Transform::default()
            .with_translation(Vec3::new(2.0, 2.0, 2.4))
            .with_scale(Vec3::new(0.3, 0.3, 0.3))
            .with_rotation(Quat::from_rotation_arc(Vec3::Y, Vec3::Z)),
    )); 
    
    
    commands.spawn((
        Name::new("DirectionalLight"),
        DirectionalLightBundle {
            directional_light: DirectionalLight::default(),
            transform: Transform::default()
                .with_translation(Vec3::new(0.0, 0.0, 10.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
    ));

    commands.spawn(( // TODO(baitcode): Level bundle
        Name::new("Level"),
        ComputedVisibility::default(),
        Visibility::default(),
        Transform::default(),
        GlobalTransform::default(),
        Level::default()
            .add_layer(
                vec![
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                ]
            )
            .add_layer(
                vec![
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                ]
            )
            .add_layer(
                vec![
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                ]
            ),
    ));


    commands.spawn((
        Name::new("DynamicObject"),
        PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: materials.add(Color::RED.into()),
            transform: Transform::default()
                .with_translation(Vec3::new(4.0, 3.0, 4.0))
                .with_scale(Vec3::new(0.3, 0.3, 0.3)),
            ..default()
        },
        RigidBody::Dynamic,
    )).with_children(|children| {
        children
            .spawn(Collider::ball(0.34))
            .insert(Transform::default().with_translation(Vec3::new(0.0, 0.0, 0.0)));
    });

}
