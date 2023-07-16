mod input;
mod orbiting;
mod level;
mod player;

use bevy::{
    prelude::*,
    input::mouse::MouseMotion,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};
use bevy_rapier3d::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_replicon::prelude::*;

use crate::input::UserInputPlugin;
use crate::orbiting::{OrbitingCameraPlugin, OrbitingCameraState};
use crate::level::{Level, draw_level};
use crate::player::{Player, move_player};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(OrbitingCameraPlugin)
        .add_plugin(UserInputPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(ReplicationPlugins)

        .insert_resource(RapierConfiguration {
            gravity: -Vec3::Z * 9.81,
            ..Default::default()
        })

        .add_startup_system(world_startup)
        .add_startup_system(spawn_player)

        .add_system(draw_level)
        .add_system(move_player)
        .add_system(rotate_camera_using_mouse)
        
        .run();
}

fn world_startup(mut commands: Commands) {
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
                    vec![2, 3, 4, 5, 1, 1, 1, 1, 1, 1],
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
}

fn spawn_player(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    let initial_player_position = Vec3::new(2.0, 2.0, 2.4);

    // TODO: How to create player bundle so that entity would still have SceneBundle transform?
    // TODO: move out to player module
    let id = commands.spawn((
        Name::new("Player"),
        Player::default(),
        TransformBundle::from_transform(
            Transform::default()
                .with_translation(initial_player_position)
                .with_scale(Vec3::new(0.3, 0.3, 0.3))
                .with_rotation(Quat::from_rotation_arc(Vec3::Y, Vec3::Z)),
        ),
        asset_server.load("cylinder2.glb#Scene0") as Handle<Scene>,
        Visibility::default(),
        ComputedVisibility::default(),
        Collider::cylinder(1.2, 1.2),
        Velocity::default(),
        // RigidBody::KinematicVelocityBased,
        RigidBody::Dynamic,
        KinematicCharacterController {
            apply_impulse_to_dynamic_bodies: true,
            up: Vec3::Z,
            // snap_to_ground: Some(CharacterLength::Relative(0.2)),
            // custom_shape: Some((Collider::cylinder(0.5, 0.5), Vec3::ZERO, Quat::IDENTITY)),
            ..default()
        }
    )).id();
    
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

    commands.spawn((
        Name::new("CameraBundle"),
        Camera3dBundle {
            transform: Transform::default()
                .with_translation(Vec3::new(10.0, 10.0, 10.0)),
            ..Default::default()
        },
        OrbitingCameraState::default()
            .with_target(id),
    ));
}


fn rotate_camera_using_mouse(
    mut motion_evr: EventReader<MouseMotion>,
    mut q1: Query<&mut OrbitingCameraState, With<Camera3d>>,
    time: Res<Time>,
) {
    for ev in motion_evr.iter() {        
        for mut camera in q1.iter_mut() {
            if ev.delta.length() < 0.1 {
                continue;
            }

            camera.longitude += ev.delta.x * time.delta_seconds() * 0.1;
            camera.latitude -= ev.delta.y * time.delta_seconds() * 0.1;
        }
    }
}

