use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::orbiting::OrbitingCameraState;
use crate::input::Move;

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
    mut velocities: Query<&mut Velocity>,
    mut movements: EventReader<Move>,

    player: Query<Entity, With<Player>>,
    orbiting: Query<&OrbitingCameraState, With<Camera>>,
) {
    if movements.is_empty() { return }

    let input = movements.iter().next().unwrap();

    let orbiting = orbiting.iter().next().unwrap();

    let player_id = player.iter().next().unwrap();
    let mut transform = transforms.get_mut(player_id).unwrap();
    let mut velocities = velocities.get_mut(player_id).unwrap();

    if input.direction != Vec3::ZERO {
        transform.look_to(input.direction, Vec3::Z);
        transform.rotation *= Quat::from_rotation_y(-orbiting.longitude + std::f32::consts::PI);
    } 
    velocities.linvel = transform.forward() * input.direction.length_squared() * input.speed;    
}
