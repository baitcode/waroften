use bevy::{
    prelude::*,
};

use crate::orbiting::OrbitingCameraState;
use crate::input::UserInput;

#[derive(Component, Default)]
pub struct Player();


pub fn move_player(
    mut transforms: Query<&mut Transform>,
    player: Query<Entity, With<Player>>,
    orbiting: Query<&OrbitingCameraState, With<Camera>>,
    input: Res<UserInput>,
    time: Res<Time>,
) {
    let orbiting = orbiting.iter().next().unwrap();

    let player_id = player.iter().next().unwrap();
    let mut player_transform = transforms.get_mut(player_id).unwrap();
    
    if input.direction.length_squared() < 0.1 { return; }

    player_transform.look_to(input.direction, Vec3::Z);
    player_transform.rotation *= Quat::from_rotation_y(-orbiting.longitude + std::f32::consts::PI);
    player_transform.translation = player_transform.translation + player_transform.forward() * time.delta_seconds();
}