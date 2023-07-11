use bevy::{
    prelude::*,
};
use bevy_rapier3d::prelude::*;

use crate::orbiting::OrbitingCameraState;
use crate::input::UserInput;

#[derive(Component)]
pub struct Player {
    pub health: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            health: 100,
        }
    }
}


pub fn move_player(
    mut transforms: Query<&mut Transform>,
    mut characters: Query<&mut KinematicCharacterController>,
    mut velocities: Query<&mut Velocity>,
    player: Query<Entity, With<Player>>,
    orbiting: Query<&OrbitingCameraState, With<Camera>>,
    input: Res<UserInput>,
    time: Res<Time>,
) {

    let orbiting = orbiting.iter().next().unwrap();

    let player_id = player.iter().next().unwrap();
    let mut transform = transforms.get_mut(player_id).unwrap();
    let mut character = characters.get_mut(player_id).unwrap();
    let mut velocities = velocities.get_mut(player_id).unwrap();

    character.up = Vec3::Y;
    
    if input.direction.length_squared() < 0.1 {
        velocities.linvel = Vec3::ZERO;
        return 
    }

    transform.look_to(input.direction, Vec3::Z);
    transform.rotation *= Quat::from_rotation_y(-orbiting.longitude + std::f32::consts::PI);
    
    velocities.linvel = transform.forward() * input.direction.length_squared() * 3.0;
    
    // let speed = transform.forward() * input.direction.length_squared();
    // transform.translation = transform.translation + speed * time.delta_seconds();
}

// pub fn move_player_velocity_based(
//     mut transforms: Query<&mut Transform>,
//     mut velocities: Query<&mut Velocity>,
//     player: Query<Entity, With<Player>>,
//     orbiting: Query<&OrbitingCameraState, With<Camera>>,
//     input: Res<UserInput>,
//     time: Res<Time>,
// ) {
//     let orbiting = orbiting.iter().next().unwrap();

//     let player_id = player.iter().next().unwrap();
//     let mut player_transform = transforms.get_mut(player_id).unwrap();
//     let mut player_velocity = velocities.get_mut(player_id).unwrap();
    
//     if input.direction.length_squared() < 0.1 { 
//         player_velocity.linvel = Vec3::ZERO;
//     } else {
//         player_transform.look_to(input.direction, Vec3::Z);
//         player_transform.rotation *= Quat::from_rotation_y(-orbiting.longitude + std::f32::consts::PI);
//         player_velocity.linvel = player_transform.forward() * input.direction.length_squared() * 3.0;
//     }
// }