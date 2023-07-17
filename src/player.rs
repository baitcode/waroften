use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_replicon::prelude::*;

use crate::input::Move;
use crate::orbiting::{OrbitingCameraState, Target};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(move_player)
            .add_system(init_player)
            .replicate::<Player>();
    }
}


#[derive(Component,Reflect)]
#[reflect(Component)]
pub struct Player {
    pub managed: bool,
    pub health: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            managed: true,
            health: 100,
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    #[bundle]
    transform: TransformBundle,
    scene: Handle<Scene>,
    visibility: Visibility,
    computed_visibility: ComputedVisibility,
    collider: Collider,
    velocity: Velocity,
    rigid_body: RigidBody,
    kinematic_character_controller: KinematicCharacterController,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Player"),
            transform: TransformBundle::default(),
            scene: Handle::default() as Handle<Scene>,
            visibility: Visibility::default(),
            computed_visibility: ComputedVisibility::default(),
            collider: Collider::cylinder(1.2, 1.2),
            velocity: Velocity::default(),
            rigid_body: RigidBody::Dynamic,
            kinematic_character_controller: KinematicCharacterController {
                apply_impulse_to_dynamic_bodies: true,
                up: Vec3::Z,
                ..default()
            },
        }
    }
}

impl PlayerBundle {
    fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = TransformBundle::from_transform(transform);
        self
    }

    fn with_scene(mut self, scene: Handle<Scene>) -> Self {
        self.scene = scene;
        self
    }

    fn with_name(mut self, name: Name) -> Self {
        self.name = name;
        self
    }
}

pub fn move_player(
    mut transforms: Query<&mut Transform>,
    mut velocities: Query<&mut Velocity>,
    mut movements: EventReader<Move>,
    
    players: Query<&mut Player>,
    player_entity: Query<Entity, With<Player>>,
    orbiting: Query<&OrbitingCameraState, With<Camera>>,
) {
    let player_id = player_entity.iter().next().unwrap();
    let player = players.get(player_id).unwrap();

    if !player.managed { return }

    if movements.is_empty() { return }
    if orbiting.is_empty() { return }
    
    let movement = movements.iter().next().unwrap();
    let orbiting = orbiting.iter().next().unwrap();
    let mut transform = transforms.get_mut(player_id).unwrap();
    let mut velocities = velocities.get_mut(player_id).unwrap();

    if movement.direction != Vec3::ZERO {
        transform.look_to(movement.direction, Vec3::Z);
        transform.rotation *= Quat::from_rotation_y(-orbiting.longitude + std::f32::consts::PI);
    } 
    
    velocities.linvel = transform.forward() * movement.direction.length_squared() * movement.speed;    
}

pub fn init_player(
    mut commands: Commands,
    players: Query<&Player>,
    transforms: Query<&Transform>,
    added_player_entities: Query<Entity, Added<Player>>,
    asset_server: Res<AssetServer>,
) {
    for player_id in added_player_entities.iter() {
        let player = players.get(player_id).unwrap();
        let player_transform = transforms.get(player_id).unwrap();

        if player.managed {
            // Camera tracking
            commands.entity(player_id).insert((
                Target::default(),
            ));
        } 

        commands.entity(player_id).insert(
            PlayerBundle::default()
                .with_name(Name::new("Player"))
                .with_transform(player_transform.clone())
                .with_scene(asset_server.load("cylinder2.glb#Scene0") as Handle<Scene>)
        );            
    }
}