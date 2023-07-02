use bevy::{
    prelude::*, 
    ecs::system::EntityCommands
};


pub struct CoordinateSystemPlugin;

impl Plugin for CoordinateSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(draw_coordinate_system);
    }
}

#[derive(Component)]
pub struct Coordinates {
    pub axis_length: f32,
    pub axis_radius: f32,
}

impl Default for Coordinates {
    fn default() -> Self {
        Self {
            axis_length: 1.0,
            axis_radius: 0.01,
        }
    }
}

// add struct for coordinate system
// and draw as many system as marked with structure. Object has to have transform

fn draw_coordinate_system(
    mut commands: Commands, 
    mut materials: ResMut<Assets<StandardMaterial>>, 
    mut meshes: ResMut<Assets<Mesh>>,
    centers: Query<(Entity, &Coordinates), Changed<Coordinates>>,
) {
    for (entity, coordinates) in centers.iter() {
        println!("Drawing coordinate system in entity");

        let axis_length = coordinates.axis_length;
        let axis_radius = coordinates.axis_radius;
    
        let cylinder_mesh = meshes.add(Mesh::from(shape::Cylinder {
            radius: axis_radius,
            height: axis_length,
            ..default()
        }));
    
        commands.entity(entity)
            .insert(
                PbrBundle {
                    mesh: cylinder_mesh.clone(),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(Vec3::new(axis_length / 2.0, 0.0, 0.0))
                        .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                    ..default()
                },
            )
            .insert(Name::new("AXIS_X"));

        commands.entity(entity)
            .insert(
                PbrBundle {
                    mesh: cylinder_mesh.clone(),
                    material: materials.add(Color::GREEN.into()),
                    transform: Transform::from_translation(Vec3::new(0.0, axis_length / 2.0, 0.0)),
                    ..default()
                },
            )
            .insert(Name::new("AXIS_Y"));

        commands.entity(entity)
            .insert(
                PbrBundle {
                    mesh: cylinder_mesh.clone(),
                    material: materials.add(Color::BLUE.into()),
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, axis_length / 2.0))
                        .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    ..default()
                },
            )
            .insert(Name::new("AXIS_Z"));

    }

}